list:
    just --list

# Formats entire workspace
format:
    cargo fmt --all

# Lints entire workspace
clippy:
    cargo clippy --all-targets -- -D warnings

# Builds entire workspace
build:
    cargo build --all

# Runs all tests in the workspace
test:
    cargo test --all

# Update dependencies
update:
    cargo update

# Runs benchmarks and generates report
bench: 
    mkdir -p ./report
    cargo bench > ./report/output.txt
    just build-readme-images

# Convert SVG benchmark images to PNG for README
build-readme-images:
    mkdir -p ./report
    resvg ./target/criterion/simple_insert/report/violin.svg ./report/simple_insert.png --background white
    resvg ./target/criterion/simple_iter/report/violin.svg ./report/simple_iter.png --background white
    resvg ./target/criterion/fragmented_iter/report/violin.svg ./report/fragmented_iter.png --background white
    resvg ./target/criterion/schedule/report/violin.svg ./report/schedule.png --background white
    resvg ./target/criterion/heavy_compute/report/violin.svg ./report/heavy_compute.png --background white
    resvg ./target/criterion/add_remove_component/report/violin.svg ./report/add_remove_component.png --background white
    # resvg ./target/criterion/serialize_text/report/violin.svg ./report/serialize_text.png --background white
    # resvg ./target/criterion/serialize_binary/report/violin.svg ./report/serialize_binary.png --background white

# Ensures everything works before publishing changes to branch
checks:
    just format
    just clippy
    just build
    just test
    just bench
