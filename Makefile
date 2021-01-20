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

# Get docker path or an empty string
DOCKER := $(shell command -v docker)
# Try to detect current branch if not provided from environment
BRANCH ?= $(shell git rev-parse --abbrev-ref HEAD)
# Commit hash from git
COMMIT=$(shell git rev-parse --short HEAD)

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

# Test if the dependencies we need to run this Makefile are installed
deps:
ifndef DOCKER
	@echo "Docker is not available. Please install docker"
	@exit 1
endif

docker: deps release etc/docker/Dockerfile
	docker build --build-arg=COMMIT=$(COMMIT) --build-arg=BRANCH=$(BRANCH)  -t 'pdf_printer' -f etc/docker/Dockerfile .
	docker tag pdf_printer safedocs-ta2-docker.cse.sri.com/pdf_printer:latest

deploy: docker
	docker push safedocs-ta2-docker.cse.sri.com/pdf_printer:latest
