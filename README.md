## Axum - Rust

### Install SQLx CLI
```
cargo install sqlx-cli
```
### Migration Usage
```
# Create migration files with up and down
sqlx migrate -r create_xxx_table

# Run or revert migration
sqlx migrate run --database-url=DATABASE_DSN
sqlx migrate revert --database-url=DATABASE_DSN

# Enable building in "offline mode" with query!()
cargo sqlx prepare
```