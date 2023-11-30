# Chapter 5: 指令集和解释器

Reference:

- https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html

编译 Java 文件：

```shell
javac resource/GaussTest.java
```

解释并执行 Class 文件：

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre GaussTest

# Output:

classpath: /Users/zk/workspace/jvm-rs/ch05_instructions/resource class: GaussTest args: []

pc: 0, inst:ICONST_0
pc: 1, inst:ISTORE_1
pc: 2, inst:ICONST_1
pc: 3, inst:ISTORE_2
pc: 4, inst:ILOAD_2
pc: 5, inst:BIPUSH { val: 100 }
pc: 7, inst:IF_ICMPGT { offset: 13 }
pc: 10, inst:ILOAD_1
pc: 11, inst:ILOAD_2
pc: 12, inst:IADD
pc: 13, inst:ISTORE_1
pc: 14, inst:IINC { index: 2, const_val: 1 }
pc: 17, inst:GOTO { offset: -13 }
pc: 4, inst:ILOAD_2
pc: 5, inst:BIPUSH { val: 100 }
pc: 7, inst:IF_ICMPGT { offset: 13 }
pc: 10, inst:ILOAD_1
pc: 11, inst:ILOAD_2
pc: 12, inst:IADD
pc: 13, inst:ISTORE_1
pc: 14, inst:IINC { index: 2, const_val: 1 }
pc: 17, inst:GOTO { offset: -13 }
pc: 4, inst:ILOAD_2
pc: 5, inst:BIPUSH { val: 100 }
pc: 7, inst:IF_ICMPGT { offset: 13 }
pc: 10, inst:ILOAD_1
pc: 11, inst:ILOAD_2
pc: 12, inst:IADD
pc: 13, inst:ISTORE_1
pc: 14, inst:IINC { index: 2, const_val: 1 }
pc: 17, inst:GOTO { offset: -13 }
pc: 4, inst:ILOAD_2
pc: 5, inst:BIPUSH { val: 100 }
pc: 7, inst:IF_ICMPGT { offset: 13 }
pc: 10, inst:ILOAD_1
pc: 11, inst:ILOAD_2
pc: 12, inst:IADD
pc: 13, inst:ISTORE_1
pc: 14, inst:IINC { index: 2, const_val: 1 }
pc: 17, inst:GOTO { offset: -13 }
pc: 4, inst:ILOAD_2
pc: 5, inst:BIPUSH { val: 100 }
pc: 7, inst:IF_ICMPGT { offset: 13 }
pc: 10, inst:ILOAD_1
pc: 11, inst:ILOAD_2
pc: 12, inst:IADD
pc: 13, inst:ISTORE_1
pc: 14, inst:IINC { index: 2, const_val: 1 }
pc: 17, inst:GOTO { offset: -13 }
pc: 4, inst:ILOAD_2
pc: 5, inst:BIPUSH { val: 100 }
pc: 7, inst:IF_ICMPGT { offset: 13 }
pc: 10, inst:ILOAD_1
pc: 11, inst:ILOAD_2
pc: 12, inst:IADD
pc: 13, inst:ISTORE_1
pc: 14, inst:IINC { index: 2, const_val: 1 }
pc: 17, inst:GOTO { offset: -13 }
pc: 4, inst:ILOAD_2
...

LocalVars: LocalVar { vars: [Ref(None), Num(5050), Num(101)] }
OperandStack: OperandStack { size: 0, vars: [Num(101), Num(100)] }
thread 'main' panicked at 'Unsupported opcode: 0xb2!', ch05_instructions/src/instructions/interpret.rs:53:17
```
