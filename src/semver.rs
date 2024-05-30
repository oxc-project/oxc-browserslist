use std::{cmp::Ordering, fmt, num::ParseIntError, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Copy, Clone)]
pub struct Version(pub u32, pub u32, pub u32);

impl Version {
    #[inline]
    pub(crate) fn major(&self) -> u32 {
        self.0
    }
}

impl FromStr for Version {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // this allows something like `4.4.3-4.4.4`
        let mut segments = s.split_once('-').map_or(s, |(v, _)| v).split('.');
        let major = match segments.next() {
            Some(n) => n.parse()?,
            None => 0,
        };
        let minor = match segments.next() {
            Some(n) => n.parse()?,
            None => 0,
        };
        let patch = match segments.next() {
            Some(n) => n.parse()?,
            None => 0,
        };

        Ok(Self(major, minor, patch))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

pub fn loose_compare(a: &Version, b: &str) -> Ordering {
    let mut b = b.split('.');
    let Some(first) = b.next() else {
        return Ordering::Equal;
    };
    let first: u32 = first.parse().unwrap_or_default();
    let x = a.0.cmp(&first);
    if !x.is_eq() {
        return x;
    }
    let Some(second) = b.next() else {
        return Ordering::Equal;
    };
    let first: u32 = second.parse().unwrap_or_default();
    a.1.cmp(&first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version() {
        assert_eq!(Ok(Version(1, 0, 0)), "1".parse());
        assert_eq!(Ok(Version(1, 2, 0)), "1.2".parse());
        assert_eq!(Ok(Version(1, 2, 3)), "1.2.3".parse());
        assert_eq!(Ok(Version(12, 34, 56)), "12.34.56".parse());

        assert_eq!(Ok(Version(1, 0, 0)), "1-2".parse());
        assert_eq!(Ok(Version(1, 2, 0)), "1.2-1.3".parse());
        assert_eq!(Ok(Version(1, 2, 3)), "1.2.3-1.2.4".parse());
        assert_eq!(Ok(Version(12, 34, 56)), "12.34.56-78.9".parse());

        assert!("tp".parse::<Version>().is_err());
    }
}
