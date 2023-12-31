set export
set dotenv-load

docker_compose_bin := "docker compose -f compose.dev.yaml"
sqlx_bin := "~/.cargo/bin/sqlx"

# env vars
RUSTFLAGS := "--cfg uuid_unstable"

start:
    {{docker_compose_bin}} up -d --wait

stop:
    {{docker_compose_bin}} down

cargo *args:
    cargo {{args}}

build:
    just cargo build

sqlx *args:
    {{sqlx_bin}} {{args}}
