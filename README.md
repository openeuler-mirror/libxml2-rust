# libxml2-rust

#### 介绍
rewrite memory leak modules for libxml2 using Rust

#### 特别说明

libxml2-2.9.12_openEuler_version目录下的是原始代码，但在openEuler操作系统下有部分ctest测试用例无法通过，故我们将libxml2官方库的代码放在libxml2-2.9.12_github_version目录下一并提交。所有对libxml2-2.9.12_openEuler_version的改写操作也会同步到libxml2-2.9.12_github_version，通过后者能否通过所有ctest测试用例来判断改写内容的正确性。

#### 软件架构
软件架构说明


#### 安装教程
准备make,cmake,rust等环境。版本推荐：
- cmake:3.10
- make: 3.8
- rust：最新稳定版

安装步骤如下：

1. 通过gitee 获取代码 git clone git@gitee.com:openeuler/libxml2-rust.git

2. 运行libxml2-2.9.12_openEuler_version/build.sh编译代码

3. 运行make install 将代码安装到系统中

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
