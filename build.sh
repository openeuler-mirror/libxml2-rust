rm -rf CMakeCache.txt
make clean
cmake -DSTEP="build"
make
cd rust_ffi
cargo clean
cargo build
cd ../rust
cargo clean
cargo build
cd ../
cmake -DSTEP="link"
make
