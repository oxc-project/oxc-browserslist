use super::{Distrib, QueryResult};
use crate::data::caniuse::CANIUSE_GLOBAL_USAGE;

pub(super) fn cover(coverage: f32) -> QueryResult {
    let mut distribs = vec![];
    let mut total = 0.0;
    for (name, version, usage) in CANIUSE_GLOBAL_USAGE {
        if total >= coverage || *usage == 0.0 {
            break;
        }
        distribs.push(Distrib::new(name, *version));
        total += usage;
    }
    Ok(distribs)
}

#[cfg(test)]
mod tests {
    use crate::{opts::Opts, test::run_compare};
    use test_case::test_case;

    #[test_case("cover 0.1%"; "global")]
    #[test_case("Cover 0.1%"; "global case insensitive")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
