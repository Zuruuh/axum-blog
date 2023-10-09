set export
set dotenv-load

docker_compose_bin := "docker compose -f compose.dev.yaml"
sqlx_bin := "~/.cargo/bin/sqlx"

# env vars
RUSTFLAGS := "--cfg uuid_unstable"

start:
    {{docker_compose_bin}} up -d

cargo *args:
    cargo {{args}}

sqlx *args:
    {{sqlx_bin}} {{args}}
