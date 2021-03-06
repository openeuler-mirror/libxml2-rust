#!/bin/bash
sudo yum clean all
sudo yum install -y gcc  openssl-libs

sudo yum install -y make cmake
sudo yum install -y libxml2-devel python3-devel zlib-devel bzip2-devel

export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup


#git加速并安装rust工具链
git config --global url."https://github.91chi.fun/https://github.com/".insteadOf "https://github.com/"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustlang.sh
sh rustlang.sh -y

source ~/.bashrc


