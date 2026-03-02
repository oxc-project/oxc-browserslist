use super::{Distrib, QueryResult};
use crate::{data::caniuse::region::get_usage_by_region, error::Error};

pub(super) fn cover_by_region(coverage: f32, region: &str) -> QueryResult {
    let normalized_region =
        if region.len() == 2 { region.to_uppercase() } else { region.to_lowercase() };

    if let Some(region_data) = get_usage_by_region(&normalized_region) {
        let mut distribs = vec![];
        let mut total = 0.0;
        for (name, version, usage) in region_data.iter() {
            if total >= coverage || usage == 0.0 {
                break;
            }
            distribs.push(Distrib::new(name, version));
            total += usage;
        }
        Ok(distribs)
    } else {
        Err(Error::UnknownRegion(region.to_string()))
    }
}
