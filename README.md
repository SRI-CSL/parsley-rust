# README and Notes for Parsley Rust


## Requirements

Rust compiler 1.37.0 or higher
Cargo package manager (included with Rust)

Install on Mac OS X with homebrew:

    $ brew uninstall rust  # if you used `brew install rust` before
    $ brew install rustup
    $ rustup-init
    [... use default configuration... ]
    $ source $HOME/.cargo/env
    $ rustc --version
    rustc 1.38.0 (625451e37 2019-09-23)
    $ cargo --version
    cargo 1.38.0 (23ef9a4ef 2019-08-20)

## Building with Cargo

    $ make  # -> actually calls:

or

    $ cargo build
       Compiling parsley-rust v0.1.0 (/Users/linda/git/GitHub/Parsley_repos/parsley-rust)
        Finished dev [unoptimized + debuginfo] target(s) in 1.92s


## Running unit tests

    $ make test  # -> actually calls:

or

    $ cargo test
