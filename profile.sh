export HYDRO_STD_ROOT=./standard_libraries/hydro
cargo build --release
valgrind --tool=callgrind target/release/ocean "$@"