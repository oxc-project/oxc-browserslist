use browserslist::{resolve, Opts};
use pico_args::Arguments;

fn main() {
    let mut args = Arguments::from_env();
    let mobile_to_desktop = args.contains("--mobile-to-desktop");
    let ignore_unknown_versions = args.contains("--ignore-unknown-versions");
    let queries = args
        .finish()
        .into_iter()
        .filter_map(|s| s.to_str().map(ToString::to_string))
        .collect::<Vec<_>>();

    match resolve(
        &queries,
        &Opts { mobile_to_desktop, ignore_unknown_versions, ..Default::default() },
    ) {
        Ok(versions) => {
            for version in versions {
                println!("{version}");
            }
        }
        Err(error) => eprintln!("{error}"),
    };
}
