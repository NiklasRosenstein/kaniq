static KANIKO_IMAGE: &'static str = "ghcr.io/niklasrosenstein/kaniq:latest";

/// Run a Kaniko container, mounting the current directory as the workspace folder.
/// Additionally, existing environment variables can be convienently re-exported
/// into the container, but remember that they will still need to be fed into the
/// Kaniko executor as build args or secrets.
///
/// This is the only Kaniku command that is expected to be called outside of a
/// Kaniko container.
#[derive(clap::Parser)]
#[clap(setting = clap::AppSettings::TrailingVarArg)]
pub struct RunArgs {
    /// Specify an environment variable in the Kaniko container. If only the variable
    /// name is specified and no value is assigned (as indicated by the presence of
    /// an equal sign followed by 0 or more characters), the variable is read from your
    /// current environment. It is an error if the variable does not exist in your
    /// environment.
    #[clap(long)]
    env: Vec<String>,

    /// The Kaniko image to use.
    #[clap(long, default_value = KANIKO_IMAGE)]
    image: String,

    /// The command to run inside the Kaniko container. Often this is just a call to
    /// "/kaniko/kaniku execute" to kick off the Kaniko executor.
    argv: Vec<String>,
}

pub fn run(args: RunArgs) {
    let env_args: Vec<String> = args
        .env
        .iter()
        .flat_map(|env| -> Vec<String> {
            match env.split_once("=") {
                Some((_, _)) => vec!["--env".to_string(), env.clone()],
                None => vec![
                    "--env".to_string(),
                    format!(
                        "{}={}",
                        env.clone(),
                        std::env::var(env)
                            .expect(format!("env variable {} is not set", env).as_str())
                    ),
                ],
            }
        })
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
    command
        .spawn()
        .expect("failed to spawn docker command")
        .wait()
        .expect("kaniko container failed");
}
