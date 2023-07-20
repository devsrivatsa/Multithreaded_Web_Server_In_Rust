FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /simple_networking
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# CREATE A NEW STAGE WITH A MINIMAL IMAGE
FROM scratch
COPY --from=builder /simple_networking/target/x86_64-unknown-linux-musl/relese/simple_networking /simple_networking
ENTRYPOINT [ "/simple_networking" ]
EXPOSE 3000