use super::{Distrib, QueryResult};
use crate::{
    data::caniuse::{
        features::{get_feature_stat, FeatureSet},
        get_browser_stat, to_desktop_name,
    },
    error::Error,
    parser::SupportKind,
    Opts,
};

pub(super) fn supports(name: &str, kind: Option<SupportKind>, opts: &Opts) -> QueryResult {
    let Some(features) = get_feature_stat(name) else {
        return Err(Error::UnknownBrowserFeature(name.to_string()));
    };

    let include_partial = matches!(kind, Some(SupportKind::Partially) | None);

    // let mut result = vec![];
    // for (name, versions) in features {
    // let Some((browser_name, browser_stat)) = get_browser_stat(name, opts.mobile_to_desktop)
    // else {
    // continue;
    // };

    // let desktop_name =
    // if opts.mobile_to_desktop { to_desktop_name(browser_name) } else { None };

    // let latest_version = browser_stat
    // .version_list
    // .iter()
    // // .filter(|v| v.release_date.is_some())
    // .rfind(|v| !(versions.0.contains(v.version) || versions.1.contains(v.version)));

    // let check_desktop = desktop_name.is_some()
    // && latest_version.is_some_and(|latest_version| {
    // is_supported(versions, latest_version.version, include_partial)
    // });
    // dbg!(check_desktop)

    // for version_detail in &browser_stat.version_list {
    // let version = version_detail.version;
    // if is_supported(versions, version, include_partial) {
    // result.push(Distrib::new(name, version));
    // }
    // if check_desktop {
    // if let Some(desktop_name) = desktop_name {
    // if let Some(versions) = features.get(desktop_name) {
    // if is_supported(versions, version, include_partial) {
    // result.push(Distrib::new(name, version));
    // }
    // }
    // }
    // }
    // }
    // }

    // Ok(result)
    // // return Err(Error::UnknownBrowserFeature(name.to_string()));
    let distribs = features
        .iter()
        .filter_map(|(name, versions)| {
            get_browser_stat(name, opts.mobile_to_desktop)
                .map(|(name, stat)| (name, stat, versions))
        })
        .flat_map(|(name, browser_stat, versions)| {
            let desktop_name = to_desktop_name(name);
            let check_desktop = if opts.mobile_to_desktop {
                desktop_name.is_some()
                    && browser_stat
                        .version_list
                        .iter()
                        .rev()
                        .filter(|v| {
                            v.release_date.is_some()
                                && (versions.0.contains(v.version)
                                    || versions.1.contains(v.version))
                        })
                        .skip(1)
                        .next()
                        .is_some_and(|latest_version| {
                            is_supported(versions, latest_version.version, include_partial)
                        })
            } else {
                false
            };
            browser_stat
                .version_list
                .iter()
                .filter_map(move |version_detail| {
                    let version = version_detail.version;
                    if is_supported(versions, version, include_partial) {
                        return Some(version);
                    }
                    if check_desktop {
                        if let Some(desktop_name) = desktop_name {
                            if let Some(versions) = features.get(desktop_name) {
                                if is_supported(versions, version, include_partial) {
                                    return Some(version);
                                }
                            }
                        }
                    }
                    None
                })
                .map(move |version| Distrib::new(name, version))
        })
        .collect();
    Ok(distribs)
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
    #[test_case("supports clipboard"; "clipboard")]
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
