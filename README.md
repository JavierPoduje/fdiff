# Fdiff

Compares the commits by summary of two branches in a Git repository.

## Dependencies

1. [Rust](https://www.rust-lang.org/tools/install).

## Initial setup

Clone the repository:

```sh
$ git clone git@github.com:JavierPoduje/fdiff.git
```

Build:

```sh
$ cargo build --release
```

Make it executable anywhere:

```sh
$ sudo cp target/release/fdiff /usr/bin/fdiff
```

## Usage

Get the commits in `branch1` that doesn't exist in `branch2`:

```sh
$ fdiff branch1 branch2
```

Same as above but excluding the commits that have the `fix/` or `release/` substrings in them:
```sh
$ fdiff branch1 branch2 --exclude fix/ release/
```
