# general-todo
interview project for General Data


# how to run

In order to run this project you will need to install rust via [rustup](https://rustup.rs/) and have a postgres database available.



to set everything up:
```
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cargo install sqlx-cli
export DATABASE_URL=postgres://postgres@localhost/my_database (change to match the database for this project)
cd todo-server
sqlx migrate run
cd ../todo-client
trunk build
```

then to run:
```
cd todo-server
cargo run
```