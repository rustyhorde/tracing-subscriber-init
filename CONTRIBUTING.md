# Contributing to tracing-subscriber-init
1. Fork the repository
1. Run the following to clone the repository.

    ```
    git clone git@github.com:<your fork>/tracing-subscriber-init.git
    cd tracing-subscriber-init
    ```

1. Install [`cargo-all-features`](https://github.com/frewsxcv/cargo-all-features)

    ```
    cargo install cargo-all-features --force
    ```

1. Make your changes
1. Before submitting a PR, make sure you have at least run the following

    ```
    cargo fmt
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -Dwarnings
    cargo build-all-features
    cargo test-all-features
    cargo +nightly doc --all-features
    ```

1. Push your changes to your fork and submit a PR.