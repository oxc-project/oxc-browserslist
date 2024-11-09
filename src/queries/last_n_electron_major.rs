use super::{Distrib, QueryResult};
use crate::data::electron::get_electron_versions;

pub(super) fn last_n_electron_major(count: usize) -> QueryResult {
    let minimum = get_electron_versions()
        .iter()
        .rev()
        .nth(count - 1)
        .map(|(electron_version, _)| electron_version.clone())
        .unwrap_or_default();

    let distribs = get_electron_versions()
        .iter()
        .filter(|(electron_version, _)| *electron_version >= minimum)
        .rev()
        .map(|(_, chromium_version)| Distrib::new("chrome", chromium_version.as_str()))
        .collect();

    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{opts::Opts, test::run_compare};

    #[test_case("last 2 electron major versions"; "basic")]
    #[test_case("last 2 Electron major versions"; "case insensitive")]
    #[test_case("last 2 electron major version"; "support pluralization")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
