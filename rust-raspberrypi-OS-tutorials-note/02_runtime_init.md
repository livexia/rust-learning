# 02_runtime_init 初始化runtime

需要了解更多的汇编指令

**指令**

1. mrs：移特殊定寄存器的值到通用寄存器，[Move the contents of a special register to a general-purpose register.](https://developer.arm.com/documentation/101272/0001/The-Cortex-M55-Instruction-Set--Reference-Material/Miscellaneous-instructions/MRS?lang=en)
2. and：和运算，可以用作内存位保留
3. cbz: 与0进行比较，并且跳转，[Compare and Branch on Zero](https://developer.arm.com/documentation/dui0489/c/arm-and-thumb-instructions/branch-and-control-instructions/cbz-and-cbnz)
4. ldr: 将数据从内存放置到cpu寄存器里
5. mov：移动寄存器的值到另一个寄存器
6. bl：分支判断，并自动改变记录返回地址的寄存器

**寄存器**

1. mpidr_el1
    - 
    - [aarch64-system-registers/mpidr_el1](https://developer.arm.com/docs/ddi0595/h/aarch64-system-registers/mpidr_el1)
