# rust-learning
learning rust by go through some examples 

## [Crust of Rust](./Crust%20of%20Rust/Readme.md)
Crust of Rust是Jon Gjengset的一个直播系列，主要关注点在于Rust中较为重要的技术点，通过学习这个直播系列，我可以对Rust的知识点和技术点有更为完整的认知。

## Practice

包含补上的 Advent of Code 代码，以及其他的小练习

- Advent of Code 2018
- Advent of Code 2020 (ToDo)

## [Operating System development tutorials in Rust on the Raspberry Pi](./rust-raspberrypi-OS-tutorials-note/README.md)
### 目标

- 通过学习编写项目 [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) 来加强rust的能力


### 进度

06_drivers_gpio_uart

### 01准备工作

1. 通过hyper-v安装Ubuntu 20.04.1虚拟机
2. 设置虚拟机环境
    1. 安装rust
    2. 安装vscode
    3. 安装docker
    4. 下载代码
3. 测试环境
    1. 使用Windows自带hyper-v虚拟机可以正常运行教程01
    2. 暂时受到虚拟机限制无法连接树莓派


### 02初始化运行时

1. 设置bss段，清空bss段
2. 清空完成进入panic，等待事件发生

### 03实现Hacky的UART打印输出

1. 编写bsp中的关于写入UART寄存器的方法，write_str
2. 编写宏print!、println!，来自rust官方
3. panic中打印信息
4. kernel_init中打印信息

### 04无损失抽象化 -> 需要补充笔记

1. 利用crate::cortex-a替换所有手写汇编
2. 新增smp模块，初始化系统，替换所有手写汇编

### 05使用全局变量进行打印输出 -> 需要补充笔记

1. 利用Rust Cell来进行全局输出记录，统计
2. 增加NullLock保证不会存在写冲突

### 06连接树莓派-运行示例06

1. 运行示例06
2. 连接树莓派至虚拟机
3. 获得预期输出