# Chapter 10: 异常处理

编译 Java 文件：

```shell
javac resource/jvmrust/ch10/*.java
```

JvmsExample:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch10.JvmsExample

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded java/io/Serializable
[ INFO]: [Loaded java/lang/reflect/AnnotatedElement
[ INFO]: [Loaded java/lang/reflect/GenericDeclaration
[ INFO]: [Loaded java/lang/reflect/Type
[ INFO]: [Loaded java/lang/Class
[ INFO]: [Loaded jvmrust/ch10/JvmsExample
[ INFO]: [Loaded java/lang/Comparable
[ INFO]: [Loaded java/lang/CharSequence
[ INFO]: [Loaded java/lang/String
[ INFO]: [Loaded java/lang/Cloneable
[ INFO]: jvmrust/ch10/JvmsExample.main # 0 NEW { index: 10 }
[ INFO]: jvmrust/ch10/JvmsExample.main # 3 DUP
[ INFO]: jvmrust/ch10/JvmsExample.main # 4 INVOKE_SPECIAL { index: 11 }
[ INFO]: jvmrust/ch10/JvmsExample.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch10/JvmsExample.<init> # 1 INVOKE_SPECIAL { index: 1 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch10/JvmsExample.<init> # 4 RETURN
[ INFO]: jvmrust/ch10/JvmsExample.main # 7 ASTORE_1
[ INFO]: jvmrust/ch10/JvmsExample.main # 8 ALOAD_1
[ INFO]: jvmrust/ch10/JvmsExample.main # 9 INVOKE_VIRTUAL { index: 12 }
[ INFO]: jvmrust/ch10/JvmsExample.catchOne # 0 ALOAD_0
[ INFO]: jvmrust/ch10/JvmsExample.catchOne # 1 INVOKE_VIRTUAL { index: 4 }
[ INFO]: jvmrust/ch10/JvmsExample.tryItOut # 0 RETURN
[ INFO]: jvmrust/ch10/JvmsExample.catchOne # 4 GOTO { offset: 9 }
[ INFO]: jvmrust/ch10/JvmsExample.catchOne #13 RETURN
[ INFO]: jvmrust/ch10/JvmsExample.main #12 ALOAD_1
[ INFO]: jvmrust/ch10/JvmsExample.main #13 INVOKE_VIRTUAL { index: 13 }
[ INFO]: jvmrust/ch10/JvmsExample.catchTwo # 0 ALOAD_0
[ INFO]: jvmrust/ch10/JvmsExample.catchTwo # 1 INVOKE_VIRTUAL { index: 4 }
[ INFO]: jvmrust/ch10/JvmsExample.tryItOut # 0 RETURN
[ INFO]: jvmrust/ch10/JvmsExample.catchTwo # 4 GOTO { offset: 18 }
[ INFO]: jvmrust/ch10/JvmsExample.catchTwo #22 RETURN
[ INFO]: jvmrust/ch10/JvmsExample.main #16 ALOAD_1
[ INFO]: jvmrust/ch10/JvmsExample.main #17 INVOKE_VIRTUAL { index: 14 }
[ INFO]: jvmrust/ch10/JvmsExample.nestedCatch # 0 ALOAD_0
[ INFO]: jvmrust/ch10/JvmsExample.nestedCatch # 1 INVOKE_VIRTUAL { index: 4 }
[ INFO]: jvmrust/ch10/JvmsExample.tryItOut # 0 RETURN
[ INFO]: jvmrust/ch10/JvmsExample.nestedCatch # 4 GOTO { offset: 9 }
[ INFO]: jvmrust/ch10/JvmsExample.nestedCatch #13 GOTO { offset: 9 }
[ INFO]: jvmrust/ch10/JvmsExample.nestedCatch #22 RETURN
[ INFO]: jvmrust/ch10/JvmsExample.main #20 RETURN
```

ExceptionTest1:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch10.ExceptionTest1

0!
0
1!
1
2!
2
3
```

ExceptionTest2:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch10.ExceptionTest2

0!
1
```

ArgsTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch10.ArgsTest

java.lang.IndexOutOfBoundsException: No args!
        at jvmrust.ch10.ArgsTest.bar(ArgsTest.java:21)
        at jvmrust.ch10.ArgsTest.foo(ArgsTest.java:13)
        at jvmrust.ch10.ArgsTest.main(ArgsTest.java:8)
```

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch10.ArgsTest 123

[123]
true
```

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch10.ArgsTest abc

[abc]
NumberFormatException: Not number!
```

StackTraceTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch10.StackTraceTest

java.lang.RuntimeException: OH!
        at jvmrust.ch10.StackTraceTest.bar(StackTraceTest.java:14)
        at jvmrust.ch10.StackTraceTest.foo(StackTraceTest.java:10)
        at jvmrust.ch10.StackTraceTest.main(StackTraceTest.java:6)
```
