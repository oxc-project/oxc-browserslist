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
    remaining = rest.trim_start();

    // Parse additional queries separated by commas or conjunctions
    while !remaining.is_empty() {
        // Check for separator (comma, "and", "or")
        if remaining.starts_with(',') {
            remaining = &remaining[1..].trim_start();
            let (query, rest) = parse_single_query_internal(remaining, false)?;
            queries.push(query);
            remaining = rest.trim_start();
        } else if remaining.starts_with(" and ") || remaining.starts_with(" AND ") {
            let skip_len = if remaining[1..4].to_lowercase() == "and" { 5 } else { 5 };
            remaining = &remaining[skip_len..].trim_start();
            let (mut query, rest) = parse_single_query_internal(remaining, true)?; // Allow not after and
            query.is_and = true;
            queries.push(query);
            remaining = rest.trim_start();
        } else if remaining.starts_with(" or ") || remaining.starts_with(" OR ") {
            let skip_len = if remaining[1..3].to_lowercase() == "or" { 4 } else { 4 };
            remaining = &remaining[skip_len..].trim_start();
            let (query, rest) = parse_single_query_internal(remaining, true)?; // Allow not after or  
            queries.push(query);
            remaining = rest.trim_start();
        } else if remaining.trim_start().starts_with("and ")
            || remaining.trim_start().starts_with("AND ")
        {
            // Handle case where " and " doesn't start with space (fallback)
            let trimmed = remaining.trim_start();
            let skip_len = if trimmed[..3].to_lowercase() == "and" { 4 } else { 4 };
            remaining = &trimmed[skip_len..].trim_start();
            let (mut query, rest) = parse_single_query_internal(remaining, true)?;
            query.is_and = true;
            queries.push(query);
            remaining = rest.trim_start();
        } else if remaining.trim_start().starts_with("or ")
            || remaining.trim_start().starts_with("OR ")
        {
            // Handle case where " or " doesn't start with space (fallback)
            let trimmed = remaining.trim_start();
            let skip_len = if trimmed[..2].to_lowercase() == "or" { 3 } else { 3 };
            remaining = &trimmed[skip_len..].trim_start();
            let (query, rest) = parse_single_query_internal(remaining, true)?;
            queries.push(query);
            remaining = rest.trim_start();
        } else {
            // If we can't find a separator, we're done or there's an error
            break;
        }
    }

    Ok((queries, ""))
}

fn parse_single_query_internal(input: &str, allow_leading_not: bool) -> PResult<SingleQuery> {
    let original_input = input;
    let mut remaining = input.trim_start();
    let mut negated = false;

    // Check for "not" prefix
    if allow_leading_not {
        if remaining.to_lowercase().starts_with("not ") {
            negated = true;
            remaining = &remaining[4..].trim_start();
        }
    } else {
        // For non-leading queries, "not" might be found later in the query chain
        if remaining.to_lowercase().starts_with("not ") {
            negated = true;
            remaining = &remaining[4..].trim_start();
        }
    }

    // Parse the actual query atom
    let (atom, rest) = parse_query_atom_internal(remaining)?;

    // Calculate the raw part
    let consumed_len = original_input.len() - rest.len();
    let raw = &original_input[..consumed_len];

    Ok((SingleQuery { raw, atom, negated, is_and: false }, rest))
}

