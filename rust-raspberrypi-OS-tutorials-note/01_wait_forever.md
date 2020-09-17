# 关键点

## 代码结构

1. 遵循rust（cargo）项目的结构
2. 所有内核的子模块都是直接位于 src 下，例如内存子模块位于 src/memory.rs 
3. 所有与架构、指令系统相关的代码都位于 src/_arch 下，例如aarch64的代码位于 src/_arch/aarch64
4. 所有与具体运行所在的开发板相关的代码都位于 src/bsp 下

## link.ld

是一个链接脚本，决定了输入的一系列 object 文件，最后是如何组成为一个 object 文件的。连接脚本由一系列的链接命令组成。

```
SECTIONS  /* script的一个最常用的命令 */
{
    /* Set current address to the value from which the RPi starts execution */
    . = 0x80000;  /* 指定起始地址 */ 

    .text : /* 输出object文件存放代码段的内容 */
    {
        *(.text._start) *(.text*)
    }

    /DISCARD/ : { *(.comment*) }
}
```

### 相关疑问

**为什么位于 bsp 下？是与开发板相关的代码吗？**

可能与物理开发板的内存排布与映射等相关

> 阅读材料：https://stuff.mit.edu/afs/athena/project/rhel-doc/3/rhel-ld-en-3/scripts.html


## main.rs

1. [asm](https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html)
2. [global-asm](https://doc.rust-lang.org/beta/unstable-book/library-features/global-asm.html)

## cpu.S

涉及指令：wfe、b

wfe：wait for event，等待事件发生，如果事件发生，cpu会进入不同的模式

b：分支判读，1b表示向上寻找标签1