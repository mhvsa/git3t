# git3t

A simple CLI tool for generating time reports from gitlab.


## Installation

You need the rust toolchain to install git3t. You can install it from [rustup](https://rustup.rs/).

```bash
cargo install git3t
```

## Usage

Set the GITLAB_TOKEN environment variable to your gitlab token.
Token has to have the api scope.
Then run `git3t help` to get a list of all commands.

```bash
export GITLAB_TOKEN=<your token>
git3t help <command>
```

