use super::{Distrib, QueryResult};

pub(super) fn firefox_esr() -> QueryResult {
    Ok(vec![Distrib::new("firefox", "140")])
}
