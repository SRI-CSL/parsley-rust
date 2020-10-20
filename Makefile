#  Copyright (c) 2019-2020 SRI International.
#  All rights reserved.
#
#     This file is part of the Parsley parser.
#
#     Parsley is free software: you can redistribute it and/or modify
#     it under the terms of the GNU General Public License as published by
#     the Free Software Foundation, either version 3 of the License, or
#     (at your option) any later version.
#
#     Parsley is distributed in the hope that it will be useful,
#     but WITHOUT ANY WARRANTY; without even the implied warranty of
#     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#     GNU General Public License for more details.
#
#     You should have received a copy of the GNU General Public License
#     along with this program.  If not, see <https://www.gnu.org/licenses/>.

.PHONY: all release clean test fmt

all:
	cargo build

release:
	cargo build --release

test:
	cargo test

fmt:
	cargo +nightly fmt

clippy:
	cargo +nightly clippy

clean:
	cargo clean

docker: release etc/docker/Dockerfile
	docker build -t 'pdf_printer' -f etc/docker/Dockerfile .
	docker tag pdf_printer safedocs-ta2-docker.cse.sri.com/pdf_printer:latest

deploy: docker
	docker push safedocs-ta2-docker.cse.sri.com/pdf_printer:latest
