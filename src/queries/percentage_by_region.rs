use super::{Distrib, QueryResult};
use crate::{data::caniuse::region::get_usage_by_region, error::Error, parser::Comparator};

pub(super) fn percentage_by_region(
    comparator: Comparator,
    popularity: f32,
    region: &str,
) -> QueryResult {
    let normalized_region =
        if region.len() == 2 { region.to_uppercase() } else { region.to_lowercase() };

    if let Some(region_data) = get_usage_by_region(&normalized_region) {
        let distribs = region_data
            .iter()
            .filter(|(_, _, usage)| match comparator {
                Comparator::Greater => *usage > popularity,
                Comparator::Less => *usage < popularity,
                Comparator::GreaterOrEqual => *usage >= popularity,
                Comparator::LessOrEqual => *usage <= popularity,
            })
            .map(|(name, version, _)| Distrib::new(name, version))
            .collect();
        Ok(distribs)
    } else {
        Err(Error::UnknownRegion(region.to_string()))
    }
}
