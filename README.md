# rust-learning
learning rust by go through some examples 

## 目标

- 通过学习编写项目 [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) 来加强rust的能力


## 进度

1. 04无损失抽象化

## 01准备工作

1. 通过hyper-v安装Ubuntu 20.04.1虚拟机
2. 设置虚拟机环境
    1. 安装rust
    2. 安装vscode
    3. 安装docker
    4. 下载代码
3. 测试环境
    1. 使用Windows自带hyper-v虚拟机可以正常运行教程01
    2. 暂时受到虚拟机限制无法连接树莓派


## 02初始化运行时

1. 设置bss段，清空bss段
2. 清空完成进入panic，等待事件发生

## 03实现Hacky的UART打印输出

1. 编写bsp中的关于写入UART寄存器的方法，write_str
2. 编写宏print!、println!，来自rust官方
3. panic中打印信息
4. kernel_init中打印信息

## 04无损失抽象化

1. 利用crate::cortex-a替换所有手写汇编
2. 新增smp模块，初始化系统，替换所有手写汇编