fn parse_query_atom_internal(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();

    // Try to parse different atom types

    // "defaults"
    if input.to_lowercase().starts_with("defaults") {
        return Ok((QueryAtom::Defaults, &input[8..]));
    }

    // "dead"
    if input.to_lowercase().starts_with("dead") {
        return Ok((QueryAtom::Dead, &input[4..]));
    }

    // "firefox esr" or "FirefoxESR"
    if input.to_lowercase().starts_with("firefox esr") {
        return Ok((QueryAtom::FirefoxESR, &input[11..]));
    }
    if input.to_lowercase().starts_with("firefoxesr") {
        return Ok((QueryAtom::FirefoxESR, &input[10..]));
    }

    // "current node"
    if input.to_lowercase().starts_with("current node") {
        return Ok((QueryAtom::CurrentNode, &input[12..]));
    }

    // "maintained node"
    if input.to_lowercase().starts_with("maintained node") {
        return Ok((QueryAtom::MaintainedNode, &input[15..]));
    }

    // "opera mini" or "op_mini_all"
    if input.to_lowercase().starts_with("opera mini")
        || input.to_lowercase().starts_with("op_mini_all")
    {
        let len = if input.to_lowercase().starts_with("opera mini") { 10 } else { 11 };
        return Ok((QueryAtom::OperaMini, &input[len..]));
    }

    // "unreleased versions"
    if input.to_lowercase().starts_with("unreleased versions") {
        return Ok((QueryAtom::Unreleased(None), &input[19..]));
    }

    // "unreleased X versions" where X is a browser name
    if input.to_lowercase().starts_with("unreleased ") {
        let rest = &input[11..];
        if let Some(pos) = rest.to_lowercase().find(" versions") {
            let browser_name = &rest[..pos];
            return Ok((QueryAtom::Unreleased(Some(browser_name)), &rest[pos + 9..]));
        }
    }

    // "last N versions"
    if input.to_lowercase().starts_with("last ") {
        return parse_last_versions(&input[5..]);
    }

    // "N years"
    if let Some((years, remaining)) = try_parse_years(input)? {
        return Ok((QueryAtom::Years(years), remaining));
    }

    // "since YYYY-MM-DD" or "since YYYY-MM" or "since YYYY"
    if input.to_lowercase().starts_with("since ") {
        return parse_since(&input[6..]);
    }

    // "> N%", ">= N%", "< N%", "<= N%"
    if let Some(rest) = try_parse_percentage(input)? {
        return Ok(rest);
    }

    // "cover N%"
    if input.to_lowercase().starts_with("cover ") {
        return parse_cover(&input[6..]);
    }

    // "supports feature-name"
    if input.to_lowercase().starts_with("supports ") {
        return parse_supports(&input[9..]);
    }

    // "phantom X"
    if input.to_lowercase().starts_with("phantom ") {
        return parse_phantom(&input[8..]);
    }

    // "extends config"
    if input.to_lowercase().starts_with("extends ") {
        let config_name = &input[8..].trim_start();
        let end_pos = config_name.find(|c: char| c.is_whitespace()).unwrap_or(config_name.len());
        return Ok((QueryAtom::Extends(&config_name[..end_pos]), &config_name[end_pos..]));
    }

    // ".browserslistrc"
    if input.starts_with(".browserslistrc") {
        return Ok((QueryAtom::BrowserslistConfig, &input[15..]));
    }

    // Browser version patterns like "chrome >= 50", "ie 11", "node 12.0.0"
    if let Some(rest) = try_parse_browser_version(input)? {
        return Ok(rest);
    }

    // "electron X" or "electron >= X"
    if input.to_lowercase().starts_with("electron ") {
        return parse_electron(&input[9..]);
    }

    // "node X" or "node >= X"
    if input.to_lowercase().starts_with("node ") {
        return parse_node(&input[5..]);
    }

    // For now, treat anything else as unknown
    // In a real implementation, we'd need to handle many more cases
    let end_pos = input
        .find(|c: char| c.is_whitespace() || c == ',' || c == ')' || c == ']')
        .unwrap_or(input.len());

    Ok((QueryAtom::Unknown(&input[..end_pos]), &input[end_pos..]))
}

fn try_parse_years(input: &str) -> Result<Option<(f64, &str)>, ParseError> {
    let input = input.trim_start();

    // Look for a number followed by "year" or "years"
    let mut i = 0;
    let mut has_dot = false;

    while i < input.len() {
        let ch = input.chars().nth(i).unwrap();
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
        return Ok(None);
    }

    let remaining = &input[i..].trim_start();
    if remaining.starts_with("year") {
        let number_str = &input[..i];
        let years: f64 = number_str
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid years number".to_string()))?;
        let len = if remaining.starts_with("years") { 5 } else { 4 };
        return Ok(Some((years, &remaining[len..])));
    }

    Ok(None)
}

