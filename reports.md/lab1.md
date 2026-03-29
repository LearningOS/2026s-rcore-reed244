## 实验一：系统调用跟踪

    每一个TaskControlBlock类型的实例都新增了一个成员变量called_times，call_times用于记录该任务被跟踪的系统调用次数。每当一个系统调用被跟踪时，当前正在运行的任务的called_times对应counter就会加1。通过这个成员变量，我们可以在用户空间的应用程序中获取当前任务被跟踪的系统调用次数，从而实现系统调用跟踪功能。

## 问答题

1. bad_address中写入违法地址0x0,触发了StoreFault。bad_instruction调用sret指令,触发了IllegalInstruction。bad_register调用csrr,触发IllegalInstruction。

2. 1. sp代表了内核态的栈指针。一是从trap返回用户态，二是第一次加载task时
   2. 分别取回了sstatus spec sscratch寄存器的值，sstatus可以帮助正确地返回用户态，spec确保了返回了用户态从那一条指令开始，sscratch则是内核态的栈指针。
   3. sp（x2）寄存器的值在sscratch中，tp（x4）寄存器程序没用
   4. sp指向用户态的栈顶，sscratch寄存器保存了内核态栈指针的值
   5. sret指令用于从内核态返回用户态，sret指令会将sstatus寄存器中的值恢复到用户态，并跳转到spec寄存器中保存的地址继续执行用户态的代码。
   6. sp指向内核态的栈顶，sscratch寄存器保存了内核态栈指针的值
   7. sbi_call处的ecall指令

## 荣誉准则

1.在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    和github-copilot就tasktrace是否需要每一个task独立记录

2.此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    rCore-tutorial-Guide:https://learningos.cn/rCore-Tutorial-Guide/honorcode.html

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
