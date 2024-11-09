use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{
        features::{get_feature_stat, FeatureSet},
        get_browser_stat, to_desktop_name, ArchivedVersionDetail,
    },
    error::Error,
    parser::SupportKind,
    Opts,
};

pub(super) fn supports(name: &str, kind: Option<SupportKind>, opts: &Opts) -> QueryResult {
    let include_partial = matches!(kind, Some(SupportKind::Partially) | None);

    if let Some(feature) = get_feature_stat(name) {
        let distribs = feature
            .iter()
            .filter_map(|(name, versions)| {
                get_browser_stat(name, opts.mobile_to_desktop)
                    .map(|(name, stat)| (name, stat, versions))
            })
            .flat_map(|(name, browser_stat, versions)| {
                let desktop_name =
                    opts.mobile_to_desktop.then_some(to_desktop_name(name)).flatten();
                let check_desktop = desktop_name.is_some()
                    && browser_stat
                        .version_list
                        .iter()
                        .filter(|version| version.release_date.is_some())
                        .last()
                        .is_some_and(|latest_version| {
                            is_supported(versions, latest_version.version.as_str(), include_partial)
                        });
                browser_stat
                    .version_list
                    .iter()
                    .filter_map(move |ArchivedVersionDetail { version, .. }| {
                        if is_supported(versions, version, include_partial) {
                            return Some(version);
                        }
                        if check_desktop {
                            if let Some(desktop_name) = desktop_name {
                                if let Some(versions) = feature.get(desktop_name) {
                                    if is_supported(versions, version, include_partial) {
                                        return Some(version);
                                    }
                                }
                            }
                        }
                        None
                    })
                    .map(move |version| Distrib::new(name, version.as_str()))
            })
            .collect();
        Ok(distribs)
    } else {
        Err(Error::UnknownBrowserFeature(name.to_string()))
    }
}

fn is_supported(set: &FeatureSet, version: &str, include_partial: bool) -> bool {
    set.0.contains(&version) || (include_partial && set.1.contains(&version))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::{
        opts::Opts,
        test::{run_compare, should_failed},
    };

    #[test_case("supports objectrtc"; "case 1")]
    #[test_case("supports    rtcpeerconnection"; "case 2")]
    #[test_case("supports        arrow-functions"; "case 3")]
    #[test_case("partially supports rtcpeerconnection"; "partially")]
    #[test_case("fully     supports rtcpeerconnection"; "fully")]
    fn default_options(query: &str) {
        run_compare(query, &Opts::default(), None);
    }

    #[test_case("supports filesystem"; "case 1")]
    #[test_case("supports  font-smooth"; "case 2")]
    fn mobile_to_desktop(query: &str) {
        run_compare(query, &Opts { mobile_to_desktop: true, ..Default::default() }, None);
    }

    #[test]
    fn invalid() {
        assert_eq!(
            should_failed("supports xxxyyyzzz", &Opts::default()),
            Error::UnknownBrowserFeature(String::from("xxxyyyzzz"))
        );
    }
}
