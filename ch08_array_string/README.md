# Chapter 8: 数组和字符串

编译 Java 文件：

```shell
javac resource/jvmrust/ch08/*.java
```

## Array

加载 Class 并执行 main：

ArrayDemo:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch08.ArrayDemo

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded jvmrust/ch08/ArrayDemo
[ INFO]: [Loaded java/io/Serializable
[ INFO]: [Loaded java/lang/Comparable
[ INFO]: [Loaded java/lang/CharSequence
[ INFO]: [Loaded java/lang/String
[ INFO]: [Loaded java/lang/Cloneable
[ INFO]: jvmrust/ch08/ArrayDemo.main # 0 BIPUSH { val: 10 }
[ INFO]: jvmrust/ch08/ArrayDemo.main # 2 NEW_ARRAY { atype: 10 }
[ INFO]: jvmrust/ch08/ArrayDemo.main # 4 ASTORE_1
[ INFO]: jvmrust/ch08/ArrayDemo.main # 5 BIPUSH { val: 10 }
[ INFO]: jvmrust/ch08/ArrayDemo.main # 7 ANEW_ARRAY { index: 2 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #10 ASTORE_2
[ INFO]: jvmrust/ch08/ArrayDemo.main #11 BIPUSH { val: 10 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #13 BIPUSH { val: 10 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #15 MULTI_ANEW_ARRAY { index: 3, dimensions: 2 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #19 ASTORE_3
[ INFO]: jvmrust/ch08/ArrayDemo.main #20 ALOAD_1
[ INFO]: jvmrust/ch08/ArrayDemo.main #21 ARRAY_LENGTH
[ INFO]: jvmrust/ch08/ArrayDemo.main #22 ISTORE { index: 4 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #24 ALOAD_1
[ INFO]: jvmrust/ch08/ArrayDemo.main #25 ICONST_0
[ INFO]: jvmrust/ch08/ArrayDemo.main #26 BIPUSH { val: 100 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #28 IASTORE
[ INFO]: jvmrust/ch08/ArrayDemo.main #29 ALOAD_1
[ INFO]: jvmrust/ch08/ArrayDemo.main #30 ICONST_0
[ INFO]: jvmrust/ch08/ArrayDemo.main #31 IALOAD
[ INFO]: jvmrust/ch08/ArrayDemo.main #32 ISTORE { index: 5 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #34 ALOAD_2
[ INFO]: jvmrust/ch08/ArrayDemo.main #35 ICONST_0
[ INFO]: jvmrust/ch08/ArrayDemo.main #36 LDC { index: 4 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #38 AASTORE
[ INFO]: jvmrust/ch08/ArrayDemo.main #39 ALOAD_2
[ INFO]: jvmrust/ch08/ArrayDemo.main #40 ICONST_0
[ INFO]: jvmrust/ch08/ArrayDemo.main #41 AALOAD
[ INFO]: jvmrust/ch08/ArrayDemo.main #42 ASTORE { index: 6 }
[ INFO]: jvmrust/ch08/ArrayDemo.main #44 RETURN
```

Array3D:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch08.Array3D

# Output:

0
1
2
1
2
3
2
3
4
3
4
5
1
2
3
2
3
4
3
4
5
...
```

BubbleSortTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch08.BubbleSortTest

# Output:

9
10
11
22
24
36
36
48
56
65
77
78
84
92
95
97
```

MultianewarrayDemo:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch08.MultianewarrayDemo

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded jvmrust/ch08/MultianewarrayDemo
[ INFO]: [Loaded java/io/Serializable
[ INFO]: [Loaded java/lang/Comparable
[ INFO]: [Loaded java/lang/CharSequence
[ INFO]: [Loaded java/lang/String
[ INFO]: [Loaded java/lang/Cloneable
[ INFO]: jvmrust/ch08/MultianewarrayDemo.main # 0 NEW { index: 2 }
[ INFO]: jvmrust/ch08/MultianewarrayDemo.main # 3 DUP
[ INFO]: jvmrust/ch08/MultianewarrayDemo.main # 4 INVOKE_SPECIAL { index: 3 }
[ INFO]: jvmrust/ch08/MultianewarrayDemo.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch08/MultianewarrayDemo.<init> # 1 INVOKE_SPECIAL { index: 1 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch08/MultianewarrayDemo.<init> # 4 RETURN
[ INFO]: jvmrust/ch08/MultianewarrayDemo.main # 7 INVOKE_VIRTUAL { index: 4 }
[ INFO]: jvmrust/ch08/MultianewarrayDemo.test # 0 ICONST_3
[ INFO]: jvmrust/ch08/MultianewarrayDemo.test # 1 ICONST_4
[ INFO]: jvmrust/ch08/MultianewarrayDemo.test # 2 ICONST_5
[ INFO]: jvmrust/ch08/MultianewarrayDemo.test # 3 MULTI_ANEW_ARRAY { index: 5, dimensions: 3 }
[ INFO]: jvmrust/ch08/MultianewarrayDemo.test # 7 ASTORE_1
[ INFO]: jvmrust/ch08/MultianewarrayDemo.test # 8 RETURN
[ INFO]: jvmrust/ch08/MultianewarrayDemo.main #10 RETURN
```

## String

StringTest:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch08.StringTest

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded java/lang/Cloneable
[ INFO]: [Loaded java/io/Serializable
[ INFO]: [Loaded java/lang/Comparable
[ INFO]: [Loaded java/lang/CharSequence
[ INFO]: [Loaded java/lang/String
[ INFO]: [Loaded jvmrust/ch08/StringTest
[ INFO]: jvmrust/ch08/StringTest.main # 0 LDC { index: 2 }
[ INFO]: jvmrust/ch08/StringTest.main # 2 ASTORE_1
[ INFO]: jvmrust/ch08/StringTest.main # 3 GET_STATIC { index: 3 }
[ INFO]: [Loaded java/lang/System
[ INFO]: java/lang/Object.<clinit> # 0 INVOKE_STATIC { index: 16 }
[ INFO]: java/lang/Object.<clinit> # 3 RETURN
[ INFO]: java/lang/System.<clinit> # 0 INVOKE_STATIC { index: 102 }
[ INFO]: java/lang/System.<clinit> # 3 ACONST_NULL
[ INFO]: java/lang/System.<clinit> # 4 PUT_STATIC { index: 103 }
[ INFO]: java/lang/System.<clinit> # 7 ACONST_NULL
[ INFO]: java/lang/System.<clinit> # 8 PUT_STATIC { index: 104 }
[ INFO]: java/lang/System.<clinit> #11 ACONST_NULL
[ INFO]: java/lang/System.<clinit> #12 PUT_STATIC { index: 105 }
[ INFO]: java/lang/System.<clinit> #15 ACONST_NULL
[ INFO]: java/lang/System.<clinit> #16 PUT_STATIC { index: 27 }
[ INFO]: java/lang/System.<clinit> #19 ACONST_NULL
[ INFO]: java/lang/System.<clinit> #20 PUT_STATIC { index: 6 }
[ INFO]: java/lang/System.<clinit> #23 RETURN
[ INFO]: jvmrust/ch08/StringTest.main # 3 GET_STATIC { index: 3 }
[ INFO]: jvmrust/ch08/StringTest.main # 6 LDC { index: 5 }
[ INFO]: jvmrust/ch08/StringTest.main # 8 INVOKE_VIRTUAL { index: 6 }
[ INFO]: [Loaded java/lang/AutoCloseable
[ INFO]: [Loaded java/io/Closeable
[ INFO]: [Loaded java/io/Flushable
[ INFO]: [Loaded java/io/OutputStream
[ INFO]: [Loaded java/io/FilterOutputStream
[ INFO]: [Loaded java/lang/Appendable
[ INFO]: [Loaded java/io/PrintStream
abc
[ INFO]: jvmrust/ch08/StringTest.main #11 GET_STATIC { index: 3 }
[ INFO]: jvmrust/ch08/StringTest.main #14 ALOAD_1
[ INFO]: jvmrust/ch08/StringTest.main #15 INVOKE_VIRTUAL { index: 6 }
xyz
[ INFO]: jvmrust/ch08/StringTest.main #18 RETURN
```

PrintArgs:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch08.PrintArgs 123 abc 你好 こんにちは

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded jvmrust/ch08/PrintArgs
[ INFO]: [Loaded java/io/Serializable
[ INFO]: [Loaded java/lang/Comparable
[ INFO]: [Loaded java/lang/CharSequence
[ INFO]: [Loaded java/lang/String
[ INFO]: [Loaded java/lang/Cloneable
[ INFO]: jvmrust/ch08/PrintArgs.main # 0 ALOAD_0
[ INFO]: jvmrust/ch08/PrintArgs.main # 1 ASTORE_1
[ INFO]: jvmrust/ch08/PrintArgs.main # 2 ALOAD_1
[ INFO]: jvmrust/ch08/PrintArgs.main # 3 ARRAY_LENGTH
[ INFO]: jvmrust/ch08/PrintArgs.main # 4 ISTORE_2
[ INFO]: jvmrust/ch08/PrintArgs.main # 5 ICONST_0
[ INFO]: jvmrust/ch08/PrintArgs.main # 6 ISTORE_3
[ INFO]: jvmrust/ch08/PrintArgs.main # 7 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main # 8 ILOAD_2
[ INFO]: jvmrust/ch08/PrintArgs.main # 9 IF_ICMPGE { offset: 22 }
[ INFO]: jvmrust/ch08/PrintArgs.main #12 ALOAD_1
[ INFO]: jvmrust/ch08/PrintArgs.main #13 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main #14 AALOAD
[ INFO]: jvmrust/ch08/PrintArgs.main #15 ASTORE { index: 4 }
[ INFO]: jvmrust/ch08/PrintArgs.main #17 GET_STATIC { index: 2 }
[ INFO]: [Loaded java/lang/System
[ INFO]: java/lang/Object.<clinit> # 0 INVOKE_STATIC { index: 16 }
[ INFO]: java/lang/Object.<clinit> # 3 RETURN
[ INFO]: java/lang/System.<clinit> # 0 INVOKE_STATIC { index: 102 }
[ INFO]: java/lang/System.<clinit> # 3 ACONST_NULL
[ INFO]: java/lang/System.<clinit> # 4 PUT_STATIC { index: 103 }
[ INFO]: java/lang/System.<clinit> # 7 ACONST_NULL
[ INFO]: java/lang/System.<clinit> # 8 PUT_STATIC { index: 104 }
[ INFO]: java/lang/System.<clinit> #11 ACONST_NULL
[ INFO]: java/lang/System.<clinit> #12 PUT_STATIC { index: 105 }
[ INFO]: java/lang/System.<clinit> #15 ACONST_NULL
[ INFO]: java/lang/System.<clinit> #16 PUT_STATIC { index: 27 }
[ INFO]: java/lang/System.<clinit> #19 ACONST_NULL
[ INFO]: java/lang/System.<clinit> #20 PUT_STATIC { index: 6 }
[ INFO]: java/lang/System.<clinit> #23 RETURN
[ INFO]: jvmrust/ch08/PrintArgs.main #17 GET_STATIC { index: 2 }
[ INFO]: jvmrust/ch08/PrintArgs.main #20 ALOAD { index: 4 }
[ INFO]: jvmrust/ch08/PrintArgs.main #22 INVOKE_VIRTUAL { index: 3 }
[ INFO]: [Loaded java/lang/AutoCloseable
[ INFO]: [Loaded java/io/Closeable
[ INFO]: [Loaded java/io/Flushable
[ INFO]: [Loaded java/io/OutputStream
[ INFO]: [Loaded java/io/FilterOutputStream
[ INFO]: [Loaded java/lang/Appendable
[ INFO]: [Loaded java/io/PrintStream
123
[ INFO]: jvmrust/ch08/PrintArgs.main #25 IINC { index: 3, const_val: 1 }
[ INFO]: jvmrust/ch08/PrintArgs.main #28 GOTO { offset: -21 }
[ INFO]: jvmrust/ch08/PrintArgs.main # 7 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main # 8 ILOAD_2
[ INFO]: jvmrust/ch08/PrintArgs.main # 9 IF_ICMPGE { offset: 22 }
[ INFO]: jvmrust/ch08/PrintArgs.main #12 ALOAD_1
[ INFO]: jvmrust/ch08/PrintArgs.main #13 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main #14 AALOAD
[ INFO]: jvmrust/ch08/PrintArgs.main #15 ASTORE { index: 4 }
[ INFO]: jvmrust/ch08/PrintArgs.main #17 GET_STATIC { index: 2 }
[ INFO]: jvmrust/ch08/PrintArgs.main #20 ALOAD { index: 4 }
[ INFO]: jvmrust/ch08/PrintArgs.main #22 INVOKE_VIRTUAL { index: 3 }
abc
[ INFO]: jvmrust/ch08/PrintArgs.main #25 IINC { index: 3, const_val: 1 }
[ INFO]: jvmrust/ch08/PrintArgs.main #28 GOTO { offset: -21 }
[ INFO]: jvmrust/ch08/PrintArgs.main # 7 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main # 8 ILOAD_2
[ INFO]: jvmrust/ch08/PrintArgs.main # 9 IF_ICMPGE { offset: 22 }
[ INFO]: jvmrust/ch08/PrintArgs.main #12 ALOAD_1
[ INFO]: jvmrust/ch08/PrintArgs.main #13 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main #14 AALOAD
[ INFO]: jvmrust/ch08/PrintArgs.main #15 ASTORE { index: 4 }
[ INFO]: jvmrust/ch08/PrintArgs.main #17 GET_STATIC { index: 2 }
[ INFO]: jvmrust/ch08/PrintArgs.main #20 ALOAD { index: 4 }
[ INFO]: jvmrust/ch08/PrintArgs.main #22 INVOKE_VIRTUAL { index: 3 }
你好
[ INFO]: jvmrust/ch08/PrintArgs.main #25 IINC { index: 3, const_val: 1 }
[ INFO]: jvmrust/ch08/PrintArgs.main #28 GOTO { offset: -21 }
[ INFO]: jvmrust/ch08/PrintArgs.main # 7 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main # 8 ILOAD_2
[ INFO]: jvmrust/ch08/PrintArgs.main # 9 IF_ICMPGE { offset: 22 }
[ INFO]: jvmrust/ch08/PrintArgs.main #12 ALOAD_1
[ INFO]: jvmrust/ch08/PrintArgs.main #13 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main #14 AALOAD
[ INFO]: jvmrust/ch08/PrintArgs.main #15 ASTORE { index: 4 }
[ INFO]: jvmrust/ch08/PrintArgs.main #17 GET_STATIC { index: 2 }
[ INFO]: jvmrust/ch08/PrintArgs.main #20 ALOAD { index: 4 }
[ INFO]: jvmrust/ch08/PrintArgs.main #22 INVOKE_VIRTUAL { index: 3 }
こんにちは
[ INFO]: jvmrust/ch08/PrintArgs.main #25 IINC { index: 3, const_val: 1 }
[ INFO]: jvmrust/ch08/PrintArgs.main #28 GOTO { offset: -21 }
[ INFO]: jvmrust/ch08/PrintArgs.main # 7 ILOAD_3
[ INFO]: jvmrust/ch08/PrintArgs.main # 8 ILOAD_2
[ INFO]: jvmrust/ch08/PrintArgs.main # 9 IF_ICMPGE { offset: 22 }
[ INFO]: jvmrust/ch08/PrintArgs.main #31 RETURN
```
