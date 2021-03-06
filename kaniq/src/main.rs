use clap::{Parser, Subcommand};

mod auth;
mod execute;
mod run;

/// Kaniq is a CLI and Docker image to simplify builds with Google's Kaniko.
/// For more information, visit https://github.com/NiklasRosenstein/kaniq.
#[derive(Parser)]
#[clap(author, version, about, long_about = None, setting = clap::AppSettings::DisableHelpSubcommand)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Auth(auth::AuthArgs),
    Execute(execute::ExecuteArgs),
    Run(run::RunArgs),
}

fn main() {
    match Args::parse().action {
        Action::Auth(args) => {
            std::process::exit(auth::run(vec![args]));
        }
        Action::Execute(args) => {
            std::process::exit(execute::run(args));
        }
        Action::Run(args) => {
            std::process::exit(run::run(args));
        }
    }
}
