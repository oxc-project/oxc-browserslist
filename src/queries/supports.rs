use super::{Distrib, QueryResult};
use crate::{
    Opts,
    data::caniuse::{features::get_feature_stat, get_browser_stat, to_desktop_name},
    error::Error,
    parser::SupportKind,
};

pub(super) fn supports(name: &str, kind: Option<SupportKind>, opts: &Opts) -> QueryResult {
    let include_partial = matches!(kind, Some(SupportKind::Partially) | None);

    if let Some(stat) = get_feature_stat(name) {
        let feature = stat.create_data();
        let feature = feature.as_slice();
        let distribs = feature
            .iter()
            .filter_map(|(name, versions)| {
                get_browser_stat(name, opts.mobile_to_desktop)
                    .map(|(name, stat)| (name, stat, versions))
            })
            .flat_map(|(name, browser_stat, versions)| {
                let desktop_name = opts.mobile_to_desktop.then(|| to_desktop_name(name)).flatten();
                let check_desktop = desktop_name.is_some()
                    && browser_stat
                        .version_list
                        .iter()
                        .filter(|version| version.release_date().is_some())
                        .rfind(|latest_version| {
                            versions
                                .supports(latest_version.version(), /* include_partial */ true)
                        })
                        .is_some_and(|latest_version| {
                            versions.supports(latest_version.version(), include_partial)
                        });

                browser_stat
                    .version_list
                    .iter()
                    .filter_map(move |version_detail| {
                        let version = version_detail.version();
                        if versions.supports(version, include_partial) {
                            return Some(version);
                        }
                        if check_desktop {
                            if let Some(desktop_name) = desktop_name {
                                if let Some(versions) =
                                    feature.iter().find_map(|(name, versions)| {
                                        (*name == desktop_name).then_some(versions)
                                    })
                                {
                                    if versions.supports(version, include_partial) {
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
    } else {
        Err(Error::UnknownBrowserFeature(name.to_string()))
    }
}
