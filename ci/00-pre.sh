#!/usr/bin/env bash

# Function to check if a file contains Chinese characters
contains_chinese() {
    grep -Pn '[\p{Han}]' "$1" && echo "DO NOT USE CHINESE CHARACTERS in code, 不要在源码中使用中文!" && exit 1
}

# Check for Chinese characters in modified Rust files
git diff origin/master --name-only | grep -F '.rs' | while IFS= read -r rustfile; do
    contains_chinese "$rustfile"
done

# Install required tools if not already installed
if [ ! -f "/etc/centos-release" ] && [ ! -f "/etc/fedora-release" ]; then
    required_packages=("gcc" "python3-devel" "make" "autoconf" "automake" "libtool" "zlib-devel" "xz-devel" "pkgconfig")
else
    required_packages=("gcc" "python3-devel" "make" "autoconf" "automake" "libtool" "zlib-devel" "xz-devel" "pkgconfig")
fi
missing_packages=()

for package in "${required_packages[@]}"; do
    rpm -qi "$package" > /dev/null 2>&1 || missing_packages+=("$package")
done

if [ "${#missing_packages[@]}" -gt 0 ]; then
    sudo sed -i "s:repo.openeuler.org:repo.huaweicloud.com/openeuler:g" /etc/yum.repos.d/*.repo
    sudo yum install --refresh --disablerepo OS --disablerepo EPOL --disablerepo source --disablerepo update --disablerepo EPOL-UPDATE --disablerepo debuginfo -y "${missing_packages[@]}" || exit 1
fi

source ~/.bashrc
cargo -v
if [ $? -ne 0 ]; then
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
curl --proto '=https'  -k https://sh.rustup.rs -o rustlang.sh
sh rustlang.sh -y --default-toolchain none
rm -rf rustlang.sh
fi

source "$HOME/.cargo/env"
rustup default 1.65

arch=`uname -m`
# install musl-build
rustup target add $arch-unknown-linux-musl

# Define the crate names to test
crate_names=("https://github.com/rust-lang/crates.io-index" \
            "https://mirrors.ustc.edu.cn/crates.io-index" \
            "https://rsproxy.cn/crates.io-index" \
            "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git" \
            "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index")

# Initialize minimum latency to a large value
min_latency=9999999

# Define timeout in seconds
timeout=10

# Define the fastest source
fastest_source=""

# Test each crate
for crate_name in "${crate_names[@]}"; do
    echo "Running test for $crate_name..."

    # Send an HTTP request to get crate information and measure the execution time
    start_time=$(date +%s%N)
    response=$(curl -s -o /dev/null --connect-timeout 10 -w "%{time_total}" "$crate_name" > /dev/null 2>&1)
    if [ $? -ne 0 ]; then
        continue
    fi
    end_time=$(date +%s%N)

    # Calculate the request time in milliseconds
    duration=$(( ($end_time - $start_time) / 1000000 ))

    echo "Test result for $crate_name: $duration ms"

    # Check if it's the fastest source
    if [ "$duration" -lt "$min_latency" ]; then
        min_latency="$duration"
        fastest_source="$crate_name"
    fi

    echo ""
done

echo "Fastest source: $fastest_source with latency $min_latency ms"

# Fix cargo clippy timeout by replacing cargo crates with the fastest source
arch=$(uname -m)

# Modify config
mkdir -p ~/.cargo
cat << EOF > ~/.cargo/config
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# Use the fastest source
replace-with = 'replace'

[source.replace]
registry = "$fastest_source"

[target.$arch-unknown-linux-musl]
rustflags = ["-C", "target-feature=-crt-static"]

[net]
git-fetch-with-cli = true
EOF

rm -rf  ~/.cargo/.package-cache

# #git加速并安装rust工具链
# repo="https://github.com/rust-lang/release-team.git"
# git config --global http.lowSpeedLimit 5
# git config --global http.lowSpeedTime 30
# git config --global url."https://gitclone.com/github.com/".insteadOf "https://github.com/"
# git clone $repo
# if [ $? -ne 0 ]; then
#     git config --unset --global url."https://gitclone.com/github.com/".insteadOf "https://github.com/"
#     git config --global url."https://gh.api.99988866.xyz/https://github.com/".insteadOf "https://github.com/"
#     git clone $repo
#     if [ $? -ne 0 ]; then
#       git config --unset --global url."https://gh.api.99988866.xyz/https://github.com/".insteadOf "https://github.com/"
#     fi
# fi
# rm -rf ./awesome-rust.git

##拉取代码
#rm -rf sysmaster
#git clone https://gitee.com/openeuler/sysmaster.git
#cd sysmaster
#git checkout -b pr_$prid
#git fetch origin pull/$prid/head:master-$prid
#git merge --no-edit master-$prid
