static KANIKO_IMAGE: &'static str = "ghcr.io/niklasrosenstein/kaniq:latest";

/// Run a Kaniko container, mounting the current directory as the workspace folder.
/// Additionally, existing environment variables can be convienently re-exported
/// into the container, but remember that they will still need to be fed into the
/// Kaniko executor as build args or secrets.
///
/// This is the only kaniq command that is expected to be called outside of a
/// Kaniko container.
#[derive(clap::Parser)]
#[clap(setting = clap::AppSettings::TrailingVarArg)]
pub struct RunArgs {
    /// Export one or more environment variables to the Kaniko container. The value for this
    /// option can take three forms: 1) `KEY=VALUE`, 2) `KEY`, 3) `KEY1,KEY2,KEY3`. In the
    /// first form, the variable and value are passed as is. In the second form, the value
    /// for environment variable `KEY` is looked up in your current environment. The third
    /// form is functionally similar than the second, but can accept multiple variable names.
    #[clap(long)]
    env: Vec<String>,

    /// The Docker image to use.
    #[clap(long, default_value = KANIKO_IMAGE)]
    image: String,

    /// Enable verbose output.
    #[clap(long, short)]
    verbose: bool,

    /// The command to run inside the Kaniko container. Usually you would pass a script that
    /// sets up the authentication and kicks off the Kaniko executor via `kaniq execute`.
    argv: Vec<String>,
}

pub fn parse_env_args(args: Vec<String>) -> std::collections::HashMap<String, String> {
    args.iter()
        .flat_map(|env| {
            match env.split_once("=") {
                // Normal key=value pair.
                Some((k, v)) => vec![(k.to_string(), v.to_string())],
                // One or more variable names to export from the current environment.
                None => env
                    .split(',')
                    .skip_while(|x| x.is_empty())
                    .flat_map(|key| {
                        vec![(
                            key.to_string(),
                            std::env::var(key)
                                .expect(format!("env variable '{}' is not set", key).as_str()),
                        )]
                    })
                    .collect(),
            }
        })
        .collect()
}

pub fn run(args: RunArgs) {
    let env_args: Vec<String> = parse_env_args(args.env)
        .iter()
        .flat_map(|(key, value)| vec!["--env".to_string(), format!("{}={}", key, value)])
        .collect();
    let pwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut command = std::process::Command::new("docker");
    command.arg("run");
    command.args(env_args);
    command.args([
        "--rm",
        "-v",
        format!("{}:/workspace", pwd).as_str(),
        "--entrypoint",
        "",
        args.image.as_str(),
    ]);
    command.args(args.argv);
    if args.verbose {
        println!("[kaniq run    ] executing command {:?}", command)
    }
    command
        .spawn()
        .expect("failed to spawn docker command")
        .wait()
        .expect("kaniko container failed");
}
