from rust:1-slim-bookworm as builder

workdir /usr/src/app

copy Cargo.toml Cargo.lock ./
copy src ./src


run cargo build --release --locked

#minimal runtime inage no compiler, no source code
from debian:bookworm-slim as runtime
arg APP_VERSION=local
env APP_VERSION=${APP_VERSION} \
    PORT=8000

run useradd --system --uid 10001 --no-create-home appuser
copy --from=builder /usr/src/app/target/release/devops-rpi-web /usr/local/bin/devops-rpi-web

user appuser
expose 8000

healthcheck --interval=30s --timeout=3s --start-period=5s --retries=3 \
  cmd ["/usr/local/bin/devops-rpi-web", "--healthcheck"]

entrypoint ["/usr/local/bin/devops-rpi-web"]
