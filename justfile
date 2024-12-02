set export
RUSTFLAGS := "-C target-cpu=native"
bench:
    cargo bench --profile profiling
