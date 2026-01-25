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

struct Parser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    #[inline]
    fn new(input: &'a str) -> Self {
        Self { input, bytes: input.as_bytes(), pos: 0 }
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    #[inline]
    fn peek(&self) -> u8 {
        self.bytes[self.pos]
    }

    #[inline]
    fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.input[start..end]
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.pos < self.bytes.len() {
            match self.peek() {
                b' ' | b'\t' => self.pos += 1,
                _ => break,
            }
        }
    }

    #[inline]
    fn skip_whitespace1(&mut self) -> bool {
        let start = self.pos;
        self.skip_whitespace();
        self.pos > start
    }

    #[inline]
    fn eat(&mut self, b: u8) -> bool {
        if self.pos < self.bytes.len() && self.peek() == b {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    #[inline]
    fn match_keyword(&mut self, kw: &[u8]) -> bool {
        let end = self.pos + kw.len();
        if end > self.bytes.len() {
            return false;
        }
        let slice = &self.bytes[self.pos..end];
        if !slice.eq_ignore_ascii_case(kw) {
            return false;
        }
        // Word boundary check
        if end < self.bytes.len() {
            let next = self.bytes[end];
            if next.is_ascii_alphanumeric() || next == b'_' {
                return false;
            }
        }
        self.pos = end;
        true
    }

    #[inline]
    fn match_bytes(&mut self, s: &[u8]) -> bool {
        let end = self.pos + s.len();
        if end <= self.bytes.len() {
            let slice = &self.bytes[self.pos..end];
            if slice == s {
                self.pos = end;
                return true;
            }
        }
        false
    }

    #[inline]
    fn match_version_keyword(&mut self) -> bool {
        let end = self.pos + 7;
        if end > self.bytes.len() {
            return false;
        }
        let slice = &self.bytes[self.pos..end];
        if !slice.eq_ignore_ascii_case(b"version") {
            return false;
        }
        self.pos = end;
        // Optional 's'
        if self.pos < self.bytes.len() && matches!(self.peek(), b's' | b'S') {
            self.pos += 1;
        }
        // Word boundary
        self.pos >= self.bytes.len() || {
            let b = self.peek();
            !b.is_ascii_alphanumeric() && b != b'_'
        }
    }

    #[inline]
    fn match_year_keyword(&mut self) -> bool {
        let end = self.pos + 4;
        if end > self.bytes.len() {
            return false;
        }
        let slice = &self.bytes[self.pos..end];
        if !slice.eq_ignore_ascii_case(b"year") {
            return false;
        }
        self.pos = end;
        if self.pos < self.bytes.len() && matches!(self.peek(), b's' | b'S') {
            self.pos += 1;
        }
        self.pos >= self.bytes.len() || {
            let b = self.peek();
            !b.is_ascii_alphanumeric() && b != b'_'
        }
    }

    /// Parse unsigned integer directly from bytes
    #[inline]
    fn parse_u16(&mut self) -> Option<u16> {
        if self.pos >= self.bytes.len() || !self.peek().is_ascii_digit() {
            return None;
        }
        let mut n: u16 = 0;
        while self.pos < self.bytes.len() {
            let b = self.peek();
            if !b.is_ascii_digit() {
                break;
            }
            n = n.wrapping_mul(10).wrapping_add((b - b'0') as u16);
            self.pos += 1;
        }
        Some(n)
    }

    #[inline]
    fn parse_u32(&mut self) -> Option<u32> {
        if self.pos >= self.bytes.len() || !self.peek().is_ascii_digit() {
            return None;
        }
        let mut n: u32 = 0;
        while self.pos < self.bytes.len() {
            let b = self.peek();
            if !b.is_ascii_digit() {
                break;
            }
            n = n.wrapping_mul(10).wrapping_add((b - b'0') as u32);
            self.pos += 1;
        }
        Some(n)
    }

    #[inline]
    fn parse_i32(&mut self) -> Option<i32> {
        let neg = self.eat(b'-');
        if !neg {
            self.eat(b'+');
        }
        let n = self.parse_u32()? as i32;
        Some(if neg { -n } else { n })
    }

    #[inline]
    fn parse_float(&mut self) -> Option<f32> {
        let start = self.pos;
        let _ = self.eat(b'-') || self.eat(b'+');
        while self.pos < self.bytes.len() && self.peek().is_ascii_digit() {
            self.pos += 1;
        }
        if self.eat(b'.') {
            while self.pos < self.bytes.len() && self.peek().is_ascii_digit() {
                self.pos += 1;
            }
        }
        if self.pos > start { self.slice(start, self.pos).parse().ok() } else { None }
    }

    #[inline]
    fn parse_double(&mut self) -> Option<f64> {
        let start = self.pos;
        let _ = self.eat(b'-') || self.eat(b'+');
        while self.pos < self.bytes.len() && self.peek().is_ascii_digit() {
            self.pos += 1;
        }
        if self.eat(b'.') {
            while self.pos < self.bytes.len() && self.peek().is_ascii_digit() {
                self.pos += 1;
            }
        }
        if self.pos > start { self.slice(start, self.pos).parse().ok() } else { None }
    }

    #[inline]
    fn parse_version(&mut self) -> Option<&'a str> {
        let start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.peek();
            if !b.is_ascii_digit() && b != b'.' {
                break;
            }
            self.pos += 1;
        }
        if self.pos > start { Some(self.slice(start, self.pos)) } else { None }
    }

    #[inline]
    fn parse_identifier(&mut self) -> Option<&'a str> {
        let start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.peek();
            if !b.is_ascii_alphabetic() && b != b'_' {
                break;
            }
            self.pos += 1;
        }
        if self.pos > start { Some(self.slice(start, self.pos)) } else { None }
    }

    #[inline]
    fn parse_comparator(&mut self) -> Option<Comparator> {
        if self.pos >= self.bytes.len() {
            return None;
        }
        match self.peek() {
            b'<' => {
                self.pos += 1;
                Some(if self.eat(b'=') { Comparator::LessOrEqual } else { Comparator::Less })
            }
            b'>' => {
                self.pos += 1;
                Some(if self.eat(b'=') { Comparator::GreaterOrEqual } else { Comparator::Greater })
            }
            _ => None,
        }
    }

    #[inline]
    fn parse_region(&mut self) -> Option<&'a str> {
        let start = self.pos;
        // Optional "alt-" prefix
        if self.pos + 4 <= self.bytes.len() {
            let slice = &self.bytes[self.pos..self.pos + 4];
            if slice.eq_ignore_ascii_case(b"alt-") {
                self.pos += 4;
            }
        }
        // Exactly 2 alphabetic
        if self.pos + 2 <= self.bytes.len() {
            let b1 = self.bytes[self.pos];
            let b2 = self.bytes[self.pos + 1];
            if b1.is_ascii_alphabetic() && b2.is_ascii_alphabetic() {
                self.pos += 2;
                return Some(self.slice(start, self.pos));
            }
        }
        self.pos = start;
        None
    }

    #[inline]
    fn parse_version_range(&mut self) -> Option<VersionRange<'a>> {
        let start = self.pos;

        // Try bounded: " 1.0 - 2.0"
        if self.skip_whitespace1() {
            if let Some(from) = self.parse_version() {
                self.skip_whitespace();
                if self.eat(b'-') {
                    self.skip_whitespace();
                    if let Some(to) = self.parse_version() {
                        return Some(VersionRange::Bounded(from, to));
                    }
                }
            }
        }
        self.pos = start;

        // Try unbounded: ">= 1.0"
        self.skip_whitespace();
        if let Some(cmp) = self.parse_comparator() {
            self.skip_whitespace();
            if let Some(ver) = self.parse_version() {
                return Some(VersionRange::Unbounded(cmp, ver));
            }
        }
        self.pos = start;

        // Try accurate: " 1.0"
        if self.skip_whitespace1() {
            if let Some(ver) = self.parse_version() {
                return Some(VersionRange::Accurate(ver));
            }
        }
        self.pos = start;
        None
    }

    // =========================================================================
    // Query atom parsers - organized for first-byte dispatch
    // =========================================================================

    fn parse_last_or_years(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"last") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }

        // Could be "last N versions" or "last N years"
        let before_num = self.pos;

        // Try years first (has fractional)
        if let Some(years) = self.parse_double() {
            if self.skip_whitespace1() && self.match_year_keyword() {
                return Some(QueryAtom::Years(years));
            }
        }
        self.pos = before_num;

        // Try last N versions
        let Some(count) = self.parse_u16() else {
            self.pos = start;
            return None;
        };
        if !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }

        // Optional browser name
        let before_name = self.pos;
        let name = self.parse_identifier().filter(|s| {
            !s.eq_ignore_ascii_case("version")
                && !s.eq_ignore_ascii_case("versions")
                && !s.eq_ignore_ascii_case("major")
                && !s.eq_ignore_ascii_case("year")
                && !s.eq_ignore_ascii_case("years")
        });

        if name.is_some() {
            if !self.skip_whitespace1() {
                self.pos = start;
                return None;
            }
        } else {
            self.pos = before_name;
        }

        let major = if self.match_keyword(b"major") {
            if !self.skip_whitespace1() {
                self.pos = start;
                return None;
            }
            true
        } else {
            false
        };

        if !self.match_version_keyword() {
            self.pos = start;
            return None;
        }

        Some(QueryAtom::Last { count, major, name })
    }

    fn parse_unreleased(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"unreleased") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }

        let before_name = self.pos;
        let name = self
            .parse_identifier()
            .filter(|s| !s.eq_ignore_ascii_case("version") && !s.eq_ignore_ascii_case("versions"));

        if name.is_some() {
            if !self.skip_whitespace1() {
                self.pos = start;
                return None;
            }
        } else {
            self.pos = before_name;
        }

        if !self.match_version_keyword() {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::Unreleased(name))
    }

    fn parse_since_or_supports(&mut self) -> Option<QueryAtom<'a>> {
        if self.pos >= self.bytes.len() {
            return None;
        }
        // Peek second char to disambiguate
        if self.pos + 1 < self.bytes.len() {
            match self.bytes[self.pos + 1] {
                b'i' | b'I' => return self.parse_since(),
                b'u' | b'U' => return self.parse_supports_only(),
                _ => return self.parse_since().or_else(|| self.parse_supports_only()),
            }
        }
        self.parse_since().or_else(|| self.parse_supports_only())
    }

    fn parse_since(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"since") {
            self.pos = start;
            return None;
        }
        if self.pos >= self.bytes.len() || !matches!(self.peek(), b' ' | b'\t') {
            self.pos = start;
            return None;
        }
        self.pos += 1;

        let Some(year) = self.parse_i32() else {
            self.pos = start;
            return None;
        };
        let month = if self.eat(b'-') { self.parse_u32() } else { None };
        let day = if self.eat(b'-') { self.parse_u32() } else { None };

        Some(QueryAtom::Since { year, month: month.unwrap_or(1), day: day.unwrap_or(1) })
    }

    fn parse_supports_only(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"supports") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }

        let feat_start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.peek();
            if !b.is_ascii_alphanumeric() && b != b'-' {
                break;
            }
            self.pos += 1;
        }
        if self.pos == feat_start {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::Supports(self.slice(feat_start, self.pos), None))
    }

    fn parse_cover_or_current(&mut self) -> Option<QueryAtom<'a>> {
        if self.pos + 1 >= self.bytes.len() {
            return None;
        }
        match self.bytes[self.pos + 1] {
            b'o' | b'O' => self.parse_cover(),
            b'u' | b'U' => self.parse_current_node(),
            _ => self.parse_cover().or_else(|| self.parse_current_node()),
        }
    }

    fn parse_cover(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"cover") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }
        let Some(coverage) = self.parse_float() else {
            self.pos = start;
            return None;
        };
        if !self.eat(b'%') {
            self.pos = start;
            return None;
        }

        let stats = if self.skip_whitespace1() && self.match_keyword(b"in") {
            if !self.skip_whitespace1() {
                self.pos = start;
                return None;
            }
            let Some(region) = self.parse_region() else {
                self.pos = start;
                return None;
            };
            Stats::Region(region)
        } else {
            Stats::Global
        };

        Some(QueryAtom::Cover { coverage, stats })
    }

    fn parse_percentage(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        let comparator = self.parse_comparator()?;
        self.skip_whitespace();
        let Some(popularity) = self.parse_float() else {
            self.pos = start;
            return None;
        };
        if !self.eat(b'%') {
            self.pos = start;
            return None;
        }

        let stats = if self.skip_whitespace1() && self.match_keyword(b"in") {
            if !self.skip_whitespace1() {
                self.pos = start;
                return None;
            }
            let Some(region) = self.parse_region() else {
                self.pos = start;
                return None;
            };
            Stats::Region(region)
        } else {
            Stats::Global
        };

        Some(QueryAtom::Percentage { comparator, popularity, stats })
    }

    fn parse_electron_or_extends(&mut self) -> Option<QueryAtom<'a>> {
        if self.pos + 1 >= self.bytes.len() {
            return None;
        }
        match self.bytes[self.pos + 1] {
            b'l' | b'L' => self.parse_electron(),
            b'x' | b'X' => self.parse_extends(),
            _ => self.parse_electron().or_else(|| self.parse_extends()),
        }
    }

    fn parse_electron(&mut self) -> Option<QueryAtom<'a>> {
        if !self.match_keyword(b"electron") {
            return None;
        }
        self.parse_version_range().map(QueryAtom::Electron)
    }

    fn parse_extends(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"extends") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }

        let name_start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.peek();
            if !b.is_ascii_alphanumeric() && !matches!(b, b'-' | b'_' | b'@' | b'/' | b'.') {
                break;
            }
            self.pos += 1;
        }
        if self.pos == name_start {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::Extends(self.slice(name_start, self.pos)))
    }

    fn parse_node(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"node") {
            return None;
        }
        if let Some(range) = self.parse_version_range() {
            return Some(QueryAtom::Node(range));
        }
        self.pos = start;
        None
    }

    fn parse_firefox_or_fully(&mut self) -> Option<QueryAtom<'a>> {
        if self.pos + 1 >= self.bytes.len() {
            return None;
        }
        match self.bytes[self.pos + 1] {
            b'i' | b'I' | b'x' | b'X' | b'f' | b'F' => self.parse_firefox_esr(),
            b'u' | b'U' => self.parse_fully_supports(),
            _ => self.parse_firefox_esr().or_else(|| self.parse_fully_supports()),
        }
    }

    fn parse_firefox_esr(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"firefox")
            && !self.match_keyword(b"ff")
            && !self.match_keyword(b"fx")
        {
            return None;
        }
        if !self.skip_whitespace1() || !self.match_keyword(b"esr") {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::FirefoxESR)
    }

    fn parse_fully_supports(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"fully") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }
        if !self.match_keyword(b"supports") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }
        let feat_start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.peek();
            if !b.is_ascii_alphanumeric() && b != b'-' {
                break;
            }
            self.pos += 1;
        }
        if self.pos == feat_start {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::Supports(self.slice(feat_start, self.pos), Some(SupportKind::Fully)))
    }

    fn parse_operamini(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"operamini") && !self.match_keyword(b"op_mini") {
            return None;
        }
        if !self.skip_whitespace1() || !self.match_keyword(b"all") {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::OperaMini)
    }

    fn parse_current_node(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"current")
            || !self.skip_whitespace1()
            || !self.match_keyword(b"node")
        {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::CurrentNode)
    }

    fn parse_maintained_node(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"maintained")
            || !self.skip_whitespace1()
            || !self.match_keyword(b"node")
            || !self.skip_whitespace1()
            || !self.match_keyword(b"versions")
        {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::MaintainedNode)
    }

    fn parse_phantom_or_partially(&mut self) -> Option<QueryAtom<'a>> {
        if self.pos + 1 >= self.bytes.len() {
            return None;
        }
        match self.bytes[self.pos + 1] {
            b'h' | b'H' => self.parse_phantom(),
            b'a' | b'A' => self.parse_partially_supports(),
            _ => self.parse_phantom().or_else(|| self.parse_partially_supports()),
        }
    }

    fn parse_phantom(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"phantomjs") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }
        if self.match_bytes(b"2.1") {
            Some(QueryAtom::Phantom(true))
        } else if self.match_bytes(b"1.9") {
            Some(QueryAtom::Phantom(false))
        } else {
            self.pos = start;
            None
        }
    }

    fn parse_partially_supports(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"partially") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }
        if !self.match_keyword(b"supports") || !self.skip_whitespace1() {
            self.pos = start;
            return None;
        }
        let feat_start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.peek();
            if !b.is_ascii_alphanumeric() && b != b'-' {
                break;
            }
            self.pos += 1;
        }
        if self.pos == feat_start {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::Supports(self.slice(feat_start, self.pos), Some(SupportKind::Partially)))
    }

    fn parse_browserslist_config(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        if !self.match_keyword(b"browserslist")
            || !self.skip_whitespace1()
            || !self.match_keyword(b"config")
        {
            self.pos = start;
            return None;
        }
        Some(QueryAtom::BrowserslistConfig)
    }

    fn parse_defaults_or_dead(&mut self) -> Option<QueryAtom<'a>> {
        if self.pos + 2 >= self.bytes.len() {
            return None;
        }
        match self.bytes[self.pos + 2] {
            b'f' | b'F' => {
                if self.match_keyword(b"defaults") {
                    Some(QueryAtom::Defaults)
                } else {
                    None
                }
            }
            b'a' | b'A' => {
                if self.match_keyword(b"dead") {
                    Some(QueryAtom::Dead)
                } else {
                    None
                }
            }
            // "defaults" has 'f' as third char, "dead" has 'a' as third char
            // So this fallback can never match either keyword
            _ => None,
        }
    }

    fn parse_browser(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        let name = self.parse_identifier()?;

        if let Some(range) = self.parse_version_range() {
            return Some(QueryAtom::Browser(name, range));
        }

        if self.skip_whitespace1() && self.match_keyword(b"tp") {
            return Some(QueryAtom::Browser(name, VersionRange::Accurate("tp")));
        }

        self.pos = start;
        None
    }

    /// Dispatch to the appropriate parser based on first character
    #[inline]
    fn parse_query_atom(&mut self) -> Option<QueryAtom<'a>> {
        if self.pos >= self.bytes.len() {
            return None;
        }

        match self.peek() | 0x20 {
            // ASCII lowercase
            b'<' | b'>' => self.parse_percentage(),
            b'l' => self.parse_last_or_years().or_else(|| self.parse_browser()),
            b'u' => self.parse_unreleased().or_else(|| self.parse_browser()),
            b's' => self.parse_since_or_supports().or_else(|| self.parse_browser()),
            b'c' => self.parse_cover_or_current().or_else(|| self.parse_browser()),
            b'e' => self.parse_electron_or_extends().or_else(|| self.parse_browser()),
            b'n' => self.parse_node().or_else(|| self.parse_browser()),
            b'f' => self.parse_firefox_or_fully().or_else(|| self.parse_browser()),
            b'o' => self.parse_operamini().or_else(|| self.parse_browser()),
            b'm' => self.parse_maintained_node().or_else(|| self.parse_browser()),
            b'p' => self.parse_phantom_or_partially().or_else(|| self.parse_browser()),
            b'b' => self.parse_browserslist_config().or_else(|| self.parse_browser()),
            b'd' => self.parse_defaults_or_dead().or_else(|| self.parse_browser()),
            b'a'..=b'z' => self.parse_browser(),
            _ => None,
        }
    }

    #[inline]
    fn at_composition_operator(&self) -> bool {
        if self.pos >= self.bytes.len() {
            return false;
        }
        if self.peek() == b',' {
            return true;
        }
        if !matches!(self.peek(), b' ' | b'\t') {
            return false;
        }
        // Skip whitespace to check for "and"/"or"
        let mut i = self.pos;
        while i < self.bytes.len() && matches!(self.bytes[i], b' ' | b'\t') {
            i += 1;
        }
        let rest = &self.bytes[i..];
        (rest.len() >= 4
            && rest[..3].eq_ignore_ascii_case(b"and")
            && matches!(rest[3], b' ' | b'\t'))
            || (rest.len() >= 3
                && rest[..2].eq_ignore_ascii_case(b"or")
                && matches!(rest[2], b' ' | b'\t'))
    }

    #[inline]
    fn parse_composition_operator(&mut self) -> Option<bool> {
        self.skip_whitespace();

        if self.eat(b',') {
            self.skip_whitespace();
            return Some(false);
        }

        if self.match_keyword(b"and") && self.skip_whitespace1() {
            return Some(true);
        } else if self.match_keyword(b"or") && self.skip_whitespace1() {
            return Some(false);
        }
        None
    }

    fn parse_unknown(&mut self) -> Option<QueryAtom<'a>> {
        let start = self.pos;
        while !self.is_eof() && !self.at_composition_operator() {
            self.pos += 1;
        }
        if self.pos > start { Some(QueryAtom::Unknown(self.slice(start, self.pos))) } else { None }
    }

    fn parse_single_query_atom(&mut self) -> Option<(bool, QueryAtom<'a>, &'a str)> {
        let start = self.pos;

        let negated = if self.match_keyword(b"not") && self.skip_whitespace1() {
            true
        } else {
            self.pos = start;
            false
        };

        let atom = self.parse_query_atom().or_else(|| self.parse_unknown())?;
        Some((negated, atom, self.slice(start, self.pos)))
    }
}

