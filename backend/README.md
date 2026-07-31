# Butterfly API

API and server component for the Butterfly project.

### Building and Running

- TBA

### Migrating Database

First install [`cargo-binstall`](https://docs.rs/crate/cargo-binstall/latest) and then install the [`sqlx–cli`](https://lib.rs/crates/sqlx-cli) command with `cargo binstall sqlx-cli`. Afterwards you can configuration the environment variables with `source` and run the `sqlx` commands.

```sh
source .env
sqlx migrate run
```
