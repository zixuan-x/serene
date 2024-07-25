# set dotenv-load
set dotenv-path := "server/.env"

run:
    cargo watch -C server -x run
