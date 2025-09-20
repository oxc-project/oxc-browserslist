use super::{Distrib, QueryResult};

pub(super) fn op_mini() -> QueryResult {
    Ok(vec![Distrib::new("op_mini", "all")])
}

#[cfg(all(test, not(miri)))]
mod tests {
    use test_case::test_case;

    use crate::{opts::Opts, test::run_compare};

    #[test_case("op_mini all"; "short")]
    #[test_case("Op_Mini All"; "short case insensitive")]
    #[test_case("operamini all"; "long")]
    #[test_case("OperaMini All"; "long case insensitive")]
    #[test_case("op_mini    all"; "more spaces")]
    fn valid(query: &str) {
        run_compare(query, &Opts::default(), None);
    }
}
