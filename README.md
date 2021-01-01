# openv

A tool that uses the credentials stored in 1password as an environment variable.

# Requirements

* [1password command-line tool](https://support.1password.com/command-line-getting-started/)

# Getting Started

```shell
$ eval $(op signin <sign_in_address>)
$ op create vault myenv

$ openv create myenv SECRET_TOKEN
myenv.SECRET_TOKEN> this-is-secret

$ openv list myenv
SECRET_TOKEN

$ env $(openv myenv get) sh -c 'echo $SECRET_TOKEN'
this-is-secret

$ env $(openv myenv get -n SECRET_TOKEN:MY_ENV) sh -c 'echo $MY_ENV'
this-is-secret
```

# Install

### On macOS via Homebrew

TBD

### From binaries

Check out the [release page](https://github.com/mrtc0/openv/releases) for prebuilt versions of `openv` for many architectures.

### From source

TBD

# Development

```shell
# Run tests
$ cargo test

# Build
$ cargo build --bins

# Install
$ cargo install --path . --locked
```
