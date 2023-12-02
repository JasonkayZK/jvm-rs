# Chapter 6: 类和对象

编译 Java 文件：

```shell
javac resource/jvmrust/ch06/*.java
```

加载 Class 并执行 main：

Circle:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch06.Circle

[ INFO]: ch06_rtda_heap::cmd - classpath: /Users/zk/workspace/jvm-rs/ch06_rtda_heap/resource class: jvmrust.ch06.Circle args: []
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded java/lang/Object
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/Circle
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 0, inst:LDC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 2, inst:PUT_STATIC { index: 3 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 5, inst:NEW { index: 4 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 8, inst:DUP
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 9, inst:INVOKE_SPECIAL { index: 5 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 12, inst:ASTORE_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 13, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 14, inst:LDC { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 16, inst:PUT_FIELD { index: 7 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 19, inst:GET_STATIC { index: 3 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 22, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 23, inst:GET_FIELD { index: 7 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 26, inst:FMUL
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 27, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 28, inst:GET_FIELD { index: 7 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 31, inst:FMUL
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 32, inst:FSTORE_2
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 33, inst:GET_STATIC { index: 8 }
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded java/lang/System
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 36, inst:FLOAD_2
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 37, inst:INVOKE_VIRTUAL { index: 9 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 94.985
thread 'main' panicked at 'Unsupported opcode: 0xb1!', ch06_rtda_heap/src/instructions/interpret.rs:49:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

FieldInitTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch06.FieldInitTest

[ INFO]: ch06_rtda_heap::cmd - classpath: /Users/zk/workspace/jvm-rs/ch06_rtda_heap/resource class: jvmrust.ch06.FieldInitTest args: []
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded java/lang/Object
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/FieldInitTest
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 0, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded java/lang/System
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 3, inst:GET_STATIC { index: 3 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 6, inst:INVOKE_VIRTUAL { index: 4 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - false
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 9, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 12, inst:GET_STATIC { index: 5 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 15, inst:INVOKE_VIRTUAL { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 18, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 21, inst:GET_STATIC { index: 7 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 24, inst:INVOKE_VIRTUAL { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 27, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 30, inst:GET_STATIC { index: 8 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 33, inst:INVOKE_VIRTUAL { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 36, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 39, inst:GET_STATIC { index: 9 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 42, inst:INVOKE_VIRTUAL { index: 10 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 45, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 48, inst:GET_STATIC { index: 11 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 51, inst:INVOKE_VIRTUAL { index: 12 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 54, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 57, inst:GET_STATIC { index: 13 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 60, inst:INVOKE_VIRTUAL { index: 14 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 63, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 66, inst:GET_STATIC { index: 15 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 69, inst:INVOKE_VIRTUAL { index: 16 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 72, inst:NEW { index: 17 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 75, inst:DUP
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 76, inst:INVOKE_SPECIAL { index: 18 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 79, inst:ASTORE_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 80, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 83, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 84, inst:GET_FIELD { index: 19 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 87, inst:INVOKE_VIRTUAL { index: 4 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - false
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 90, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 93, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 94, inst:GET_FIELD { index: 20 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 97, inst:INVOKE_VIRTUAL { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 100, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 103, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 104, inst:GET_FIELD { index: 21 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 107, inst:INVOKE_VIRTUAL { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 110, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 113, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 114, inst:GET_FIELD { index: 22 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 117, inst:INVOKE_VIRTUAL { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 120, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 123, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 124, inst:GET_FIELD { index: 23 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 127, inst:INVOKE_VIRTUAL { index: 10 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 130, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 133, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 134, inst:GET_FIELD { index: 24 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 137, inst:INVOKE_VIRTUAL { index: 12 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 140, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 143, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 144, inst:GET_FIELD { index: 25 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 147, inst:INVOKE_VIRTUAL { index: 14 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 150, inst:GET_STATIC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 153, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 154, inst:GET_FIELD { index: 26 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 157, inst:INVOKE_VIRTUAL { index: 16 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 0
thread 'main' panicked at 'Unsupported opcode: 0xb1!', ch06_rtda_heap/src/instructions/interpret.rs:49:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

FieldResolutionTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch06.FieldResolutionTest

[ INFO]: ch06_rtda_heap::cmd - classpath: /Users/zk/workspace/jvm-rs/ch06_rtda_heap/resource class: jvmrust.ch06.FieldResolutionTest args: []
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded java/lang/Object
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/FieldResolutionTest
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 0, inst:NEW { index: 2 }
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/FieldResolutionTest$B
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/FieldResolutionTest$I
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/FieldResolutionTest$C
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 3, inst:DUP
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 4, inst:ACONST_NULL
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 5, inst:INVOKE_SPECIAL { index: 3 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 8, inst:ASTORE_1
thread 'main' panicked at 'Unsupported opcode: 0xb1!', ch06_rtda_heap/src/instructions/interpret.rs:49:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

FieldTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch06.FieldTest

[ INFO]: ch06_rtda_heap::cmd - classpath: /Users/zk/workspace/jvm-rs/ch06_rtda_heap/resource class: jvmrust.ch06.FieldTest args: []
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded java/lang/Object
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/FieldTest
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 0, inst:NEW { index: 2 }
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/FieldTest$Inner
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 3, inst:DUP
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 4, inst:INVOKE_SPECIAL { index: 3 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 7, inst:ASTORE_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 8, inst:ALOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 9, inst:BIPUSH { val: 100 }
thread 'main' panicked at 'Unsupported opcode: 0xb8!', ch06_rtda_heap/src/instructions/interpret.rs:49:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

MyObject:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch06.MyObject

[ INFO]: ch06_rtda_heap::cmd - classpath: /Users/zk/workspace/jvm-rs/ch06_rtda_heap/resource class: jvmrust.ch06.MyObject args: []
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded java/lang/Object
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded jvmrust/ch06/MyObject
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 0, inst:LDC { index: 2 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 2, inst:ISTORE_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 3, inst:NEW { index: 3 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 6, inst:DUP
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 7, inst:INVOKE_SPECIAL { index: 4 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 10, inst:ASTORE_2
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 11, inst:ILOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 12, inst:PUT_STATIC { index: 5 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 15, inst:GET_STATIC { index: 5 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 18, inst:ISTORE_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 19, inst:ALOAD_2
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 20, inst:ILOAD_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 21, inst:PUT_FIELD { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 24, inst:ALOAD_2
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 25, inst:GET_FIELD { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 28, inst:ISTORE_1
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 29, inst:ALOAD_2
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 30, inst:ASTORE_3
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 31, inst:ALOAD_3
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 32, inst:INSTANCE_OF { index: 3 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 35, inst:IFEQ { offset: 18 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 38, inst:ALOAD_3
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 39, inst:CHECK_CAST { index: 3 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 42, inst:ASTORE_2
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 43, inst:GET_STATIC { index: 7 }
[ INFO]: ch06_rtda_heap::rtda::heap::class_loader - [Loaded java/lang/System
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 46, inst:ALOAD_2
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 47, inst:GET_FIELD { index: 6 }
[ INFO]: ch06_rtda_heap::instructions::interpret - pc: 50, inst:INVOKE_VIRTUAL { index: 8 }
[ INFO]: ch06_rtda_heap::instructions::references::invokevirtual - 32768
thread 'main' panicked at 'Unsupported opcode: 0xb1!', ch06_rtda_heap/src/instructions/interpret.rs:49:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
