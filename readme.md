# kaniq

<a href="https://github.com/NiklasRosenstein/kaniq/actions/workflows/rust.yml"><img src="https://github.com/NiklasRosenstein/kaniq/actions/workflows/rust.yml/badge.svg"></a>
<a href="https://github.com/NiklasRosenstein/kaniq/actions/workflows/docker-image.yml"><img src="https://github.com/NiklasRosenstein/kaniq/actions/workflows/docker-image.yml/badge.svg"></a>
<a href="https://app.bitrise.io/app/150d483f5a30bb14"><img src="https://app.bitrise.io/app/150d483f5a30bb14/status.svg?token=fdwUoFfzB8XHHaA_04cmDA&branch=master"></a>

Kaniq <small>[/ ˈkɑnikju: /]</small> is intended to simplify builds with Google's [Kaniko]. It provides the `kaniq`
CLI as a standalone executable for use on your local machine as well as from inside a Kaniko container. Prebuilt
images that embedd the Kaniq CLI are provided at [`gcr.io/niklasrosenstein/kaniq`](https://github.com/users/NiklasRosenstein/packages/container/package/kaniq).

  [Kaniko]: https://github.com/GoogleContainerTools/kaniko

<table align="center">
<tr><th colspan="2">Run Kaniko locally</th></tr>
<tr><td>

```
docker run --rm -it 
  -v $PWD:/workspace 
  -w /workspace 
  --env USER=$USER 
  --env PASSWORD=... 
  --entrypoint '' 
  gcr.io/kaniko-project/executor:debug 
  scripts/kaniko_build.sh
```

</td><td>

```
kaniq run 
  --env USER 
  --env PASSWORD=... 
  scripts/kaniko_build.sh
```

</td></tr>

<tr><th colspan="2">Setup authentication</th></tr>
<tr><td>

```
mkdir -p /kaniko/.docker
auth=$(echo -n ${USER}:${PASSWORD} | base64)
cat << EOF > /kaniko/.docker/config.json
  {"auths": {"${REGISTRY}": "auth": "${auth}"}}
EOF
/kaniko/executor ...
```

</td><td>

```
/kaniko/kaniq execute
  --auth ${REGISTRY} ${USER} ${PASSWORD}
  ...
```

</td></tr>

<tr><th colspan="2">Add secrets</th></tr>
<tr><td>

```
mkdir -p /kaniko/secrets
echo "foobar" > /kaniko/secrets/USER
echo "$PASSWORD" > /kaniko/secrets/PASSWORD
/kaniko/executor ...
```

</td><td>

```
/kaniko/kaniq execute
  --secret USER=foobar --secret PASSWORD
  ...
```

</td></tr>

<tr><th colspan="2">CI Usage</th></tr>
<tr><td>

```yml
build_image:
  image:
    name: gcr.io/kaniko-project/executor:debug
    entrypoint: [""]
  script:
    - scripts/kaniko_build.sh
```

</td><td>

```yml
build_image:
  image:
    name: ghcr.io/niklasrosenstein/kaniq:latest
  script:
    - scripts/kaniko_build.sh
```

</td></tr>
</table>
