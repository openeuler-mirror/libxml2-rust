# CMake 最低版本号要求
cmake_minimum_required (VERSION 2.8)

# 项目信息
project (rust)

# 项目运行
execute_process(COMMAND cargo build)
