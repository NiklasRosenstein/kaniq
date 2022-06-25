// The path to the Docker configuration file that Kaniko will read to look up
// credentials for Docker registries.
const KANIKO_DOCKER_CONFIG_FILE: &'static str = "/kaniko/.docker/config.json";

/// Add or overwrite credentials for a Docker registry to Kaniko's Docker configuration.
#[derive(clap::Parser)]
pub struct AuthArgs {
    /// Docker registry to configure credentials for. If the specified value contains a slash,
    /// only the first part of the path is used, assuming it is a hostname.
    registry: String,

    /// The username for the registry.
    username: String,

    /// The user's password to authenticate registry.
    password: String,
}

pub fn run(args: AuthArgs) {
    let mut data = json::parse(
        std::fs::read(KANIKO_DOCKER_CONFIG_FILE)
            .map(|d| String::from_utf8(d).unwrap())
            .unwrap_or("{}".to_string())
            .as_str(),
    )
    .unwrap();

    let mut registry = args.registry.clone();
    if registry.contains("/") {
        registry = String::from(registry.split_once("/").unwrap().0);
        println!(
            "note: using registry name \"{}\" from full argument \"{}\"",
            registry, args.registry
        );
    }

    data[registry] = json::object! {
        auth: base64::encode(format!("{}:{}", args.username, args.password.as_str())),
    };

    std::fs::write(KANIKO_DOCKER_CONFIG_FILE, data.dump())
        .expect(format!("unable to write to file {}", KANIKO_DOCKER_CONFIG_FILE).as_str());
}
