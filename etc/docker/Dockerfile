# Copyright (c) 2019-2020 SRI International.
# All rights reserved.
#
#    This file is part of the Parsley parser.
#
#    Parsley is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    Parsley is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <https://www.gnu.org/licenses/>.

FROM rust AS builder

# build time optimization from:
# http://whitfin.io/speeding-up-rust-docker-builds/

# create a new empty shell project
RUN USER=root cargo new --bin parsley
WORKDIR /parsley

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# build for release
RUN rm ./target/release/deps/parsley*
RUN cargo build --release

RUN mkdir -p /build-out
RUN cp target/release/pdf_printer /build-out/

FROM debian:stretch-slim

COPY --from=builder /build-out/pdf_printer /

# No command or entrypoint since we are using this to just deploy the binary!