pub fn parse_browserslist_query(input: &str) -> Result<(&str, Vec<SingleQuery<'_>>), &str> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(("", vec![]));
    }

    let mut parser = Parser::new(input);
    let mut queries = Vec::with_capacity(4);

    let (negated, atom, raw) = parser.parse_single_query_atom().ok_or(input)?;
    queries.push(SingleQuery { raw, atom, negated, is_and: false });

    while !parser.is_eof() {
        let Some(is_and) = parser.parse_composition_operator() else {
            return Err(parser.slice(parser.pos, parser.bytes.len()));
        };

        let (negated, atom, raw) =
            parser.parse_single_query_atom().ok_or(parser.slice(parser.pos, parser.bytes.len()))?;
        queries.push(SingleQuery { raw, atom, negated, is_and });
    }

    Ok(("", queries))
}

#[cfg(all(test, not(miri)))]
mod tests {
    use test_case::test_case;

    use crate::{opts::Opts, resolve, test::run_compare};

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

    // Tests for edge cases and improved coverage
    #[test_case("> .5%"; "float with leading dot")]
    #[test_case(">= 0.1%"; "percentage with zero")]
    #[test_case("< 1%"; "less than percentage")]
    #[test_case("<= 5%"; "less or equal percentage")]
    #[test_case("> 1% in US"; "percentage in region")]
    #[test_case("> 1% in alt-AS"; "percentage in alt region")]
    #[test_case("cover 0.5% in US"; "cover in region")]
    #[test_case("cover 0.1% in alt-EU"; "cover in alt region")]
    fn percentage_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("since 2020"; "since year only")]
    #[test_case("since 2020-06"; "since year month")]
    #[test_case("since 2020-06-15"; "since year month day")]
    fn since_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("last 1 year"; "singular year")]
    #[test_case("last 2 years"; "plural years")]
    #[test_case("last 1.5 years"; "fractional years")]
    fn years_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("last 1 version"; "singular version")]
    #[test_case("last 2 versions"; "plural versions")]
    #[test_case("last 1 major version"; "singular major version")]
    #[test_case("last 2 major versions"; "plural major versions")]
    #[test_case("last 1 Chrome version"; "browser singular")]
    #[test_case("last 2 Chrome versions"; "browser plural")]
    #[test_case("last 1 Chrome major version"; "browser major singular")]
    #[test_case("last 2 Chrome major versions"; "browser major plural")]
    fn last_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("unreleased versions"; "unreleased all")]
    #[test_case("unreleased Chrome versions"; "unreleased browser")]
    #[test_case("unreleased electron versions"; "unreleased electron")]
    fn unreleased_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("supports es6-module"; "supports")]
    #[test_case("fully supports es6-module"; "fully supports")]
    #[test_case("partially supports es6-module"; "partially supports")]
    fn supports_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("firefox esr"; "firefox esr")]
    #[test_case("ff esr"; "ff esr")]
    #[test_case("fx esr"; "fx esr")]
    #[test_case("Firefox ESR"; "firefox esr uppercase")]
    fn firefox_esr_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("operamini all"; "operamini all")]
    #[test_case("op_mini all"; "op mini all")]
    fn operamini_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("phantomjs 2.1"; "phantom 2.1")]
    #[test_case("phantomjs 1.9"; "phantom 1.9")]
    fn phantom_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("maintained node versions"; "maintained node")]
    fn maintained_node_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("node >= 10"; "node unbounded")]
    #[test_case("node 10 - 14"; "node bounded")]
    #[test_case("node 18"; "node accurate")]
    fn node_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("electron >= 1.0"; "electron unbounded")]
    #[test_case("electron 0.36 - 1.2"; "electron bounded")]
    #[test_case("electron 1.1"; "electron accurate")]
    fn electron_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("defaults"; "defaults")]
    #[test_case("dead"; "dead")]
    #[test_case("Defaults"; "defaults uppercase")]
    #[test_case("Dead"; "dead uppercase")]
    fn defaults_dead_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("safari tp"; "safari tp")]
    #[test_case("Safari TP"; "safari tp uppercase")]
    fn browser_tp_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("ie 8 - 10"; "ie range")]
    #[test_case("chrome >= 50"; "chrome unbounded")]
    #[test_case("chrome 90"; "chrome accurate")]
    fn browser_range_edge_cases(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    // Tests for parser error paths
    #[test]
    fn invalid_queries_return_unknown() {
        // Queries that don't match any pattern should return Unknown error
        let result = resolve(&["unknown_query_xyz"], &Opts::default());
        assert!(result.is_err());
    }

    #[test]
    fn parse_composition_with_extra_spaces() {
        run_compare("ie >= 6   and   ie <= 7", &Opts::default(), None);
        run_compare("ie >= 6  ,  ie <= 7", &Opts::default(), None);
        run_compare("ie >= 6   or   ie <= 7", &Opts::default(), None);
    }

    #[test]
    fn case_insensitive_keywords() {
        // Test that uppercase keywords are parsed correctly
        // We just verify they don't error, not exact results (bundled data may differ)
        assert!(resolve(&["LAST 2 VERSIONS"], &Opts::default()).is_ok());
        assert!(resolve(&["DEFAULTS"], &Opts::default()).is_ok());
        assert!(resolve(&["DEAD"], &Opts::default()).is_ok());
        assert!(resolve(&["SUPPORTS es6-module"], &Opts::default()).is_ok());
        assert!(resolve(&["COVER 0.1%"], &Opts::default()).is_ok());
        assert!(resolve(&["SINCE 2020"], &Opts::default()).is_ok());
    }

    // Unit tests for parser internals to improve coverage
    mod parser_internals {
        use super::super::*;

        #[test]
        fn parse_browserslist_query_empty() {
            let result = parse_browserslist_query("");
            assert!(result.is_ok());
            assert!(result.unwrap().1.is_empty());
        }

        #[test]
        fn parse_browserslist_query_whitespace_only() {
            let result = parse_browserslist_query("   ");
            assert!(result.is_ok());
            assert!(result.unwrap().1.is_empty());
        }

        #[test]
        fn parse_float_leading_dot() {
            let mut parser = Parser::new(".5");
            let result = parser.parse_float();
            assert!(result.is_some());
            assert_eq!(result.unwrap(), 0.5);
        }

        #[test]
        fn parse_double_leading_dot() {
            let mut parser = Parser::new(".25");
            let result = parser.parse_double();
            assert!(result.is_some());
            assert!((result.unwrap() - 0.25).abs() < 0.001);
        }

        #[test]
        fn parse_version_basic() {
            let mut parser = Parser::new("1.2.3");
            let result = parser.parse_version();
            assert_eq!(result, Some("1.2.3"));
        }

        #[test]
        fn parse_version_empty() {
            let mut parser = Parser::new("abc");
            let result = parser.parse_version();
            assert_eq!(result, None);
        }

        #[test]
        fn parse_identifier_basic() {
            let mut parser = Parser::new("chrome");
            let result = parser.parse_identifier();
            assert_eq!(result, Some("chrome"));
        }

        #[test]
        fn parse_identifier_with_underscore() {
            let mut parser = Parser::new("op_mini");
            let result = parser.parse_identifier();
            assert_eq!(result, Some("op_mini"));
        }

        #[test]
        fn parse_comparator_all_types() {
            let mut p1 = Parser::new("<");
            assert!(matches!(p1.parse_comparator(), Some(Comparator::Less)));

            let mut p2 = Parser::new("<=");
            assert!(matches!(p2.parse_comparator(), Some(Comparator::LessOrEqual)));

            let mut p3 = Parser::new(">");
            assert!(matches!(p3.parse_comparator(), Some(Comparator::Greater)));

            let mut p4 = Parser::new(">=");
            assert!(matches!(p4.parse_comparator(), Some(Comparator::GreaterOrEqual)));

            let mut p5 = Parser::new("x");
            assert!(p5.parse_comparator().is_none());
        }

        #[test]
        fn parse_region_basic() {
            let mut parser = Parser::new("US");
            let result = parser.parse_region();
            assert_eq!(result, Some("US"));
        }

        #[test]
        fn parse_region_alt_prefix() {
            let mut parser = Parser::new("alt-AS");
            let result = parser.parse_region();
            assert_eq!(result, Some("alt-AS"));
        }

        #[test]
        fn parse_region_invalid() {
            let mut parser = Parser::new("1X");
            let result = parser.parse_region();
            assert_eq!(result, None);
        }

        #[test]
        fn parse_i32_negative() {
            let mut parser = Parser::new("-2020");
            let result = parser.parse_i32();
            assert_eq!(result, Some(-2020));
        }

        #[test]
        fn parse_i32_positive_with_sign() {
            let mut parser = Parser::new("+2020");
            let result = parser.parse_i32();
            assert_eq!(result, Some(2020));
        }

        #[test]
        fn match_keyword_word_boundary() {
            let mut parser = Parser::new("lastX");
            assert!(!parser.match_keyword(b"last"));
        }

        #[test]
        fn match_keyword_too_short() {
            let mut parser = Parser::new("la");
            assert!(!parser.match_keyword(b"last"));
        }

        #[test]
        fn match_version_keyword_singular() {
            let mut parser = Parser::new("version");
            assert!(parser.match_version_keyword());
        }

        #[test]
        fn match_version_keyword_plural() {
            let mut parser = Parser::new("versions");
            assert!(parser.match_version_keyword());
        }

        #[test]
        fn match_year_keyword_singular() {
            let mut parser = Parser::new("year");
            assert!(parser.match_year_keyword());
        }

        #[test]
        fn match_year_keyword_plural() {
            let mut parser = Parser::new("years");
            assert!(parser.match_year_keyword());
        }

        #[test]
        fn match_year_keyword_invalid() {
            let mut parser = Parser::new("yearly");
            assert!(!parser.match_year_keyword());
        }

        #[test]
        fn parse_version_range_bounded() {
            let mut parser = Parser::new(" 1.0 - 2.0");
            let result = parser.parse_version_range();
            assert!(matches!(result, Some(VersionRange::Bounded("1.0", "2.0"))));
        }

        #[test]
        fn parse_version_range_unbounded() {
            let mut parser = Parser::new(" >= 1.0");
            let result = parser.parse_version_range();
            assert!(matches!(
                result,
                Some(VersionRange::Unbounded(Comparator::GreaterOrEqual, "1.0"))
            ));
        }

        #[test]
        fn parse_version_range_accurate() {
            let mut parser = Parser::new(" 1.0");
            let result = parser.parse_version_range();
            assert!(matches!(result, Some(VersionRange::Accurate("1.0"))));
        }

        #[test]
        fn parse_version_range_no_space() {
            let mut parser = Parser::new("1.0");
            let result = parser.parse_version_range();
            assert!(result.is_none());
        }

        #[test]
        fn parse_query_atom_defaults() {
            let mut parser = Parser::new("defaults");
            let result = parser.parse_query_atom();
            assert!(matches!(result, Some(QueryAtom::Defaults)));
        }

        #[test]
        fn parse_query_atom_dead() {
            let mut parser = Parser::new("dead");
            let result = parser.parse_query_atom();
            assert!(matches!(result, Some(QueryAtom::Dead)));
        }

        #[test]
        fn parse_query_atom_percentage() {
            let mut parser = Parser::new("> 1%");
            let result = parser.parse_query_atom();
            assert!(matches!(result, Some(QueryAtom::Percentage { .. })));
        }

        #[test]
        fn parse_query_atom_browser() {
            let mut parser = Parser::new("chrome 90");
            let result = parser.parse_query_atom();
            assert!(matches!(result, Some(QueryAtom::Browser(_, _))));
        }

        #[test]
        fn parse_unknown_query() {
            let mut parser = Parser::new("###invalid###");
            let result = parser.parse_unknown();
            assert!(matches!(result, Some(QueryAtom::Unknown("###invalid###"))));
        }

        #[test]
        fn parse_browserslist_with_or() {
            let result = parse_browserslist_query("ie 10 or ie 11");
            assert!(result.is_ok());
            let (_, queries) = result.unwrap();
            assert_eq!(queries.len(), 2);
            assert!(!queries[1].is_and);
        }

        #[test]
        fn parse_browserslist_with_and() {
            let result = parse_browserslist_query("ie >= 10 and ie <= 11");
            assert!(result.is_ok());
            let (_, queries) = result.unwrap();
            assert_eq!(queries.len(), 2);
            assert!(queries[1].is_and);
        }

        #[test]
        fn parse_browserslist_with_not() {
            let result = parse_browserslist_query("ie >= 10, not ie 11");
            assert!(result.is_ok());
            let (_, queries) = result.unwrap();
            assert_eq!(queries.len(), 2);
            assert!(queries[1].negated);
        }

        // ================================================================
        // Tests for uncovered error paths
        // ================================================================

        #[test]
        fn match_bytes_no_match() {
            let mut parser = Parser::new("abc");
            assert!(!parser.match_bytes(b"xyz"));
        }

        #[test]
        fn match_bytes_too_short() {
            let mut parser = Parser::new("a");
            assert!(!parser.match_bytes(b"abc"));
        }

        #[test]
        fn match_version_keyword_too_short() {
            let mut parser = Parser::new("ver");
            assert!(!parser.match_version_keyword());
        }

        #[test]
        fn match_version_keyword_wrong_word() {
            let mut parser = Parser::new("verbose");
            assert!(!parser.match_version_keyword());
        }

        #[test]
        fn match_year_keyword_too_short() {
            let mut parser = Parser::new("ye");
            assert!(!parser.match_year_keyword());
        }

        #[test]
        fn parse_u16_non_digit() {
            let mut parser = Parser::new("abc");
            assert!(parser.parse_u16().is_none());
        }

        #[test]
        fn parse_u16_empty() {
            let mut parser = Parser::new("");
            assert!(parser.parse_u16().is_none());
        }

        #[test]
        fn parse_u32_non_digit() {
            let mut parser = Parser::new("xyz");
            assert!(parser.parse_u32().is_none());
        }

        #[test]
        fn parse_u32_empty() {
            let mut parser = Parser::new("");
            assert!(parser.parse_u32().is_none());
        }

        #[test]
        fn parse_region_single_char() {
            let mut parser = Parser::new("U");
            assert!(parser.parse_region().is_none());
        }

        #[test]
        fn parse_region_non_alpha() {
            let mut parser = Parser::new("12");
            assert!(parser.parse_region().is_none());
        }

        #[test]
        fn parse_version_range_bounded_incomplete() {
            // Has "from" version but missing "to" version after dash
            let mut parser = Parser::new(" 1.0 - ");
            let result = parser.parse_version_range();
            // Falls back to accurate version
            assert!(matches!(result, Some(VersionRange::Accurate("1.0"))));
        }

        #[test]
        fn parse_version_range_unbounded_no_version() {
            let mut parser = Parser::new(" >= ");
            let result = parser.parse_version_range();
            assert!(result.is_none());
        }

        // Test parse_last_or_years failure paths
        #[test]
        fn parse_last_no_whitespace_after() {
            let mut parser = Parser::new("last");
            let result = parser.parse_last_or_years();
            assert!(result.is_none());
        }

        #[test]
        fn parse_last_no_number() {
            let mut parser = Parser::new("last abc");
            let result = parser.parse_last_or_years();
            assert!(result.is_none());
        }

        #[test]
        fn parse_last_number_no_whitespace() {
            let mut parser = Parser::new("last 2");
            let result = parser.parse_last_or_years();
            assert!(result.is_none());
        }

        #[test]
        fn parse_last_browser_no_whitespace() {
            let mut parser = Parser::new("last 2 chrome");
            let result = parser.parse_last_or_years();
            assert!(result.is_none());
        }

        #[test]
        fn parse_last_major_no_whitespace() {
            let mut parser = Parser::new("last 2 major");
            let result = parser.parse_last_or_years();
            assert!(result.is_none());
        }

        #[test]
        fn parse_last_no_version_keyword() {
            let mut parser = Parser::new("last 2 xyz");
            let result = parser.parse_last_or_years();
            assert!(result.is_none());
        }

        #[test]
        fn parse_last_major_versions_special() {
            // Test "last 2 major versions" where name is "major"
            let mut parser = Parser::new("last 2 major versions");
            let result = parser.parse_last_or_years();
            assert!(matches!(result, Some(QueryAtom::Last { count: 2, major: true, name: None })));
        }

        // Test parse_unreleased failure paths
        #[test]
        fn parse_unreleased_no_whitespace() {
            let mut parser = Parser::new("unreleased");
            let result = parser.parse_unreleased();
            assert!(result.is_none());
        }

        #[test]
        fn parse_unreleased_browser_no_whitespace() {
            let mut parser = Parser::new("unreleased chrome");
            let result = parser.parse_unreleased();
            assert!(result.is_none());
        }

        #[test]
        fn parse_unreleased_no_version_keyword() {
            let mut parser = Parser::new("unreleased xyz abc");
            let result = parser.parse_unreleased();
            assert!(result.is_none());
        }

        // Test parse_since failure paths
        #[test]
        fn parse_since_no_whitespace() {
            let mut parser = Parser::new("since");
            let result = parser.parse_since();
            assert!(result.is_none());
        }

        #[test]
        fn parse_since_no_year() {
            let mut parser = Parser::new("since abc");
            let result = parser.parse_since();
            assert!(result.is_none());
        }

        #[test]
        fn parse_since_or_supports_empty() {
            let mut parser = Parser::new("");
            let result = parser.parse_since_or_supports();
            assert!(result.is_none());
        }

        #[test]
        fn parse_since_or_supports_fallback() {
            // Starts with 's' but second char is neither 'i' nor 'u'
            let mut parser = Parser::new("safari 10");
            let result = parser.parse_since_or_supports();
            assert!(result.is_none()); // Falls through to browser parsing later
        }

        // Test parse_supports failure paths
        #[test]
        fn parse_supports_no_whitespace() {
            let mut parser = Parser::new("supports");
            let result = parser.parse_supports_only();
            assert!(result.is_none());
        }

        #[test]
        fn parse_supports_no_feature() {
            let mut parser = Parser::new("supports ");
            let result = parser.parse_supports_only();
            assert!(result.is_none());
        }

        // Test parse_cover failure paths
        #[test]
        fn parse_cover_no_whitespace() {
            let mut parser = Parser::new("cover");
            let result = parser.parse_cover();
            assert!(result.is_none());
        }

        #[test]
        fn parse_cover_no_number() {
            let mut parser = Parser::new("cover abc");
            let result = parser.parse_cover();
            assert!(result.is_none());
        }

        #[test]
        fn parse_cover_no_percent() {
            let mut parser = Parser::new("cover 50");
            let result = parser.parse_cover();
            assert!(result.is_none());
        }

        #[test]
        fn parse_cover_in_no_whitespace() {
            let mut parser = Parser::new("cover 50% in");
            let result = parser.parse_cover();
            assert!(result.is_none());
        }

        #[test]
        fn parse_cover_in_no_region() {
            let mut parser = Parser::new("cover 50% in 12");
            let result = parser.parse_cover();
            assert!(result.is_none());
        }

        #[test]
        fn parse_cover_or_current_short() {
            let mut parser = Parser::new("c");
            let result = parser.parse_cover_or_current();
            assert!(result.is_none());
        }

        #[test]
        fn parse_cover_or_current_fallback() {
            // Second char is neither 'o' nor 'u'
            let mut parser = Parser::new("chrome 90");
            let result = parser.parse_cover_or_current();
            assert!(result.is_none()); // Falls to browser parsing
        }

        // Test parse_percentage failure paths
        #[test]
        fn parse_percentage_no_number() {
            let mut parser = Parser::new("> abc");
            let result = parser.parse_percentage();
            assert!(result.is_none());
        }

        #[test]
        fn parse_percentage_no_percent() {
            let mut parser = Parser::new("> 50");
            let result = parser.parse_percentage();
            assert!(result.is_none());
        }

        #[test]
        fn parse_percentage_in_no_whitespace() {
            let mut parser = Parser::new("> 50% in");
            let result = parser.parse_percentage();
            assert!(result.is_none());
        }

        #[test]
        fn parse_percentage_in_no_region() {
            let mut parser = Parser::new("> 50% in 12");
            let result = parser.parse_percentage();
            assert!(result.is_none());
        }

        // Test parse_electron_or_extends failure paths
        #[test]
        fn parse_electron_or_extends_short() {
            let mut parser = Parser::new("e");
            let result = parser.parse_electron_or_extends();
            assert!(result.is_none());
        }

        #[test]
        fn parse_electron_or_extends_fallback() {
            // Second char is neither 'l' nor 'x'
            let mut parser = Parser::new("edge 90");
            let result = parser.parse_electron_or_extends();
            assert!(result.is_none());
        }

        #[test]
        fn parse_electron_no_keyword() {
            let mut parser = Parser::new("electronics");
            let result = parser.parse_electron();
            assert!(result.is_none());
        }

        #[test]
        fn parse_extends_no_whitespace() {
            let mut parser = Parser::new("extends");
            let result = parser.parse_extends();
            assert!(result.is_none());
        }

        #[test]
        fn parse_extends_no_package() {
            let mut parser = Parser::new("extends ");
            let result = parser.parse_extends();
            assert!(result.is_none());
        }

        // Test parse_node failure paths
        #[test]
        fn parse_node_no_keyword() {
            let mut parser = Parser::new("nodejs");
            let result = parser.parse_node();
            assert!(result.is_none());
        }

        #[test]
        fn parse_node_no_version() {
            let mut parser = Parser::new("node");
            let result = parser.parse_node();
            assert!(result.is_none());
        }

        // Test parse_firefox_or_fully failure paths
        #[test]
        fn parse_firefox_or_fully_short() {
            let mut parser = Parser::new("f");
            let result = parser.parse_firefox_or_fully();
            assert!(result.is_none());
        }

        #[test]
        fn parse_firefox_or_fully_fallback() {
            // Second char is not 'i', 'x', 'f', or 'u'
            let mut parser = Parser::new("fa");
            let result = parser.parse_firefox_or_fully();
            assert!(result.is_none());
        }

        #[test]
        fn parse_firefox_esr_no_keyword() {
            let mut parser = Parser::new("fire");
            let result = parser.parse_firefox_esr();
            assert!(result.is_none());
        }

        #[test]
        fn parse_firefox_esr_no_whitespace() {
            let mut parser = Parser::new("firefox");
            let result = parser.parse_firefox_esr();
            assert!(result.is_none());
        }

        #[test]
        fn parse_firefox_esr_no_esr() {
            let mut parser = Parser::new("firefox xyz");
            let result = parser.parse_firefox_esr();
            assert!(result.is_none());
        }

        #[test]
        fn parse_fully_supports_no_whitespace() {
            let mut parser = Parser::new("fully");
            let result = parser.parse_fully_supports();
            assert!(result.is_none());
        }

        #[test]
        fn parse_fully_supports_no_supports() {
            let mut parser = Parser::new("fully abc");
            let result = parser.parse_fully_supports();
            assert!(result.is_none());
        }

        #[test]
        fn parse_fully_supports_supports_no_whitespace() {
            let mut parser = Parser::new("fully supports");
            let result = parser.parse_fully_supports();
            assert!(result.is_none());
        }

        #[test]
        fn parse_fully_supports_no_feature() {
            let mut parser = Parser::new("fully supports ");
            let result = parser.parse_fully_supports();
            assert!(result.is_none());
        }

        // Test parse_operamini failure paths
        #[test]
        fn parse_operamini_no_whitespace() {
            let mut parser = Parser::new("operamini");
            let result = parser.parse_operamini();
            assert!(result.is_none());
        }

        #[test]
        fn parse_operamini_no_all() {
            let mut parser = Parser::new("operamini xyz");
            let result = parser.parse_operamini();
            assert!(result.is_none());
        }

        // Test parse_current_node failure paths
        #[test]
        fn parse_current_node_partial() {
            let mut parser = Parser::new("current");
            let result = parser.parse_current_node();
            assert!(result.is_none());
        }

        #[test]
        fn parse_current_node_no_node() {
            let mut parser = Parser::new("current xyz");
            let result = parser.parse_current_node();
            assert!(result.is_none());
        }

        // Test parse_maintained_node failure paths
        #[test]
        fn parse_maintained_node_partial() {
            let mut parser = Parser::new("maintained");
            let result = parser.parse_maintained_node();
            assert!(result.is_none());
        }

        // Test parse_phantom_or_partially failure paths
        #[test]
        fn parse_phantom_or_partially_short() {
            let mut parser = Parser::new("p");
            let result = parser.parse_phantom_or_partially();
            assert!(result.is_none());
        }

        #[test]
        fn parse_phantom_or_partially_fallback() {
            // Second char is neither 'h' nor 'a'
            let mut parser = Parser::new("px");
            let result = parser.parse_phantom_or_partially();
            assert!(result.is_none());
        }

        #[test]
        fn parse_phantom_no_whitespace() {
            let mut parser = Parser::new("phantomjs");
            let result = parser.parse_phantom();
            assert!(result.is_none());
        }

        #[test]
        fn parse_phantom_wrong_version() {
            let mut parser = Parser::new("phantomjs 3.0");
            let result = parser.parse_phantom();
            assert!(result.is_none());
        }

        #[test]
        fn parse_partially_supports_no_whitespace() {
            let mut parser = Parser::new("partially");
            let result = parser.parse_partially_supports();
            assert!(result.is_none());
        }

        #[test]
        fn parse_partially_supports_no_supports() {
            let mut parser = Parser::new("partially abc");
            let result = parser.parse_partially_supports();
            assert!(result.is_none());
        }

        #[test]
        fn parse_partially_supports_supports_no_whitespace() {
            let mut parser = Parser::new("partially supports");
            let result = parser.parse_partially_supports();
            assert!(result.is_none());
        }

        #[test]
        fn parse_partially_supports_no_feature() {
            let mut parser = Parser::new("partially supports ");
            let result = parser.parse_partially_supports();
            assert!(result.is_none());
        }

        // Test parse_browserslist_config failure paths
        #[test]
        fn parse_browserslist_config_partial() {
            let mut parser = Parser::new("browserslist");
            let result = parser.parse_browserslist_config();
            assert!(result.is_none());
        }

        #[test]
        fn parse_browserslist_config_no_config() {
            let mut parser = Parser::new("browserslist xyz");
            let result = parser.parse_browserslist_config();
            assert!(result.is_none());
        }

        // Test parse_defaults_or_dead failure paths
        #[test]
        fn parse_defaults_or_dead_short() {
            let mut parser = Parser::new("de");
            let result = parser.parse_defaults_or_dead();
            assert!(result.is_none());
        }

        #[test]
        fn parse_defaults_or_dead_fallback() {
            // Third char is neither 'f' nor 'a' - test the fallback branch
            let mut parser = Parser::new("dex");
            let result = parser.parse_defaults_or_dead();
            assert!(result.is_none());
        }

        #[test]
        fn parse_defaults_or_dead_def_but_not_defaults() {
            let mut parser = Parser::new("default"); // Missing 's'
            let result = parser.parse_defaults_or_dead();
            assert!(result.is_none());
        }

        #[test]
        fn parse_defaults_or_dead_dea_but_not_dead() {
            let mut parser = Parser::new("dealer");
            let result = parser.parse_defaults_or_dead();
            assert!(result.is_none());
        }

        // Test parse_browser failure paths
        #[test]
        fn parse_browser_no_version() {
            let mut parser = Parser::new("chrome");
            let result = parser.parse_browser();
            assert!(result.is_none());
        }

        // Test parse_query_atom edge cases
        #[test]
        fn parse_query_atom_empty() {
            let mut parser = Parser::new("");
            let result = parser.parse_query_atom();
            assert!(result.is_none());
        }

        #[test]
        fn parse_query_atom_non_letter() {
            let mut parser = Parser::new("123");
            let result = parser.parse_query_atom();
            assert!(result.is_none());
        }

        #[test]
        fn parse_query_atom_special_char() {
            let mut parser = Parser::new("@#$");
            let result = parser.parse_query_atom();
            assert!(result.is_none());
        }

        // Test at_composition_operator paths
        #[test]
        fn at_composition_operator_empty() {
            let parser = Parser::new("");
            assert!(!parser.at_composition_operator());
        }

        #[test]
        fn at_composition_operator_comma() {
            let parser = Parser::new(",");
            assert!(parser.at_composition_operator());
        }

        #[test]
        fn at_composition_operator_no_space() {
            let parser = Parser::new("x");
            assert!(!parser.at_composition_operator());
        }

        #[test]
        fn at_composition_operator_space_then_and() {
            let parser = Parser::new(" and ");
            assert!(parser.at_composition_operator());
        }

        #[test]
        fn at_composition_operator_space_then_or() {
            let parser = Parser::new(" or ");
            assert!(parser.at_composition_operator());
        }

        #[test]
        fn at_composition_operator_space_then_other() {
            let parser = Parser::new(" xyz");
            assert!(!parser.at_composition_operator());
        }

        // Test parse_unknown with composition operator
        #[test]
        fn parse_unknown_stops_at_comma() {
            let mut parser = Parser::new("abc,def");
            let result = parser.parse_unknown();
            assert!(matches!(result, Some(QueryAtom::Unknown("abc"))));
        }

        #[test]
        fn parse_unknown_stops_at_and() {
            let mut parser = Parser::new("abc and def");
            let result = parser.parse_unknown();
            assert!(matches!(result, Some(QueryAtom::Unknown("abc"))));
        }

        #[test]
        fn parse_unknown_stops_at_or() {
            let mut parser = Parser::new("abc or def");
            let result = parser.parse_unknown();
            assert!(matches!(result, Some(QueryAtom::Unknown("abc"))));
        }

        // ================================================================
        // Tests for remaining edge cases
        // ================================================================

        // Test feature name parsing with terminating special characters
        #[test]
        fn parse_supports_feature_with_comma() {
            // Feature name should stop at comma
            let mut parser = Parser::new("supports flexbox,");
            let result = parser.parse_query_atom();
            assert!(matches!(result, Some(QueryAtom::Supports("flexbox", None))));
        }

        #[test]
        fn parse_supports_feature_with_space() {
            // Feature name stops at space before "and"
            let mut parser = Parser::new("supports flexbox and");
            let result = parser.parse_query_atom();
            assert!(matches!(result, Some(QueryAtom::Supports("flexbox", None))));
        }

        #[test]
        fn parse_fully_supports_feature_terminated() {
            let mut parser = Parser::new("fully supports flexbox,");
            let result = parser.parse_query_atom();
            assert!(matches!(
                result,
                Some(QueryAtom::Supports("flexbox", Some(SupportKind::Fully)))
            ));
        }

        #[test]
        fn parse_partially_supports_feature_terminated() {
            let mut parser = Parser::new("partially supports flexbox,");
            let result = parser.parse_query_atom();
            assert!(matches!(
                result,
                Some(QueryAtom::Supports("flexbox", Some(SupportKind::Partially)))
            ));
        }

        // Test since_or_supports fallback when second char doesn't match
        #[test]
        fn parse_since_or_supports_with_s_third_char() {
            // Starts with 's', second char is 'a' (not 'i' or 'u'), falls to fallback
            let mut parser = Parser::new("sa");
            let result = parser.parse_since_or_supports();
            assert!(result.is_none());
        }

        // Test that "last N chromex" fails when chromex is followed by invalid keyword
        #[test]
        fn parse_last_browser_invalid_suffix() {
            let mut parser = Parser::new("last 2 chromex abc");
            let result = parser.parse_last_or_years();
            // Should fail because "abc" is not "version(s)" or "major version(s)"
            assert!(result.is_none());
        }

        #[test]
        fn parse_since_or_supports_single_char() {
            // Input is just "s" - only 1 char, so we skip the if block
            let mut parser = Parser::new("s");
            let result = parser.parse_since_or_supports();
            assert!(result.is_none());
        }
    }
}
