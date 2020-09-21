# rust-learning
learning rust by go through some examples 

## 目标

- 通过学习编写项目 [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) 来加强rust的能力


## 进度

1. 02 初始化运行时

## 准备工作

1. 通过hyper-v安装Ubuntu 20.04.1虚拟机
2. 设置虚拟机环境
    1. 安装rust
    2. 安装vscode
    3. 安装docker
    4. 下载代码
3. 测试环境
    1. 使用Windows自带hyper-v虚拟机可以正常运行教程01
    2. 暂时受到虚拟机限制无法连接树莓派


## 初始化运行时

1. 设置bss段，清空bss段
2. 清空完成进入panic，等待事件发生