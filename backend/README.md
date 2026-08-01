# Butterfly API

API and server component for the Butterfly project.

### Building and Running

```sh
cargo run
```

### Migrating Database

Automatic migrations can be configured in the environment files. For manual migrations, first install [`cargo-binstall`](https://docs.rs/crate/cargo-binstall/latest) and then install the [`sqlx–cli`](https://lib.rs/crates/sqlx-cli) command with `cargo binstall sqlx-cli`. Afterwards you can configure the environment variables with `source` and run the `sqlx` commands.

```sh
source .env
sqlx migrate run
```
