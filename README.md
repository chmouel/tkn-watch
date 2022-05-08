[![codecov](https://codecov.io/gh/chmouel/tkn-watch/branch/main/graph/badge.svg?token=71R8DKNQVS)](https://codecov.io/gh/chmouel/tkn-watch) [![Version](https://img.shields.io/crates/v/tkn-watch.svg)](https://crates.io/crates/tkn-watch) [![AUR](https://img.shields.io/aur/version/tkn-watch-bin)](https://aur.archlinux.org/packages/tkn-watch-bin) [![CICD](https://github.com/chmouel/tkn-watch/actions/workflows/rust.yaml/badge.svg)](https://github.com/chmouel/tkn-watch/actions/workflows/rust.yaml) [![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

# tkn-watch - watch a PipelineRuns on its way to success or failures

tkn-watch is a simple extension to the [tkn](https://github.com/tektoncd/cli) command line tool that watches a PipelineRuns and exit with the PipelineRun status.

It mimics the behaviour of GitHub [cli](https://github.com/cli/cli) `run` `watch` command.

## Screencast

<https://user-images.githubusercontent.com/98980/167255324-d1c16973-df5f-473a-9285-2b7422840dd8.mov>

## Installation

### [Binaries](https://github.com/chmouel/tkn-watch/releases)

Go to the [release](https://github.com/chmouel/tkn-watch/releases) page and grab the archive or package targeting your platform.

### [Homebrew](https://homebrew.sh)

```shell
brew tap chmouel/tkn-watch https://github.com/chmouel/tkn-watch
brew install tkn-watch
```

### [Crates.io](https://crates.io/crates/tkn-watch)

```shell
cargo install tkn-watch
```

### [Arch](https://aur.archlinux.org/packages/tkn-watch-bin)

With your favourite aurhelper for example [yay](https://github.com/Jguer/yay) :

```shell
yay -S tkn-watch-bin
```

### [Docker](https://github.com/chmouel/tkn-watch/pkgs/container/tkn-watch)

```shell
docker run -i ghcr.io/chmouel/tkn-watch # don't forget to bind your kubeconfig
```

## Usage

```shell
% tkn watch <pipelinerun-name>
```

If you don't have `tkn` cli installed you can call the plug-in directly with `tkn-watch`

If you don't specify a PipelineRun it will ask you nicely for a running
Pipelinerun to watch, auto-selecting one if there is only one running.

When you give the flag `-l`/`--last` tkn-watch will use the last PipelineRun started.

You can use the flag `-n` to specify another namespace than the current one.

You can adjust the time to wait between checks with the flag `-r`/`--refresh-seconds`, the default is 3 seconds.

`tkn watch` exit with the pipelinerun status, so you can do fancy things like (on macOS):

```shell
tkn watch -l || osascript -e 'display notification "PipelineRun Has Failed :("' && osascript -e 'display notification "PipelineRun Has Succeeded, time to commit again :)"'
```

## Copyright

[Apache-2.0](./LICENSE)

## Authors

Chmouel Boudjnah <[@chmouel](https://twitter.com/chmouel)>
