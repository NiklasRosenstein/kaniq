static KANIKO_SECRETS_DIR: &'static str = "/kaniko/secrets";
static KANIKO_EXECUTOR: &'static str = "/kaniko/executor";

/// A wrapper around /kaniko/executor that allows you to configure secrets that can be
/// read from the /kaniko/secrets folder.
#[derive(clap::Parser)]
#[clap(setting = clap::AppSettings::TrailingVarArg)]
pub struct ExecuteArgs {
    /// Configure a secret that can be read from the /kaniko/secrets folder. The option value
    /// must be formatted as 1) `SECRET_NAME=VALUE`, 2) `SECRET`, 3) `SECRET1,SECRET2,SECRET3`.
    /// The behaviour is identical to that of the `kaniq run --env` option.
    #[clap(long)]
    secret: Vec<String>,

    /// Forwards the build arguments to `/kaniko/executor`, but similar to `--secret`, this
    /// option accepts multiple environment variables names separated by commas to export them
    /// all as build arguments.
    #[clap(long)]
    build_arg: Vec<String>,

    /// Set up credentials for a Docker registry. This is a shorthand to using the `kaniq auth` command.
    /// The argument can be specified multiple times.
    #[clap(long, number_of_values = 3, multiple = true)]
    auth: Vec<String>,

    /// Enable verbose output.
    #[clap(long, short)]
    verbose: bool,

    /// Remaining args for the executor.
    argv: Vec<String>,
}

pub fn run(args: ExecuteArgs) -> i32 {
    crate::auth::run(
        args.auth
            .chunks(3)
            .map(|auth| crate::auth::AuthArgs {
                registry: auth[0].clone(),
                username: auth[1].clone(),
                password: auth[2].clone(),
            })
            .collect(),
    );
    if !std::path::Path::new(KANIKO_SECRETS_DIR).is_dir() {
        std::fs::create_dir(KANIKO_SECRETS_DIR)
            .expect(format!("could not create directory {}", KANIKO_SECRETS_DIR).as_str());
    }
    crate::run::parse_env_args(args.secret)
        .iter()
        .for_each(|(key, value)| {
            let secret_path = std::path::Path::new(KANIKO_SECRETS_DIR).join(key);
            println!("[kaniq execute] create secret {:?}", secret_path);
            std::fs::write(secret_path, value).unwrap();
        });
    let mut command = std::process::Command::new(KANIKO_EXECUTOR);
    command.args(args.argv);
    crate::run::parse_env_args(args.build_arg)
        .iter()
        .for_each(|(key, value)| {
            command.args(vec!["--env".to_string(), format!("{}={}", key, value)]);
        });
    if args.verbose {
        println!("[kaniq execute] executing command {:?}", command);
    }
    command.spawn().unwrap().wait().unwrap().code().unwrap()
}