fn parse_since(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();

    // Parse YYYY-MM-DD, YYYY-MM, or YYYY
    let parts: Vec<&str> = input.split('-').take(3).collect();

    if parts.is_empty() {
        return Err(ParseError::InvalidFormat("Expected date after 'since'".to_string()));
    }

    let year: i32 =
        parts[0].parse().map_err(|_| ParseError::InvalidFormat("Invalid year".to_string()))?;
    let month = if parts.len() > 1 {
        parts[1].parse().map_err(|_| ParseError::InvalidFormat("Invalid month".to_string()))?
    } else {
        1
    };
    let day = if parts.len() > 2 {
        parts[2].parse().map_err(|_| ParseError::InvalidFormat("Invalid day".to_string()))?
    } else {
        1
    };

    let consumed_len = parts.iter().map(|p| p.len()).sum::<usize>() + parts.len() - 1;

    Ok((QueryAtom::Since { year, month, day }, &input[consumed_len.min(input.len())..]))
}

fn parse_cover(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();

    // Parse number followed by %
    let mut i = 0;
    let mut has_dot = false;

    while i < input.len() {
        let ch = input.chars().nth(i).unwrap();
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
        return Err(ParseError::InvalidFormat("Expected number after 'cover'".to_string()));
    }

    let remaining = &input[i..];
    if !remaining.starts_with('%') {
        return Err(ParseError::InvalidFormat("Expected '%' after coverage number".to_string()));
    }

    let number_str = &input[..i];
    let coverage: f32 = number_str
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid coverage number".to_string()))?;

    Ok((QueryAtom::Cover { coverage, stats: Stats::Global }, &remaining[1..]))
}

fn parse_supports(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();

    // Find the feature name (everything up to whitespace or end)
    let end_pos = input.find(|c: char| c.is_whitespace()).unwrap_or(input.len());
    let feature = &input[..end_pos];
    let remaining = &input[end_pos..];

    // Check for support kind (fully, partially)
    let remaining = remaining.trim_start();
    let support_kind = if remaining.starts_with("fully") {
        Some(SupportKind::Fully)
    } else if remaining.starts_with("partially") {
        Some(SupportKind::Partially)
    } else {
        None
    };

    let final_remaining = if support_kind.is_some() {
        let skip_len = if remaining.starts_with("fully") { 5 } else { 9 };
        &remaining[skip_len..]
    } else {
        remaining
    };

    Ok((QueryAtom::Supports(feature, support_kind), final_remaining))
}

fn parse_phantom(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();

    // Phantom can be followed by version numbers
    if input.starts_with("1.9") {
        return Ok((QueryAtom::Phantom(false), &input[3..]));
    } else if input.starts_with("2.1") {
        return Ok((QueryAtom::Phantom(true), &input[3..]));
    }

    // For other versions, just consume what looks like a version
    let end_pos = input.find(|c: char| c.is_whitespace()).unwrap_or(input.len());
    Ok((QueryAtom::Phantom(false), &input[end_pos..]))
}

fn try_parse_browser_version(input: &str) -> Result<Option<(QueryAtom, &str)>, ParseError> {
    let input = input.trim_start();
    let lower_input = input.to_lowercase();

    // Look for known browser names (expand the list)
    let browser_names = [
        ("chrome", "chrome"),
        ("firefox", "firefox"),
        ("safari", "safari"),
        ("edge", "edge"),
        ("ie", "ie"),
        ("opera", "opera"),
        ("samsung", "samsung"),
        ("android", "android"),
        ("ios_saf", "ios_saf"),
        ("op_mob", "op_mob"),
        ("and_chr", "and_chr"),
        ("and_ff", "and_ff"),
        ("kaios", "kaios"),
        ("baidu", "baidu"),
        ("and_qq", "and_qq"),
        ("and_uc", "and_uc"),
        ("op_mini", "op_mini"),
    ];

    for &(browser_lower, browser_canonical) in &browser_names {
        if lower_input.starts_with(browser_lower) {
            let rest = &input[browser_lower.len()..].trim_start();

            // Must be followed by version info or end
            if rest.is_empty() {
                // Just browser name without version - treat as unknown for now
                continue;
            }

            // Check if followed by version info
            let first_char = rest.chars().next().unwrap();
            if !first_char.is_ascii_digit()
                && first_char != '>'
                && first_char != '<'
                && first_char != '='
            {
                continue;
            }

            // Parse version range
            let (version_range, remaining) = parse_version_range(rest)?;
            return Ok(Some((QueryAtom::Browser(browser_canonical, version_range), remaining)));
        }
    }

    Ok(None)
}

