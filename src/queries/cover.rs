use super::{Distrib, QueryResult};

use crate::data::caniuse::caniuse_global_usage;

pub(super) fn cover(coverage: f32) -> QueryResult {
    let mut distribs = vec![];
    let mut total = 0.0;
    for (name, version, usage) in caniuse_global_usage().iter() {
        if total >= coverage || *usage == 0.0 {
            break;
        }
        distribs.push(Distrib::new(name, version.as_str()));
        total += usage;
    }
    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{opts::Opts, test::run_compare};

    #[test_case("cover 0.1%"; "global")]
    #[test_case("Cover 0.1%"; "global case insensitive")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
