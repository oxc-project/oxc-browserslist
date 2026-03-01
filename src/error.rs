use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
/// The errors may occur when querying with browserslist.
pub enum Error {
    #[error("failed to parse the rest of input: ...'{0}'")]
    /// Error of parsing query.
    Parse(String),

    #[error("invalid date: {0}")]
    /// Date format is invalid.
    InvalidDate(String),

    #[error("query cannot start with 'not'; add any other queries before '{0}'")]
    /// Query can't start with a negated query which starts with `not`.
    NotAtFirst(String),

    #[error("unknown browser: '{0}'")]
    /// The given browser name can't be found.
    BrowserNotFound(String),

    #[error("unknown Electron version: {0}")]
    /// The given Electron version can't be found.
    UnknownElectronVersion(String),

    #[error("unknown Node.js version: {0}")]
    /// The given Node.js version can't be found.
    UnknownNodejsVersion(String),

    #[error("unknown version '{1}' of browser '{0}'")]
    /// The given version of the given browser can't be found.
    UnknownBrowserVersion(String, String),

    #[error("current environment for querying `current node` is not supported")]
    /// Current environment doesn't support querying `current node`,
    /// for example, running this library on Non-Node.js platform or
    /// no Node.js installed.
    UnsupportedCurrentNode,

    #[error("unknown browser feature: '{0}'")]
    /// Unknown browser feature.
    UnknownBrowserFeature(String),

    #[error("unknown region: '{0}'")]
    /// Unknown Can I Use region.
    UnknownRegion(String),

    #[error("unknown browser query: '{0}'")]
    /// Query can't be recognized.
    UnknownQuery(String),

    #[error("year overflow")]
    /// Year overflow.
    YearOverflow,
}

impl<'a> From<&'a str> for Error {
    fn from(input: &'a str) -> Self {
        Self::Parse(input.to_owned())
    }
}
