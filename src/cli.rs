use clap::{Arg, Command};
use std::ffi::OsStr;

pub fn new(current_dir: &OsStr) -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .disable_help_subcommand(true)
        .propagate_version(true)
        .mut_arg("version", |a| a.short('v'))
        .arg(no_color_flag())
        .arg(quiet_flag())
        .subcommands([check_command(current_dir), compare_command(), fix_command(current_dir), list_command()])
}

fn check_command<'a>(current_dir: &OsStr) -> Command {
    Command::new("check")
        .args(common_args(current_dir))
        .about("Checks files for issues")
}

fn fix_command(current_dir: &OsStr) -> Command {
    Command::new("fix")
        .args(common_args(current_dir))
        .arg(
            Arg::new("no-backup")
                .long("no-backup")
                .help("Prevents backing up .env files"),
        )
        .override_usage("dotenv-linter fix [OPTIONS] <input>...")
        .about("Automatically fixes warnings")
}

fn compare_command<'a>() -> Command<'a> {
    Command::new("compare")
        .arg(
            Arg::new("input")
                .help("Files to compare")
                .multiple_occurrences(true)
                .multiple_values(true)
                .min_values(2)
                .required(true),
        )
        .about("Compares if files have the same keys")
        .override_usage("dotenv-linter compare [OPTIONS] <input>...")
}

fn list_command<'a>() -> Command<'a> {
    Command::new("list")
        .override_usage("dotenv-linter list")
        .about("Shows list of available checks")
}

fn common_args(current_dir: &OsStr) -> Vec<Arg> {
    vec![
        Arg::new("input")
            .help("files or paths")
            .index(1)
            .default_value_os(current_dir)
            .multiple_occurrences(true)
            .multiple_values(true),
        Arg::new("exclude")
            .short('e')
            .long("exclude")
            .value_name("FILE_NAME")
            .help("Excludes files from check")
            .multiple_occurrences(true)
            .multiple_values(true)
            .takes_value(true),
        Arg::new("skip")
            .short('s')
            .long("skip")
            .value_name("CHECK_NAME")
            .help("Skips checks")
            .multiple_occurrences(true)
            .multiple_values(true)
            .takes_value(true),
        Arg::new("recursive")
            .short('r')
            .long("recursive")
            .help("Recursively searches and checks .env files"),
    ]
}

fn quiet_flag<'a>() -> Arg<'a> {
    Arg::new("quiet")
        .short('q')
        .long("quiet")
        .help("Doesn't display additional information")
        .global(true)
}

fn no_color_flag<'a>() -> Arg<'a> {
    Arg::new("no-color")
        .long("no-color")
        .env("NO_COLOR") // TODO: add a test for this
        .help("Turns off the colored output")
        .global(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn verify_app() {
        let current_dir = env::current_dir().expect("Failed to get current dir");
        new(current_dir.as_os_str()).debug_assert();
    }
}
