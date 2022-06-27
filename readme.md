# kaniq

Kaniq <small>[/ ˈkɑnikju: /]</small> is intended to simplify builds with Google's [Kaniko]. It provides the `kaniq`
CLI as a standalone executable for use on your local machine as well as from inside a Kaniko container. Prebuilt
images that embedd the Kaniq CLI are provided at `gcr.io/niklasrosenstein/kaniq`.

  [Kaniko]: https://github.com/GoogleContainerTools/kaniko

__Features__

* Invoke a local Kaniko build without having to remember the long Docker command-line.
* Easily configure Kaniko Docker registry authentication
* Adds support for secrets

__Build status__

<table>
    <tr>
        <td>Apple Silicon</th>
        <td>
          <a href="https://app.bitrise.io/app/150d483f5a30bb14"><img src="https://app.bitrise.io/app/150d483f5a30bb14/status.svg?token=fdwUoFfzB8XHHaA_04cmDA&branch=master"></a>
        </td>
    <tr>
    </tr>
        <td>Linux amd64, macOS Intel</td>
        <td>
          <a href="https://github.com/NiklasRosenstein/kaniq/actions/workflows/rust.yml"><img src="https://github.com/NiklasRosenstein/kaniq/actions/workflows/rust.yml/badge.svg"></a>
        </td>
    <tr>
    </tr>
        <td>Docker</td>
        <td>
          <a href="https://github.com/NiklasRosenstein/kaniq/actions/workflows/docker-image.yml"><src img="https://github.com/NiklasRosenstein/kaniq/actions/workflows/docker-image.yml/badge.svg"></a>
        </td>
    </tr>

</table>

## Quickstart

Download the Kaniq CLI:

```
$ curl https://github.com/NiklasRosenstein/kaniq/releases/0.1.0/kaniq_darwin_arm64 -o kaniq
```

Define a script that serves as the entrypoint to kick off the Kaniko build:

```
$ cat << EOF > kaniko-entrypoint.sh
#!/bin/sh

/kaniko/kaniq execute \
    --auth my.jfrog.io $ARTIFACTORY_USER $ARTIFACTORY_PASSWORD \
    --secret ARTIFACTORY_USER,ARTIFACTORY_PASSWORD \
    --cache=true --cache-copy-layers \
    --destination my.jfrog.io/docker/my-project:latest

EOF
$ chmod +x kaniko-entrypoint.sh
```

Kick off the Kaniko build locally:

```
$ export ARTIFACTORY_USER=... ARTIFACTORY_PASSWORD=...
# Run the Kaniko build, including the two environment variables for your entrypoint script.
$ kaniq run --env ARTIFACTORY_USER,ARTIFACTORY_PASSWORD kaniko-entrypoint.sh 
```

Example for Gitlab CI:

```
build:
    image: ghcr.io/niklasrosenstein/kaniq:latest
    variables:
        ARTIFACTORY_USER: ...
        ARTIFACTORY_PASSWORD: ...
    script:
        - ./kaniko-entrypoint.sh
```
