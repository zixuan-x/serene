# set dotenv-load

run-server:
    set dotenv-path := "server/.env"
    cargo watch -C server -x run
run-web:
    npm --prefix web run dev
