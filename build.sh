rm -rf CMakeCache.txt
make clean
cmake -DSTEP="build"
make
cd rust
cargo clean
cargo build --release -v
cd ../
cmake -DSTEP="link"
make
