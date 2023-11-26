### Chapter 1: 命令行工具

运行：

Help:

```shell
cargo run -- -h
Usage: ch01_cmd --classpath <CLASSPATH> <CLASS> [ARGS]...

Arguments:
  <CLASS>    Main class name
  [ARGS]...  Arguments

Options:
      --classpath <CLASSPATH>  The classpath
  -h, --help                   Print help
  -V, --version                Print version
```

Version:

```shell
cargo run -- -V

java 0.0.1
```

Classpath:

```shell
cargo run -- --cp foo/bar MyApp arg1 arg2

classpath: foo/bar class: MyApp args: ["arg1", "arg2"]
```
