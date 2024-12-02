set export
RUSTFLAGS := "-C target-cpu=native"
bench:
    cargo bench --profile profiling
report:
    open target/criterion/report/index.html
