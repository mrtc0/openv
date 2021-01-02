# openv

A tool that uses the credentials stored in 1password as an environment variable.

# Requirements

* [1password command-line tool](https://support.1password.com/command-line-getting-started/)

# Getting Started

```shell
# Sign in to 1password. e.g. `op signin my`.
$ eval $(op signin <sign_in_address>)

# Create vault in 1password.
$ op create vault myenv


# openv can register credentials to 1password vault.
$ openv myenv create SECRET_TOKEN
myenv.SECRET_TOKEN> this-is-token

$ openv myenv create HIDDEN_TOKEN
myenv.HIDDEN_TOKEN > this-is-hidden

# List credentials for the specified vault.
$ openv myenv list
HIDDEN_TOKEN
SECRET_TOKEN

# All credentials in vault can be set as environment variables with the `get` command
$ env $(openv myenv get) sh -c 'env | grep TOKEN'
SECRET_TOKEN=this-is-token
HIDDEN_TOKEN=this-is-hidden

# With the `-n` option, specify the credentials in vault.
# Also by separating item name with a colon, specify environment variable name.
$ env $(openv myenv get -n SECRET_TOKEN:MY_ENV) sh -c 'echo $MY_ENV'
this-is-token

> env $(openv myenv get -n SECRET_TOKEN:MY_ENV -n HIDDEN_TOKEN) sh -c 'echo $MY_ENV $HIDDEN_TOKEN'
this-is-token this-is-hidden
```

# Install

### On macOS via Homebrew

```shell
$ brew tap mrtc0/openv
$ brew instlal openv
```

### From binaries

Check out the [release page](https://github.com/mrtc0/openv/releases) for prebuilt versions of `openv` for many architectures.

### From source

```shell
$ cargo install openv
```

# Development

```shell
# Run tests
$ cargo test

# Build
$ cargo build --bins

# Install
$ cargo install --path . --locked
```
