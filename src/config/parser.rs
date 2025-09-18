use rustc_hash::FxHashSet;

use super::PartialConfig;
use crate::error::Error;

pub fn parse(source: &str, env: &str, throw_on_missing: bool) -> Result<PartialConfig, Error> {
    let mut encountered_sections = FxHashSet::default();
    let mut current_section = Some("defaults");
    let mut defaults_queries = Vec::new();
    let mut env_queries: Option<Vec<String>> = None;

    // Process lines efficiently in a single loop
    for line in source.lines() {
        // Remove comments and trim in one step
        let line =
            if let Some(index) = line.find('#') { line[..index].trim() } else { line.trim() };

        if line.is_empty() {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            // Parse section header inline
            let sections: Vec<&str> = line[1..line.len() - 1].split_whitespace().collect();

            // Check for duplicates and collect into owned strings
            for section in &sections {
                if encountered_sections.contains(*section) {
                    return Err(Error::DuplicatedSection(section.to_string()));
                }
                encountered_sections.insert(section.to_string());
            }

            // Update current section
            current_section = sections.iter().find(|&&s| s == env).copied();

            // Initialize env queries if needed
            if env_queries.is_none() && encountered_sections.contains(env) {
                env_queries = Some(Vec::new());
            }
        } else if current_section.is_some() {
            // Add query to appropriate collection
            if let Some(ref mut env_queries) = env_queries {
                env_queries.push(line.to_string());
            } else {
                defaults_queries.push(line.to_string());
            }
        }
    }

    // Validate environment requirement
    if throw_on_missing && env != "defaults" && !encountered_sections.contains(env) {
        return Err(Error::MissingEnv(env.to_string()));
    }

    Ok(PartialConfig { defaults: defaults_queries, env: env_queries })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let source = "  \t  \n  \r\n  # comment ";
        let config = parse(source, "production", false).unwrap();
        assert!(config.defaults.is_empty());
        assert!(config.env.is_none());
    }

    #[test]
    fn no_sections() {
        let source = r"
last 2 versions
not dead
";
        let config = parse(source, "production", false).unwrap();
        assert_eq!(&*config.defaults, ["last 2 versions", "not dead"]);
        assert!(config.env.is_none());
    }

    #[test]
    fn single_line() {
        let source = r"last 2 versions, not dead";
        let config = parse(source, "production", false).unwrap();
        assert_eq!(&*config.defaults, ["last 2 versions, not dead"]);
        assert!(config.env.is_none());
    }

    #[test]
    fn empty_lines() {
        let source = r"
last 2 versions


not dead
";
        let config = parse(source, "production", false).unwrap();
        assert_eq!(&*config.defaults, ["last 2 versions", "not dead"]);
        assert!(config.env.is_none());
    }

    #[test]
    fn comments() {
        let source = r"
last 2 versions  #trailing comment
#line comment
not dead
";
        let config = parse(source, "production", false).unwrap();
        assert_eq!(&*config.defaults, ["last 2 versions", "not dead"]);
        assert!(config.env.is_none());
    }

    #[test]
    fn spaces() {
        let source = "    last 2 versions     \n  not dead    ";
        let config = parse(source, "production", false).unwrap();
        assert_eq!(&*config.defaults, ["last 2 versions", "not dead"]);
        assert!(config.env.is_none());
    }

    #[test]
    fn one_section() {
        let source = r"
[production]
last 2 versions
not dead
";
        let config = parse(source, "production", false).unwrap();
        assert!(config.defaults.is_empty());
        assert_eq!(config.env.as_deref().unwrap(), ["last 2 versions", "not dead"]);
    }

    #[test]
    fn defaults_and_env_mixed() {
        let source = r"
> 1%

[production]
last 2 versions
not dead
";
        let config = parse(source, "production", false).unwrap();
        assert_eq!(&*config.defaults, ["> 1%"]);
        assert_eq!(config.env.as_deref().unwrap(), ["last 2 versions", "not dead"]);
    }

    #[test]
    fn multi_sections() {
        let source = r"
[production]
> 1%
ie 10

[  modern]
last 1 chrome version
last 1 firefox version

[ssr  ]
node 12
";
        let config = parse(source, "production", false).unwrap();
        assert!(config.defaults.is_empty());
        assert_eq!(config.env.as_deref().unwrap(), ["> 1%", "ie 10"]);

        let config = parse(source, "modern", false).unwrap();
        assert!(config.defaults.is_empty());
        assert_eq!(
            config.env.as_deref().unwrap(),
            ["last 1 chrome version", "last 1 firefox version"]
        );

        let config = parse(source, "ssr", false).unwrap();
        assert!(config.defaults.is_empty());
        assert_eq!(config.env.as_deref().unwrap(), ["node 12"]);
    }

    #[test]
    fn shared_multi_sections() {
        let source = r"
[production   development]
> 1%
ie 10
";
        let config = parse(source, "development", false).unwrap();
        assert!(config.defaults.is_empty());
        assert_eq!(config.env.as_deref().unwrap(), ["> 1%", "ie 10"]);
    }

    #[test]
    fn duplicated_sections() {
        let source = r"
[production production]
> 1%
ie 10
";
        assert_eq!(
            parse(source, "testing", false),
            Err(Error::DuplicatedSection("production".into()))
        );

        let source = r"
[development]
last 1 chrome version

[production]
> 1 %
not dead

[development]
last 1 firefox version
";
        assert_eq!(
            parse(source, "testing", false),
            Err(Error::DuplicatedSection("development".into()))
        );
    }

    #[test]
    fn mismatch_section() {
        let source = r"
[production]
> 1%
ie 10
";
        let config = parse(source, "development", false).unwrap();
        assert!(config.defaults.is_empty());
        assert!(config.env.is_none());
    }

    #[test]
    fn throw_on_missing_env() {
        let source = "node 16";
        let err = parse(source, "SSR", true).unwrap_err();
        assert_eq!(err, Error::MissingEnv("SSR".into()));
    }

    #[test]
    fn dont_throw_if_existed() {
        let source = r"
[production]
> 1%
ie 10
";
        let config = parse(source, "production", true).unwrap();
        assert!(config.defaults.is_empty());
        assert!(config.env.is_some());
    }

    #[test]
    fn dont_throw_for_defaults() {
        let source = r"
[production]
> 1%
ie 10
";
        let config = parse(source, "defaults", true).unwrap();
        assert!(config.defaults.is_empty());
        assert!(config.env.is_none());
    }
}
