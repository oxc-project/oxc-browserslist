#[cfg(test)]
use crate::{resolve, Opts};

#[test]
fn test_basic_query_functionality() {
    let distribs = resolve(&["ie >= 8"], &Opts::default()).unwrap();
    assert!(!distribs.is_empty());
    assert!(distribs.iter().any(|d| d.name() == "ie" && d.version() == "11"));
    assert!(distribs.iter().any(|d| d.name() == "ie" && d.version() == "10"));
    assert!(distribs.iter().any(|d| d.name() == "ie" && d.version() == "9"));
    assert!(distribs.iter().any(|d| d.name() == "ie" && d.version() == "8"));
    
    // Should not include ie 7
    assert!(!distribs.iter().any(|d| d.name() == "ie" && d.version() == "7"));
}

#[test] 
fn test_last_versions_query() {
    let distribs = resolve(&["last 2 versions"], &Opts::default()).unwrap();
    assert!(!distribs.is_empty());
    
    // Should include modern browsers
    assert!(distribs.iter().any(|d| d.name() == "chrome"));
    assert!(distribs.iter().any(|d| d.name() == "firefox"));
}

#[test]
fn test_percentage_query() {
    let distribs = resolve(&["> 1%"], &Opts::default()).unwrap();
    assert!(!distribs.is_empty());
    
    // Should include popular browsers
    assert!(distribs.iter().any(|d| d.name() == "chrome"));
}