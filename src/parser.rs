// Manual parsing implementation to replace nom

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedEnd,
    UnexpectedChar(char),
    InvalidFormat(String),
    ParseError(String),
}

impl From<ParseError> for crate::error::Error {
    fn from(e: ParseError) -> Self {
        match e {
            ParseError::ParseError(msg) => Self::Parse(msg),
            ParseError::InvalidFormat(msg) => Self::Parse(msg),
            ParseError::UnexpectedEnd => Self::Parse("Unexpected end of input".to_string()),
            ParseError::UnexpectedChar(ch) => Self::Parse(format!("Unexpected character: {}", ch)),
        }
    }
}

type PResult<'a, Output> = Result<(Output, &'a str), ParseError>;

#[derive(Debug, Clone)]
pub enum QueryAtom<'a> {
    Last { count: u16, major: bool, name: Option<&'a str> },
    Unreleased(Option<&'a str>),
    Years(f64),
    Since { year: i32, month: u32, day: u32 },
    Percentage { comparator: Comparator, popularity: f32, stats: Stats<'a> },
    Cover { coverage: f32, stats: Stats<'a> },
    Supports(&'a str, Option<SupportKind>),
    Electron(VersionRange<'a>),
    Node(VersionRange<'a>),
    Browser(&'a str, VersionRange<'a>),
    FirefoxESR,
    OperaMini,
    CurrentNode,
    MaintainedNode,
    Phantom(bool),
    BrowserslistConfig,
    Defaults,
    Dead,
    Extends(&'a str),
    Unknown(&'a str),
}

#[derive(Debug, Clone)]
pub enum Stats<'a> {
    Global,
    Region(&'a str),
}

#[derive(Debug, Clone)]
pub enum SupportKind {
    Fully,
    Partially,
}

#[derive(Debug, Clone)]
pub enum Comparator {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

#[derive(Debug, Clone)]
pub enum VersionRange<'a> {
    Bounded(&'a str, &'a str),
    Unbounded(Comparator, &'a str),
    Accurate(&'a str),
}

#[derive(Debug)]
pub struct SingleQuery<'a> {
    pub(crate) raw: &'a str,
    pub(crate) atom: QueryAtom<'a>,
    pub(crate) negated: bool,
    pub(crate) is_and: bool,
}

// For now, just handle the empty case and return an error for non-empty input
// This allows the code to compile while we work on the complete implementation
pub fn parse_browserslist_query(input: &str) -> PResult<Vec<SingleQuery>> {
    let input = input.trim();
    if input.is_empty() {
        return Ok((vec![], ""));
    }
    
    // Start implementing the manual parser
    let mut queries = Vec::new();
    let mut remaining = input;
    
    // Parse the first query (might be negated)
    let (first_query, rest) = parse_single_query_internal(remaining, true)?;
    queries.push(first_query);
    remaining = rest;
    
    // Parse additional queries separated by commas or conjunctions
    while !remaining.trim().is_empty() {
        remaining = remaining.trim_start();
        
        // Check for separator (comma, "and", "or")
        if remaining.starts_with(',') {
            remaining = &remaining[1..].trim_start();
            let (query, rest) = parse_single_query_internal(remaining, false)?;
            queries.push(query);
            remaining = rest;
        } else if remaining.starts_with(" and ") {
            remaining = &remaining[5..].trim_start();
            let (mut query, rest) = parse_single_query_internal(remaining, false)?;
            query.is_and = true;
            queries.push(query);
            remaining = rest;
        } else if remaining.starts_with(" or ") {
            remaining = &remaining[4..].trim_start();
            let (query, rest) = parse_single_query_internal(remaining, false)?;
            queries.push(query);
            remaining = rest;
        } else {
            // Try to parse as a space-separated continuation
            let (query, rest) = parse_single_query_internal(remaining, false)?;
            queries.push(query);
            remaining = rest;
        }
    }
    
    Ok((queries, ""))
}

fn parse_single_query_internal(input: &str, allow_leading_not: bool) -> PResult<SingleQuery> {
    let original_input = input;
    let mut remaining = input.trim_start();
    let mut negated = false;
    
    // Check for "not" prefix
    if allow_leading_not && remaining.starts_with("not ") {
        negated = true;
        remaining = &remaining[4..].trim_start();
    }
    
    // Parse the actual query atom
    let (atom, rest) = parse_query_atom_internal(remaining)?;
    
    // Calculate the raw part
    let consumed_len = original_input.len() - rest.len();
    let raw = &original_input[..consumed_len];
    
    Ok((SingleQuery {
        raw,
        atom,
        negated,
        is_and: false,
    }, rest))
}

fn parse_query_atom_internal(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();
    
    // Try to parse different atom types
    
    // "defaults"
    if input.starts_with("defaults") {
        return Ok((QueryAtom::Defaults, &input[8..]));
    }
    
    // "dead"
    if input.starts_with("dead") {
        return Ok((QueryAtom::Dead, &input[4..]));
    }
    
    // "firefox esr" or "FirefoxESR"
    if input.to_lowercase().starts_with("firefox esr") || input.to_lowercase().starts_with("firefoxesr") {
        let len = if input.to_lowercase().starts_with("firefox esr") { 11 } else { 10 };
        return Ok((QueryAtom::FirefoxESR, &input[len..]));
    }
    
    // "current node"
    if input.to_lowercase().starts_with("current node") {
        return Ok((QueryAtom::CurrentNode, &input[12..]));
    }
    
    // "maintained node"
    if input.to_lowercase().starts_with("maintained node") {
        return Ok((QueryAtom::MaintainedNode, &input[15..]));
    }
    
    // "unreleased versions"
    if input.to_lowercase().starts_with("unreleased versions") {
        return Ok((QueryAtom::Unreleased(None), &input[19..]));
    }
    
    // "last N versions"
    if input.to_lowercase().starts_with("last ") {
        return parse_last_versions(&input[5..]);
    }
    
    // "> N%", ">= N%", "< N%", "<= N%"
    if let Some(rest) = try_parse_percentage(input)? {
        return Ok(rest);
    }
    
    // For now, treat anything else as unknown
    // In a real implementation, we'd need to handle many more cases
    let end_pos = input.find(|c: char| c.is_whitespace() || c == ',' || c == ')' || c == ']')
        .unwrap_or(input.len());
    
    Ok((QueryAtom::Unknown(&input[..end_pos]), &input[end_pos..]))
}

fn parse_last_versions(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();
    
    // Parse number
    let mut i = 0;
    while i < input.len() && input.chars().nth(i).unwrap().is_ascii_digit() {
        i += 1;
    }
    
    if i == 0 {
        return Err(ParseError::InvalidFormat("Expected number after 'last'".to_string()));
    }
    
    let count_str = &input[..i];
    let count: u16 = count_str.parse().map_err(|_| ParseError::InvalidFormat("Invalid number".to_string()))?;
    let remaining = &input[i..].trim_start();
    
    // Check for "version" or "versions"
    if remaining.starts_with("version") {
        let len = if remaining.starts_with("versions") { 8 } else { 7 };
        return Ok((QueryAtom::Last { count, major: false, name: None }, &remaining[len..]));
    }
    
    // Check for "major version" or "major versions"
    if remaining.starts_with("major version") {
        let len = if remaining.starts_with("major versions") { 14 } else { 13 };
        return Ok((QueryAtom::Last { count, major: true, name: None }, &remaining[len..]));
    }
    
    // Check for browser name + versions (e.g., "2 Chrome versions")
    let words: Vec<&str> = remaining.split_whitespace().collect();
    if words.len() >= 2 && (words[words.len()-1] == "versions" || words[words.len()-1] == "version") {
        let browser_name = words[..words.len()-1].join(" ");
        let consumed_len = remaining.len() - remaining.split_whitespace().last().unwrap_or("").len();
        return Ok((QueryAtom::Last { count, major: false, name: Some(&input[i+consumed_len-browser_name.len()..i+consumed_len]) }, &remaining[consumed_len..]));
    }
    
    Err(ParseError::InvalidFormat("Expected 'version(s)' after number".to_string()))
}

fn try_parse_percentage(input: &str) -> Result<Option<(QueryAtom, &str)>, ParseError> {
    let input = input.trim_start();
    
    // Check for comparator symbols
    let (comparator, offset) = if input.starts_with(">=") {
        (Comparator::GreaterOrEqual, 2)
    } else if input.starts_with("<=") {
        (Comparator::LessOrEqual, 2)
    } else if input.starts_with(">") {
        (Comparator::Greater, 1)
    } else if input.starts_with("<") {
        (Comparator::Less, 1)
    } else {
        return Ok(None);
    };
    
    let remaining = &input[offset..].trim_start();
    
    // Parse number
    let mut i = 0;
    let mut has_dot = false;
    while i < remaining.len() {
        let ch = remaining.chars().nth(i).unwrap();
        if ch.is_ascii_digit() {
            i += 1;
        } else if ch == '.' && !has_dot {
            has_dot = true;
            i += 1;
        } else {
            break;
        }
    }
    
    if i == 0 {
        return Err(ParseError::InvalidFormat("Expected number after comparator".to_string()));
    }
    
    let after_number = &remaining[i..];
    if !after_number.starts_with('%') {
        return Ok(None);
    }
    
    let number_str = &remaining[..i];
    let popularity: f32 = number_str.parse().map_err(|_| ParseError::InvalidFormat("Invalid percentage number".to_string()))?;
    
    Ok(Some((QueryAtom::Percentage { comparator, popularity, stats: Stats::Global }, &after_number[1..])))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::{opts::Opts, test::run_compare};

    #[test_case(""; "empty")]
    #[test_case("ie >= 6, ie <= 7"; "comma")]
    #[test_case("ie >= 6 and ie <= 7"; "and")]
    #[test_case("ie < 11 and not ie 7"; "and with not")]
    #[test_case("last 1 Baidu version and not <2%"; "with not and one-version browsers as and query")]
    #[test_case("ie >= 6 or ie <= 7"; "or")]
    #[test_case("ie < 11 or not ie 7"; "or with not")]
    #[test_case("last 2 versions and > 1%"; "swc issue 4871")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}