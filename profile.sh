export HYDRO_STD_ROOT=./standard_libraries/hydro
cargo build --release
valgrind --tool=callgrind target/release/ocean "$@"
kcachegrind --desktopfile "$(find . -type f -exec ls -t1 {} + | head -1)"