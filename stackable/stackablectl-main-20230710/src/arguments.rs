use crate::{
    demo::CliCommandDemo, operator::CliCommandOperator, release::CliCommandRelease,
    services::CliCommandServices, stack::CliCommandStack,
};
use clap::{Command, Parser, ValueEnum, ValueHint};
use clap_complete::{generate, Generator, Shell};
use log::LevelFilter;
use std::io;

#[derive(Parser)]
#[command(author, version, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub cmd: CliCommand,

    /// Log level.
    #[arg(short, long, value_enum, default_value = "info")]
    pub log_level: LogLevel,

    /// Namespace where to deploy the products and operators
    #[arg(short, long, default_value = "default", value_hint = ValueHint::Other)]
    pub namespace: String,

    /// Overwrite the URL of the stable helm repo
    ///
    /// If you don't have access to the Stackable Helm repos you can mirror the repo and provide the URL here
    /// (e.g. <https://my.repo/repository/stackable-stable/>).
    #[arg(
        long,
        default_value = "https://repo.stackable.tech/repository/helm-stable",
        value_hint = ValueHint::Url,
    )]
    pub helm_repo_stackable_stable: String,

    /// Overwrite the URL of the test helm repo
    ///
    /// If you don't have access to the Stackable Helm repos you can mirror the repo and provide the URL here
    /// (e.g. <https://my.repo/repository/stackable-test/>).
    #[arg(
        long,
        default_value = "https://repo.stackable.tech/repository/helm-test",
        value_hint = ValueHint::Url,
    )]
    pub helm_repo_stackable_test: String,

    /// Overwrite the URL of the dev helm repo
    ///
    /// If you don't have access to the Stackable Helm repos you can mirror the repo and provide the URL here
    /// (e.g. <https://my.repo/repository/stackable-dev/>).
    #[arg(
        long,
        default_value = "https://repo.stackable.tech/repository/helm-dev",
        value_hint = ValueHint::Url,
    )]
    pub helm_repo_stackable_dev: String,

    /// Adds a YAML file containing custom releases
    ///
    /// If you do not have access to the Stackable repositories on GitHub or if you want to maintain your own releases, you can specify additional YAML files containing release information.
    /// Have a look at <https://raw.githubusercontent.com/stackabletech/release/main/releases.yaml> for the structure.
    /// Can either be a URL or a path to a file, e.g. `https://my.server/my-releases.yaml`, '/etc/my-releases.yaml' or `C:\Users\Bob\my-releases.yaml`.
    /// Can be specified multiple times.
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub additional_releases_file: Vec<String>,

    /// Adds a YAML file containing custom stacks
    ///
    /// If you do not have access to the Stackable repositories on GitHub or if you want to maintain your own stacks, you can specify additional YAML files containing stack information.
    /// Have a look at <https://raw.githubusercontent.com/stackabletech/stackablectl/main/stacks/stacks-v1.yaml> for the structure.
    /// Can either be a URL or a path to a file, e.g. `https://my.server/my-stacks.yaml`, '/etc/my-stacks.yaml' or `C:\Users\Bob\my-stacks.yaml`.
    /// Can be specified multiple times.
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub additional_stacks_file: Vec<String>,

    /// Adds a YAML file containing custom demos
    ///
    /// If you do not have access to the Stackable repositories on GitHub or if you want to maintain your own demos, you can specify additional YAML files containing demo information.
    /// Have a look at <https://raw.githubusercontent.com/stackabletech/stackablectl/main/demos/demos-v1.yaml> for the structure.
    /// Can either be a URL or a path to a file, e.g. `https://my.server/my-demos.yaml`, '/etc/my-demos.yaml' or `C:\Users\Bob\my-demos.yaml`.
    /// Can be specified multiple times.
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub additional_demos_file: Vec<String>,
}

#[derive(Parser)]
pub enum CliCommand {
    /// This command interacts with single operators if you don’t want to install the full platform.
    #[command(subcommand, alias("o"), alias("op"))]
    Operator(CliCommandOperator),

    /// This command interacts with all operators of the platform that are released together.
    #[command(subcommand, alias("r"), alias("re"))]
    Release(CliCommandRelease),

    /// This command interacts with stacks, which are ready-to-use combinations of products.
    #[command(subcommand, alias("s"), alias("st"))]
    Stack(CliCommandStack),

    /// This command interacts with deployed services of products.
    #[command(subcommand, alias("svc"))]
    Services(CliCommandServices),

    /// This command interacts with demos, which are end-to-end demonstrations of the usage of the Stackable data platform.
    #[command(subcommand, alias("d"), alias("de"))]
    Demo(CliCommandDemo),

    /// Output shell completion code for the specified shell.
    Completion(CliCommandCompletion),
}

#[derive(Clone, Parser, ValueEnum)]
pub enum OutputType {
    Text,
    Json,
    Yaml,
}

#[derive(Clone, Copy, Parser, Debug, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(Parser)]
pub struct CliCommandCompletion {
    // Shell to generate the completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[cfg(test)]
mod test {
    use crate::arguments::CliArgs;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CliArgs::command().debug_assert()
    }
}
