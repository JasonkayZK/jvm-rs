# Chapter 9: 本地方法调用

编译 Java 文件：

```shell
javac resource/jvmrust/ch09/*.java
```

## 反射

GetClassTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.GetClassTest

# Output:

void
boolean
byte
char
short
int
long
float
double
java.lang.Object
jvmrust.ch09.GetClassTest
[I
[[I
[Ljava.lang.Object;
[[Ljava.lang.Object;
java.lang.Runnable
java.lang.String
[D
[Ljava.lang.String;
```

## 字符串拼接

StringBuilderTest:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.StringBuilderTest

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded java/io/Serializable
[ INFO]: [Loaded java/lang/reflect/AnnotatedElement
[ INFO]: [Loaded java/lang/reflect/GenericDeclaration
[ INFO]: [Loaded java/lang/reflect/Type
[ INFO]: [Loaded java/lang/Class
[ INFO]: [Loaded jvmrust/ch09/StringBuilderTest
[ INFO]: [Loaded java/lang/Comparable
[ INFO]: [Loaded java/lang/CharSequence
[ INFO]: [Loaded java/lang/String
[ INFO]: [Loaded java/lang/Cloneable
[ INFO]: jvmrust/ch09/StringBuilderTest.main # 0 LDC { index: 2 }
[ INFO]: jvmrust/ch09/StringBuilderTest.main # 2 ASTORE_1
[ INFO]: jvmrust/ch09/StringBuilderTest.main # 3 LDC { index: 3 }
[ INFO]: jvmrust/ch09/StringBuilderTest.main # 5 ASTORE_2
[ INFO]: jvmrust/ch09/StringBuilderTest.main # 6 NEW { index: 4 }
[ INFO]: [Loaded java/lang/Appendable
[ INFO]: [Loaded java/lang/AbstractStringBuilder
[ INFO]: [Loaded java/lang/StringBuilder
...
[ INFO]: java/lang/StringBuilder.toString #16 ARETURN
[ INFO]: jvmrust/ch09/StringBuilderTest.main #24 ASTORE_3
[ INFO]: jvmrust/ch09/StringBuilderTest.main #25 GET_STATIC { index: 8 }
[ INFO]: jvmrust/ch09/StringBuilderTest.main #28 ALOAD_3
[ INFO]: jvmrust/ch09/StringBuilderTest.main #29 INVOKE_VIRTUAL { index: 9 }
[ INFO]: [Loaded java/lang/AutoCloseable
[ INFO]: [Loaded java/io/Closeable
[ INFO]: [Loaded java/io/Flushable
[ INFO]: [Loaded java/io/OutputStream
[ INFO]: [Loaded java/io/FilterOutputStream
[ INFO]: [Loaded java/io/PrintStream
hello,world!
[ INFO]: jvmrust/ch09/StringBuilderTest.main #32 RETURN
```

StringTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.StringTest

# Output:

true
false
true
```

## Object.hashCode()

ObjectTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.ObjectTest

# Output:

32676248
jvmrust.ch09.ObjectTest@1f29998
false
true
```

HashMapTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.HashMapTest

# Output:

123
987
{abc=123, xyz=987}
```

## Object.clone()

CloneTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.CloneTest

# Output:

3.1415926
3.14
```


## 自动装箱和拆箱

BoxTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.BoxTest

# Output:

[1, 2, 3]
1
2
3
```


## 其他

PrintlnTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.PrintlnTest

# Output:

false
true
1
2
120
3
4
3.14
3.14
abc
```

ArrayListTest:

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch09.ArrayListTest

# Output:

hello
world
```