fn parse_electron(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();
    let (version_range, remaining) = parse_version_range(input)?;
    Ok((QueryAtom::Electron(version_range), remaining))
}

fn parse_node(input: &str) -> PResult<QueryAtom> {
    let input = input.trim_start();
    let (version_range, remaining) = parse_version_range(input)?;
    Ok((QueryAtom::Node(version_range), remaining))
}

fn parse_version_range(input: &str) -> PResult<VersionRange> {
    let input = input.trim_start();

    // Check for comparators
    let (comparator, offset) = if input.starts_with(">=") {
        (Some(Comparator::GreaterOrEqual), 2)
    } else if input.starts_with("<=") {
        (Some(Comparator::LessOrEqual), 2)
    } else if input.starts_with(">") {
        (Some(Comparator::Greater), 1)
    } else if input.starts_with("<") {
        (Some(Comparator::Less), 1)
    } else if input.starts_with("=") {
        (None, 1) // Exact version
    } else {
        (None, 0) // Exact version without =
    };

    let remaining = &input[offset..].trim_start();

    // Find version string (everything up to whitespace)
    let end_pos = remaining.find(|c: char| c.is_whitespace()).unwrap_or(remaining.len());
    let version = &remaining[..end_pos];

    if version.is_empty() {
        return Err(ParseError::InvalidFormat("Expected version after comparator".to_string()));
    }

    let final_remaining = &remaining[end_pos..];

    if let Some(comp) = comparator {
        Ok((VersionRange::Unbounded(comp, version), final_remaining))
    } else {
        Ok((VersionRange::Accurate(version), final_remaining))
    }
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
    let count: u16 =
        count_str.parse().map_err(|_| ParseError::InvalidFormat("Invalid number".to_string()))?;
    let remaining = &input[i..].trim_start();

    // Check for various patterns
    if remaining.starts_with("major version") {
        let len = if remaining.starts_with("major versions") { 14 } else { 13 };
        return Ok((QueryAtom::Last { count, major: true, name: None }, &remaining[len..]));
    }

    if remaining.starts_with("version") {
        let len = if remaining.starts_with("versions") { 8 } else { 7 };
        return Ok((QueryAtom::Last { count, major: false, name: None }, &remaining[len..]));
    }

    // Parse browser-specific versions like "last 2 Chrome versions"
    let words: Vec<&str> = remaining.split_whitespace().collect();
    if words.len() >= 2 {
        let last_word = words[words.len() - 1];
        if last_word == "versions" || last_word == "version" {
            let second_last = words[words.len() - 2];

            // Check for major versions
            if second_last == "major" && words.len() >= 3 {
                let browser_words = &words[..words.len() - 2];
                let browser_name = browser_words.join(" ");
                let browser_start = remaining.find(&browser_name).unwrap();
                let browser_end = browser_start + browser_name.len();
                return Ok((
                    QueryAtom::Last {
                        count,
                        major: true,
                        name: Some(&remaining[browser_start..browser_end]),
                    },
                    &remaining[remaining.rfind(last_word).unwrap() + last_word.len()..],
                ));
            } else {
                // Regular browser versions
                let browser_words = &words[..words.len() - 1];
                let browser_name = browser_words.join(" ");
                let browser_start = remaining.find(&browser_name).unwrap();
                let browser_end = browser_start + browser_name.len();
                return Ok((
                    QueryAtom::Last {
                        count,
                        major: false,
                        name: Some(&remaining[browser_start..browser_end]),
                    },
                    &remaining[remaining.rfind(last_word).unwrap() + last_word.len()..],
                ));
            }
        }
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
    let popularity: f32 = number_str
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid percentage number".to_string()))?;

    Ok(Some((
        QueryAtom::Percentage { comparator, popularity, stats: Stats::Global },
        &after_number[1..],
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_query() {
        let result = parse_browserslist_query("");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert!(queries.is_empty());
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_defaults_query() {
        let result = parse_browserslist_query("defaults");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert_eq!(queries.len(), 1);
        assert_eq!(remaining, "");
        match &queries[0].atom {
            QueryAtom::Defaults => {}
            _ => panic!("Expected Defaults atom, got {:?}", queries[0].atom),
        }
    }

    #[test]
    fn test_last_versions() {
        let result = parse_browserslist_query("last 2 versions");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert_eq!(queries.len(), 1);
        assert_eq!(remaining, "");
        match &queries[0].atom {
            QueryAtom::Last { count, major, name } => {
                assert_eq!(*count, 2);
                assert_eq!(*major, false);
                assert_eq!(*name, None);
            }
            _ => panic!("Expected Last atom, got {:?}", queries[0].atom),
        }
    }

    #[test]
    fn test_percentage() {
        let result = parse_browserslist_query("> 1%");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert_eq!(queries.len(), 1);
        assert_eq!(remaining, "");
        match &queries[0].atom {
            QueryAtom::Percentage { comparator, popularity, stats } => {
                matches!(comparator, Comparator::Greater);
                assert_eq!(*popularity, 1.0);
                matches!(stats, Stats::Global);
            }
            _ => panic!("Expected Percentage atom, got {:?}", queries[0].atom),
        }
    }

    #[test]
    fn test_firefox_esr() {
        let result = parse_browserslist_query("firefox esr");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert_eq!(queries.len(), 1);
        assert_eq!(remaining, "");
        match &queries[0].atom {
            QueryAtom::FirefoxESR => {}
            _ => panic!("Expected FirefoxESR atom, got {:?}", queries[0].atom),
        }
    }

    #[test]
    fn test_comma_separated() {
        let result = parse_browserslist_query("defaults, > 1%");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert_eq!(queries.len(), 2);
        assert_eq!(remaining, "");
        match &queries[0].atom {
            QueryAtom::Defaults => {}
            _ => panic!("Expected Defaults atom, got {:?}", queries[0].atom),
        }
        match &queries[1].atom {
            QueryAtom::Percentage { .. } => {}
            _ => panic!("Expected Percentage atom, got {:?}", queries[1].atom),
        }
    }

    #[test]
    fn test_last_chrome_versions() {
        let result = parse_browserslist_query("last 2 Chrome versions");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert_eq!(queries.len(), 1);
        assert_eq!(remaining, "");
        match &queries[0].atom {
            QueryAtom::Last { count, major, name } => {
                assert_eq!(*count, 2);
                assert_eq!(*major, false);
                assert_eq!(*name, Some("Chrome"));
            }
            _ => panic!("Expected Last atom, got {:?}", queries[0].atom),
        }
    }

    #[test]
    fn test_and_with_not() {
        let result = parse_browserslist_query("last 2 versions and not > 5%");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert_eq!(queries.len(), 2);
        assert_eq!(remaining, "");
        assert!(!queries[0].negated);
        assert!(!queries[0].is_and);
        assert!(queries[1].negated);
        assert!(queries[1].is_and);
    }

    #[test]
    fn test_browser_version() {
        let result = parse_browserslist_query("chrome >= 50");
        assert!(result.is_ok());
        let (queries, remaining) = result.unwrap();
        assert_eq!(queries.len(), 1);
        assert_eq!(remaining, "");
        match &queries[0].atom {
            QueryAtom::Browser(browser, version_range) => {
                assert_eq!(*browser, "chrome");
                match version_range {
                    VersionRange::Unbounded(comp, version) => {
                        matches!(comp, Comparator::GreaterOrEqual);
                        assert_eq!(*version, "50");
                    }
                    _ => panic!("Expected Unbounded version range"),
                }
            }
            _ => panic!("Expected Browser atom, got {:?}", queries[0].atom),
        }
    }

    #[test]
    fn test_api_integration() {
        // Test that the main resolve function works with our parser
        let result = crate::resolve(&["defaults"], &crate::Opts::default());
        assert!(result.is_ok(), "API should work with manual parser");

        let result2 = crate::resolve(&["last 2 versions"], &crate::Opts::default());
        assert!(result2.is_ok(), "Last N versions should work");
    }
}
