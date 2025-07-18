use super::{Distrib, QueryResult, count_filter_versions};
use crate::{data::caniuse::get_browser_stat, error::Error, opts::Opts};

pub(super) fn last_n_x_major_browsers(count: usize, name: &str, opts: &Opts) -> QueryResult {
    let (name, stat) = get_browser_stat(name, opts.mobile_to_desktop)
        .ok_or_else(|| Error::BrowserNotFound(name.to_string()))?;
    let count = count_filter_versions(name, opts.mobile_to_desktop, count);
    let mut vec = stat
        .version_list
        .iter()
        .filter(|version| version.release_date().is_some())
        .map(|version| version.version())
        .rev()
        .map(|version| version.split('.').next().unwrap())
        .collect::<Vec<_>>();
    vec.dedup();
    let minimum = vec.get(count - 1).and_then(|minimum| minimum.parse().ok()).unwrap_or(0);

    let distribs = stat
        .version_list
        .iter()
        .filter(|version| version.release_date().is_some())
        .map(|version| version.version())
        .filter(move |version| version.split('.').next().unwrap().parse().unwrap_or(0) >= minimum)
        .rev()
        .map(move |version| Distrib::new(name, version))
        .collect();

    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::test::run_compare;

    #[test_case("last 2 edge major versions"; "basic")]
    #[test_case("last 1 bb major version"; "support pluralization")]
    #[test_case("last 3 Chrome major versions"; "case insensitive")]
    #[test_case("last 2 android major versions"; "android")]
    #[test_case("last 2 bb major versions"; "non-sequential version numbers")]
    #[test_case("last 3 bb major versions"; "more versions than have been released")]
    fn default_options(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test]
    fn mobile_to_desktop() {
        run_compare(
            "last 2 android major versions",
            &Opts { mobile_to_desktop: true, ..Default::default() },
            None,
        );
    }
}
