# README and Notes for Parsley Rust

[![Build Status](https://travis-ci.com/SRI-CSL/parsley-rust.svg?token=o5TGhkjJAL4mzkVZNy5S&branch=master)](https://travis-ci.com/SRI-CSL/parsley-rust)

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

To create a stand-alone binary:

    $ cargo build --release
    $ ls -la target/release/pdf_printer
    -rwxr-xr-x  2 linda  staff  690124 Nov 12 16:13 target/release/pdf_printer

or use:

    $ make release

## Running the PDF Parser

With cargo:

    $ cargo run <PDF_file>

for example:

    $ cargo run tests/test_files/minimal_leading_garbage.pdf
       Compiling parsley-rust v0.1.0 (/Users/linda/git/GitHub/Parsley_repos/parsley-rust)
        Finished dev [unoptimized + debuginfo] target(s) in 2.71s
         Running `target/debug/pdf_printer tests/test_files/minimal_leading_garbage.pdf`
    INFO     - minimal_leading_garbage.pdf at         21 - Found 21 bytes of leading garbage, dropping from buffer.
    INFO     - minimal_leading_garbage.pdf at        754 - Found %%EOF at offset 754.
    INFO     - minimal_leading_garbage.pdf at        746 - Found startxref at offset 746.
    INFO     - minimal_leading_garbage.pdf at        740 -  startxref span: 740..753.
    INFO     - minimal_leading_garbage.pdf at        586 - startxref points to offset 586 for xref
    INFO     - minimal_leading_garbage.pdf at        570 - Found 5 objects starting at 0:
    DEBUG    - minimal_leading_garbage.pdf    free object (next is 0).
    DEBUG    - minimal_leading_garbage.pdf    inuse object at 18.
    DEBUG    - minimal_leading_garbage.pdf    inuse object at 77.
    DEBUG    - minimal_leading_garbage.pdf    inuse object at 178.
    DEBUG    - minimal_leading_garbage.pdf    inuse object at 457.
    INFO     - minimal_leading_garbage.pdf at          0 - Found trailer 0 bytes from end of xref table.
    DEBUG    - minimal_leading_garbage.pdf Beginning breadth-first traversal of root object:
    [...]
    
Or the stand-alone binary:

    $ target/release/pdf_printer
    Usage:
    	target/release/pdf_printer <pdf-file>

etc.
    
## Installing a static binary

    $ cargo install --path .
      Installing parsley-rust v0.1.0 (/Users/linda/git/GitHub/Parsley_repos/parsley-rust)
        Updating crates.io index
      Downloaded env_logger v0.7.1
      Downloaded libc v0.2.65
      Downloaded autocfg v0.1.7
       Compiling autocfg v0.1.7
       Compiling libc v0.2.65
       Compiling num-traits v0.2.8
       Compiling num-integer v0.1.41
       Compiling time v0.1.42
       Compiling atty v0.2.13
       Compiling env_logger v0.7.1
       Compiling chrono v0.4.9
       Compiling parsley-rust v0.1.0 (/Users/linda/git/GitHub/Parsley_repos/parsley-rust)
        Finished release [optimized] target(s) in 36.82s
      Installing /Users/linda/.cargo/bin/pdf_printer
       Installed package `parsley-rust v0.1.0 (/Users/linda/git/GitHub/Parsley_repos/parsley-rust)` (executable `pdf_printer`)
    CSL-CAS15874:parsley-rust linda$ which pdf_printer
    /Users/linda/.cargo/bin/pdf_printer
         
## Running unit tests

    $ make test  # -> actually calls:

or

    $ cargo test

## Dockerizing Rust binary

    $ make docker

See `etc/docker/README.md` for more details.
