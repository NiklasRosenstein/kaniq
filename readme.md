# kaniq

Kaniq <small>[/ ˈkɑnikju: /]</small> is intended to simplify builds with Google's [Kaniko]. It provides the `kaniq`
CLI as a standalone executable for use on your local machine as well as from inside a Kaniko container. Prebuilt
images that embedd the Kaniq CLI are provided at `gcr.io/niklasrosenstein/kaniq`.

__Features__

* Invoke a local Kaniko build without having to remember the long Docker command-line.
* Easily configure Kaniko Docker registry authentication
* Adds support for secrets

## Quickstart

Download the Kaniq CLI:

```
$ curl https://github.com/NiklasRosenstein/kaniq/releases/0.1.0/kaniq_darwin_arm64 -o kaniq
```

Define a script that serves as the entrypoint to kick off the Kaniko build:

```
$ cat << EOF > kaniko-entrypoint.sh
#!/bin/sh

# Log into Artifactory so we can push to it.
kaniq auth my.jfrog.io $ARTIFACTORY_USER $ARTIFACTORY_PASWORD

# Expose the same credentials to the Docker build as secrets.
kaniq execute --secret ARTIFACTORY_USER --secret ARTIFACTORY_PASSWORD \
    --cache=true --cache-copy-layers \
    --destination my.jfrog.io/docker/my-project:latest 

EOF
$ chmod +x kaniko-entrypoint.sh
```

Kick off the Kaniko build locally:

```
$ export ARTIFACTORY_USER=... ARTIFACTORY_PASSWORD=...
# Run the Kaniko build, exporting your entire environment to the Kaniko container
# such that they are accessible by your entrypoint script.
$ kaniq run --env-all kaniko-entrypoint.sh 
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
