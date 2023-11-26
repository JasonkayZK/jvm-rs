# jvm-rs

[![CI](https://github.com/JasonkayZK/jvm-rs/workflows/CI/badge.svg)](https://github.com/JasonkayZK/jvm-rs/actions)

A JVM implemented in rust.

Reference:

- https://github.com/zxh0/jvmgo-book/
- https://github.com/dxx/jvm-rust/

## 目录说明

本源码目录结构按照 [jvmgo-book](https://github.com/zxh0/jvmgo-book) 组织，说明如下：

* ch01_cmd: 命令行处理
* ch02_classpath: 查找 Class 文件
* ch03_classfile: 解析 Class 文件
* ch04_rtda: 运行时数据区
* ch05_instructions: 指令集和解释器
* ch06_rtda_heap: 类和对象
* ch07_method_invoke: 方法调用和返回
* ch08_array_string: 数组和字符串
* ch09_native: 本地方法调用
* ch10_exception: 异常处理

`java` 目录存放测试使用的 Java 代码。

## 运行

参考每个目录中的 `README.md`。
