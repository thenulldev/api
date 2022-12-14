# NB: This is not a production-grade Dockerfile.

#################
## build stage ##
#################
FROM rust:latest AS builder
WORKDIR /code

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
RUN USER=root cargo init
RUN apt update && apt install pkg-config libssl-dev -y
COPY Cargo.toml Cargo.toml
RUN cargo fetch

# copy app files
COPY src src

# compile app
RUN cargo build --release

###############
## run stage ##
###############
FROM debian:latest
WORKDIR /app

RUN apt update
RUN apt install ca-certificates -y

# copy server binary from build stage
COPY --from=builder /code/target/release/null-api null-api

# set user to non-root unless root is required for your app
USER 1001

# indicate what port the server is running on
EXPOSE 8080

# run server
CMD [ "/app/null-api" ]
