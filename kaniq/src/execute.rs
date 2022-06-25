static KANIKO_SECRETS_DIR: &'static str = "/kaniko/secrets";
static KANIKO_EXECUTOR: &'static str = "/kaniko/executor";

/// A wrapper around /kaniko/executor that allows you to configure secrets that can be
/// read from the /kaniko/secrets folder.
#[derive(clap::Parser)]
#[clap(setting = clap::AppSettings::TrailingVarArg)]
pub struct ExecuteArgs {
    /// Configure a secret that can be read from the /kaniko/secrets folder. The option
    /// value must be formatted as SECRET_NAME=VALUE.
    #[clap(long)]
    secret: Vec<String>,

    /// Remaining args for the executor.
    argv: Vec<String>,
}

pub fn run(args: ExecuteArgs) {
    if !std::path::Path::new(KANIKO_SECRETS_DIR).is_dir() {
        std::fs::create_dir(KANIKO_SECRETS_DIR)
            .expect(format!("could not create directory {}", KANIKO_SECRETS_DIR).as_str());
    }
    args.secret
        .iter()
        .for_each(|secret| match secret.split_once("=") {
            None => panic!("bad value of --secret option: {}", secret),
            Some((key, value)) => {
                let secret_path = std::path::Path::new(KANIKO_SECRETS_DIR).join(key);
                println!("create secret {:?}", secret_path);
                std::fs::write(secret_path, value).unwrap();
            }
        });
    let mut command = std::process::Command::new(KANIKO_EXECUTOR);
    command.args(args.argv);
    command
        .spawn()
        .expect("failed to spawn kaniko executor")
        .wait()
        .expect("kaniko executor failed");
}
