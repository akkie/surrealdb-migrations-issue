# surrealdb-migrations-issue

Run `cargo test`.

The output is:

```
running 2 tests
test test_one ... FAILED
test deep::test::test_one ... FAILED

failures:

---- test_one stdout ----
Use .surrealdb file from: /usr/src/app/.surrealdb
thread 'test_one' panicked at app-main/tests/test.rs:78:14:
Failed to apply migrations: Error listing schemas directory

Caused by:
    Path does not directory

Location:
    /usr/local/cargo/registry/src/index.crates.io-6f17d22bba15001f/surrealdb-migrations-1.0.1/src/io.rs:219:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- deep::test::test_one stdout ----
Use .surrealdb file from: /usr/src/app/.surrealdb
thread 'deep::test::test_one' panicked at app-main/tests/test.rs:78:14:
Failed to apply migrations: Error listing schemas directory

Caused by:
    Path does not directory

```

Can also be executed in a VSCode devcontainer.