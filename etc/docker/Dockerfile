# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.

FROM rust AS builder

# Copy our source code
COPY ./ ./

# Fix permissions on source code.
#RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/pdf_printer /build-out/

# Now, we need to build our _real_ Docker container, copying in our executable.
FROM alpine

RUN apk --no-cache add ca-certificates

COPY --from=builder /build-out/pdf_printer /

# No command since we are using this to just deploy the binary!
