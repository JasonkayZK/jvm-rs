# **Chapter 3: 解析 Class 文件**

## **解析标准库**

执行：

```shell
cargo run -- --cp $JAVA_HOME/jre --xjre $JAVA_HOME/jre java.lang.String

# Output:

classpath: /Library/Java/JavaVirtualMachines/jdk1.8.0_361.jdk/Contents/Home/jre class: java.lang.String args: []
version: 52.0
constants count: 550
access flags: 0x31
this class: java/lang/String
super class: java/lang/Object
interfaces: ["java/io/Serializable", "java/lang/Comparable", "java/lang/CharSequence"]
fields count: 5
 value
 hash
 serialVersionUID
 serialPersistentFields
 CASE_INSENSITIVE_ORDER
methods count: 94
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 checkBounds
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 length
 isEmpty
 charAt
 codePointAt
 codePointBefore
 codePointCount
 offsetByCodePoints
 getChars
 getChars
 getBytes
 getBytes
 getBytes
 getBytes
 equals
 contentEquals
 nonSyncContentEquals
 contentEquals
 equalsIgnoreCase
 compareTo
 compareToIgnoreCase
 regionMatches
 regionMatches
 startsWith
 startsWith
 endsWith
 hashCode
 indexOf
 indexOf
 indexOfSupplementary
 lastIndexOf
 lastIndexOf
 lastIndexOfSupplementary
 indexOf
 indexOf
 indexOf
 indexOf
 lastIndexOf
 lastIndexOf
 lastIndexOf
 lastIndexOf
 substring
 substring
 subSequence
 concat
 replace
 matches
 contains
 replaceFirst
 replaceAll
 replace
 split
 split
 join
 join
 toLowerCase
 toLowerCase
 toUpperCase
 toUpperCase
 trim
 toString
 toCharArray
 format
 format
 valueOf
 valueOf
 valueOf
 copyValueOf
 copyValueOf
 valueOf
 valueOf
 valueOf
 valueOf
 valueOf
 valueOf
 intern
 compareTo
 <clinit>
```

## **解析自定义类**

编译 Java 文件：

```shell
javac resource/ClassFileTest.java
```

解析 Class 文件：

```shell
cargo run -- --cp ./resource --xjre $JAVA_HOME/jre ClassFileTest

# Output:

classpath: /Users/zk/workspace/jvm-rs/ch03_classfile/resource class: ClassFileTest args: []
version: 52.0
constants count: 59
access flags: 0x21
this class: jvmrust/ch03/ClassFileTest
super class: java/lang/Object
interfaces: []
fields count: 8
 FLAG
 BYTE
 X
 SHORT
 INT
 LONG
 PI
 E
methods count: 2
 <init>
 main
```

> **注：报错 `Warning: reading class err: Read class failed: "Read class ClassFileTest.class err: specified file not found in archive"`**
> 
> 表示在 lib、lib/ext 等目录中没有找到这个 Class 的提示，忽略即可！
