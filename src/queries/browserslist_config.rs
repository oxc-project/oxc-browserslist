use super::QueryResult;
use crate::opts::Opts;

pub(super) fn browserslist_config(opts: &Opts) -> QueryResult {
    #[cfg(target_arch = "wasm32")]
    {
        crate::resolve(&["defaults"], opts)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::execute(opts)
    }
}
