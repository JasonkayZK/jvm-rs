### Chapter 7: 方法调用和返回

编译 Java 文件：

```shell
javac resource/jvmrust/ch07/*.java
```

加载 Class 并执行 main：

AccSuperDemo:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch07.AccSuperDemo

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded jvmrust/ch07/AccSuperDemo
[ INFO]: jvmrust/ch07/AccSuperDemo.main # 0 NEW { index: 2 }
[ INFO]: [Loaded jvmrust/ch07/AccSuperDemo$A
[ INFO]: [Loaded jvmrust/ch07/AccSuperDemo$B
[ INFO]: [Loaded jvmrust/ch07/AccSuperDemo$C
[ INFO]: jvmrust/ch07/AccSuperDemo.main # 3 DUP
[ INFO]: jvmrust/ch07/AccSuperDemo.main # 4 INVOKE_SPECIAL { index: 3 }
[ INFO]: jvmrust/ch07/AccSuperDemo$C.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/AccSuperDemo$C.<init> # 1 INVOKE_SPECIAL { index: 1 }
[ INFO]: jvmrust/ch07/AccSuperDemo$B.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/AccSuperDemo$B.<init> # 1 INVOKE_SPECIAL { index: 1 }
[ INFO]: jvmrust/ch07/AccSuperDemo$A.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/AccSuperDemo$A.<init> # 1 INVOKE_SPECIAL { index: 1 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch07/AccSuperDemo$A.<init> # 4 RETURN
[ INFO]: jvmrust/ch07/AccSuperDemo$B.<init> # 4 RETURN
[ INFO]: jvmrust/ch07/AccSuperDemo$C.<init> # 4 RETURN
[ INFO]: jvmrust/ch07/AccSuperDemo.main # 7 INVOKE_VIRTUAL { index: 4 }
[ INFO]: jvmrust/ch07/AccSuperDemo$C.foo # 0 ALOAD_0
[ INFO]: jvmrust/ch07/AccSuperDemo$C.foo # 1 INVOKE_SPECIAL { index: 2 }
[ INFO]: jvmrust/ch07/AccSuperDemo$A.foo # 0 GET_STATIC { index: 2 }
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
[ INFO]: jvmrust/ch07/AccSuperDemo$A.foo # 0 GET_STATIC { index: 2 }
[ INFO]: jvmrust/ch07/AccSuperDemo$A.foo # 3 LDC { index: 3 }
thread 'main' panicked at 'TODO: ldc!', ch07_method_invoke/src/instructions/constants/ldc.rs:115:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

FibonacciTest:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch07.FibonacciTest

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded jvmrust/ch07/FibonacciTest
[ INFO]: jvmrust/ch07/FibonacciTest.main # 0 LDC2_W { index: 2 }
[ INFO]: jvmrust/ch07/FibonacciTest.main # 3 INVOKE_STATIC { index: 4 }
[ INFO]: java/lang/Object.<clinit> # 0 INVOKE_STATIC { index: 16 }
[ INFO]: java/lang/Object.<clinit> # 3 RETURN
[ INFO]: jvmrust/ch07/FibonacciTest.main # 3 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 8 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 9 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #10 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #11 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 8 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 9 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #10 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #11 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 8 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 9 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #10 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #11 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 8 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 9 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #10 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #11 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 6 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 7 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #14 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #15 LDC2_W { index: 7 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #18 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #19 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 6 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 7 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #22 LADD
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #23 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #14 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #15 LDC2_W { index: 7 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #18 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #19 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 6 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 7 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #22 LADD
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #23 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #14 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #15 LDC2_W { index: 7 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #18 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #19 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 8 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 9 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #10 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #11 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 6 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 7 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #14 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #15 LDC2_W { index: 7 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #18 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #19 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 6 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 7 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #22 LADD
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #23 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #22 LADD
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #23 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #14 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #15 LDC2_W { index: 7 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #18 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #19 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 8 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 9 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #10 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #11 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 8 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 9 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #10 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #11 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 6 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 7 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #14 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #15 LDC2_W { index: 7 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #18 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #19 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 6 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 7 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #22 LADD
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #23 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #14 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #15 LDC2_W { index: 7 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #18 LSUB
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #19 INVOKE_STATIC { index: 4 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 0 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 1 LCONST_1
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 2 LCMP
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 3 IFGT { offset: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 6 LLOAD_0
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci # 7 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #22 LADD
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #23 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #22 LADD
[ INFO]: jvmrust/ch07/FibonacciTest.fibonacci #23 LRETURN
[ INFO]: jvmrust/ch07/FibonacciTest.main # 6 LSTORE_1
[ INFO]: jvmrust/ch07/FibonacciTest.main # 7 GET_STATIC { index: 5 }
[ INFO]: [Loaded java/lang/System
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
[ INFO]: jvmrust/ch07/FibonacciTest.main # 7 GET_STATIC { index: 5 }
[ INFO]: jvmrust/ch07/FibonacciTest.main #10 LLOAD_1
[ INFO]: jvmrust/ch07/FibonacciTest.main #11 INVOKE_VIRTUAL { index: 6 }
[ INFO]: [Loaded java/lang/AutoCloseable
[ INFO]: [Loaded java/io/Closeable
[ INFO]: [Loaded java/io/Flushable
[ INFO]: [Loaded java/io/OutputStream
[ INFO]: [Loaded java/io/FilterOutputStream
[ INFO]: [Loaded java/lang/Appendable
[ INFO]: [Loaded java/io/PrintStream
5
[ INFO]: jvmrust/ch07/FibonacciTest.main #14 RETURN
```

InvokeDemo:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch07.InvokeDemo

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded java/lang/Runnable
[ INFO]: [Loaded jvmrust/ch07/InvokeDemo
[ INFO]: jvmrust/ch07/InvokeDemo.main # 0 NEW { index: 2 }
[ INFO]: jvmrust/ch07/InvokeDemo.main # 3 DUP
[ INFO]: jvmrust/ch07/InvokeDemo.main # 4 INVOKE_SPECIAL { index: 3 }
[ INFO]: jvmrust/ch07/InvokeDemo.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/InvokeDemo.<init> # 1 INVOKE_SPECIAL { index: 1 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.<init> # 4 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.main # 7 INVOKE_VIRTUAL { index: 4 }
[ INFO]: jvmrust/ch07/InvokeDemo.test # 0 INVOKE_STATIC { index: 5 }
[ INFO]: java/lang/Object.<clinit> # 0 INVOKE_STATIC { index: 16 }
[ INFO]: java/lang/Object.<clinit> # 3 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.test # 0 INVOKE_STATIC { index: 5 }
[ INFO]: jvmrust/ch07/InvokeDemo.staticMethod # 0 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.test # 3 NEW { index: 2 }
[ INFO]: jvmrust/ch07/InvokeDemo.test # 6 DUP
[ INFO]: jvmrust/ch07/InvokeDemo.test # 7 INVOKE_SPECIAL { index: 3 }
[ INFO]: jvmrust/ch07/InvokeDemo.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/InvokeDemo.<init> # 1 INVOKE_SPECIAL { index: 1 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.<init> # 4 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.test #10 ASTORE_1
[ INFO]: jvmrust/ch07/InvokeDemo.test #11 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeDemo.test #12 INVOKE_SPECIAL { index: 6 }
[ INFO]: jvmrust/ch07/InvokeDemo.instanceMethod # 0 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.test #15 ALOAD_0
[ INFO]: jvmrust/ch07/InvokeDemo.test #16 ACONST_NULL
[ INFO]: jvmrust/ch07/InvokeDemo.test #17 INVOKE_SPECIAL { index: 7 }
[ INFO]: java/lang/Object.equals # 0 ALOAD_0
[ INFO]: java/lang/Object.equals # 1 ALOAD_1
[ INFO]: java/lang/Object.equals # 2 IF_ACMPNE { offset: 7 }
[ INFO]: java/lang/Object.equals # 9 ICONST_0
[ INFO]: java/lang/Object.equals #10 IRETURN
[ INFO]: jvmrust/ch07/InvokeDemo.test #20 POP
[ INFO]: jvmrust/ch07/InvokeDemo.test #21 ALOAD_0
[ INFO]: jvmrust/ch07/InvokeDemo.test #22 INVOKE_VIRTUAL { index: 8 }
[ INFO]: jvmrust/ch07/InvokeDemo.run # 0 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.test #25 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeDemo.test #26 INVOKE_INTERFACE { index: 9 }
[ INFO]: jvmrust/ch07/InvokeDemo.run # 0 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.test #31 RETURN
[ INFO]: jvmrust/ch07/InvokeDemo.main #10 RETURN
```

InvokeInterfaceTest:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch07.InvokeInterfaceTest

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded jvmrust/ch07/InvokeInterfaceTest
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main # 0 NEW { index: 2 }
[ INFO]: [Loaded jvmrust/ch07/Vector
[ INFO]: [Loaded jvmrust/ch07/Vector2D
[ INFO]: [Loaded jvmrust/ch07/Vector3D
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main # 3 DUP
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main # 4 LDC2_W { index: 3 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main # 7 LDC2_W { index: 5 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #10 LDC2_W { index: 7 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #13 INVOKE_SPECIAL { index: 9 }
[ INFO]: jvmrust/ch07/Vector3D.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.<init> # 1 DLOAD_1
[ INFO]: jvmrust/ch07/Vector3D.<init> # 2 DLOAD_3
[ INFO]: jvmrust/ch07/Vector3D.<init> # 3 INVOKE_SPECIAL { index: 2 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 1 INVOKE_SPECIAL { index: 2 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch07/Vector2D.<init> # 4 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 5 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.<init> # 6 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 9 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> #10 DLOAD_3
[ INFO]: jvmrust/ch07/Vector2D.<init> #11 PUT_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.<init> #14 RETURN
[ INFO]: jvmrust/ch07/Vector3D.<init> # 6 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.<init> # 7 DLOAD { index: 5 }
[ INFO]: jvmrust/ch07/Vector3D.<init> # 9 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector3D.<init> #12 RETURN
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #16 ASTORE_1
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #17 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #18 LDC2_W { index: 10 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #21 INVOKE_INTERFACE { index: 12 }
[ INFO]: jvmrust/ch07/Vector3D.multiply # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.multiply # 1 DLOAD_1
[ INFO]: jvmrust/ch07/Vector3D.multiply # 2 INVOKE_SPECIAL { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.multiply # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.multiply # 1 DUP
[ INFO]: jvmrust/ch07/Vector2D.multiply # 2 GET_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.multiply # 5 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.multiply # 6 DMUL
[ INFO]: jvmrust/ch07/Vector2D.multiply # 7 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #10 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.multiply #11 DUP
[ INFO]: jvmrust/ch07/Vector2D.multiply #12 GET_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #15 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.multiply #16 DMUL
[ INFO]: jvmrust/ch07/Vector2D.multiply #17 PUT_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #20 RETURN
[ INFO]: jvmrust/ch07/Vector3D.multiply # 5 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.multiply # 6 DUP
[ INFO]: jvmrust/ch07/Vector3D.multiply # 7 GET_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector3D.multiply #10 DLOAD_1
[ INFO]: jvmrust/ch07/Vector3D.multiply #11 DMUL
[ INFO]: jvmrust/ch07/Vector3D.multiply #12 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector3D.multiply #15 RETURN
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #26 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #27 CHECK_CAST { index: 2 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #30 ASTORE_2
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #31 GET_STATIC { index: 13 }
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
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #31 GET_STATIC { index: 13 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #34 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #35 GET_FIELD { index: 14 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #38 INVOKE_VIRTUAL { index: 15 }
[ INFO]: [Loaded java/lang/AutoCloseable
[ INFO]: [Loaded java/io/Closeable
[ INFO]: [Loaded java/io/Flushable
[ INFO]: [Loaded java/io/OutputStream
[ INFO]: [Loaded java/io/FilterOutputStream
[ INFO]: [Loaded java/lang/Appendable
[ INFO]: [Loaded java/io/PrintStream
9.3
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #41 GET_STATIC { index: 13 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #44 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #45 GET_FIELD { index: 16 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #48 INVOKE_VIRTUAL { index: 15 }
9.600000000000001
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #51 GET_STATIC { index: 13 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #54 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #55 GET_FIELD { index: 17 }
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #58 INVOKE_VIRTUAL { index: 15 }
9.899999999999999
[ INFO]: jvmrust/ch07/InvokeInterfaceTest.main #61 RETURN
```

InvokeSpecialTest:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch07.InvokeSpecialTest

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded jvmrust/ch07/InvokeSpecialTest
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main # 0 NEW { index: 2 }
[ INFO]: [Loaded jvmrust/ch07/Vector
[ INFO]: [Loaded jvmrust/ch07/Vector2D
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main # 3 DUP
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main # 4 LDC2_W { index: 3 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main # 7 LDC2_W { index: 5 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #10 INVOKE_SPECIAL { index: 7 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 1 INVOKE_SPECIAL { index: 2 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch07/Vector2D.<init> # 4 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 5 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.<init> # 6 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 9 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> #10 DLOAD_3
[ INFO]: jvmrust/ch07/Vector2D.<init> #11 PUT_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.<init> #14 RETURN
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #13 ASTORE_1
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #14 GET_STATIC { index: 8 }
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
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #14 GET_STATIC { index: 8 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #17 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #18 GET_FIELD { index: 9 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #21 INVOKE_VIRTUAL { index: 10 }
[ INFO]: [Loaded java/lang/AutoCloseable
[ INFO]: [Loaded java/io/Closeable
[ INFO]: [Loaded java/io/Flushable
[ INFO]: [Loaded java/io/OutputStream
[ INFO]: [Loaded java/io/FilterOutputStream
[ INFO]: [Loaded java/lang/Appendable
[ INFO]: [Loaded java/io/PrintStream
2.1
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #24 GET_STATIC { index: 8 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #27 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #28 GET_FIELD { index: 11 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #31 INVOKE_VIRTUAL { index: 10 }
2.2
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #34 NEW { index: 12 }
[ INFO]: [Loaded jvmrust/ch07/Vector3D
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #37 DUP
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #38 LDC2_W { index: 13 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #41 LDC2_W { index: 15 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #44 LDC2_W { index: 17 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #47 INVOKE_SPECIAL { index: 19 }
[ INFO]: jvmrust/ch07/Vector3D.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.<init> # 1 DLOAD_1
[ INFO]: jvmrust/ch07/Vector3D.<init> # 2 DLOAD_3
[ INFO]: jvmrust/ch07/Vector3D.<init> # 3 INVOKE_SPECIAL { index: 2 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 1 INVOKE_SPECIAL { index: 2 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch07/Vector2D.<init> # 4 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 5 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.<init> # 6 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 9 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> #10 DLOAD_3
[ INFO]: jvmrust/ch07/Vector2D.<init> #11 PUT_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.<init> #14 RETURN
[ INFO]: jvmrust/ch07/Vector3D.<init> # 6 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.<init> # 7 DLOAD { index: 5 }
[ INFO]: jvmrust/ch07/Vector3D.<init> # 9 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector3D.<init> #12 RETURN
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #50 ASTORE_2
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #51 GET_STATIC { index: 8 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #54 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #55 GET_FIELD { index: 20 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #58 INVOKE_VIRTUAL { index: 10 }
3.1
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #61 GET_STATIC { index: 8 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #64 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #65 GET_FIELD { index: 21 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #68 INVOKE_VIRTUAL { index: 10 }
3.2
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #71 GET_STATIC { index: 8 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #74 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #75 GET_FIELD { index: 22 }
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #78 INVOKE_VIRTUAL { index: 10 }
3.3
[ INFO]: jvmrust/ch07/InvokeSpecialTest.main #81 RETURN
```

InvokeVirtualTest:

```shell
cargo run -- --verbose:class --verbose:inst --cp ./resource --xjre $JAVA_HOME/jre jvmrust.ch07.InvokeVirtualTest

# Output:

[ INFO]: [Loaded java/lang/Object
[ INFO]: [Loaded jvmrust/ch07/InvokeVirtualTest
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main # 0 NEW { index: 2 }
[ INFO]: [Loaded jvmrust/ch07/Vector
[ INFO]: [Loaded jvmrust/ch07/Vector2D
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main # 3 DUP
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main # 4 LDC2_W { index: 3 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main # 7 LDC2_W { index: 5 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #10 INVOKE_SPECIAL { index: 7 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 1 INVOKE_SPECIAL { index: 2 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch07/Vector2D.<init> # 4 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 5 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.<init> # 6 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 9 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> #10 DLOAD_3
[ INFO]: jvmrust/ch07/Vector2D.<init> #11 PUT_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.<init> #14 RETURN
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #13 ASTORE_1
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #14 NEW { index: 8 }
[ INFO]: [Loaded jvmrust/ch07/Vector3D
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #17 DUP
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #18 LDC2_W { index: 9 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #21 LDC2_W { index: 11 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #24 LDC2_W { index: 13 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #27 INVOKE_SPECIAL { index: 15 }
[ INFO]: jvmrust/ch07/Vector3D.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.<init> # 1 DLOAD_1
[ INFO]: jvmrust/ch07/Vector3D.<init> # 2 DLOAD_3
[ INFO]: jvmrust/ch07/Vector3D.<init> # 3 INVOKE_SPECIAL { index: 2 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 1 INVOKE_SPECIAL { index: 2 }
[ INFO]: java/lang/Object.<init> # 0 RETURN
[ INFO]: jvmrust/ch07/Vector2D.<init> # 4 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> # 5 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.<init> # 6 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.<init> # 9 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.<init> #10 DLOAD_3
[ INFO]: jvmrust/ch07/Vector2D.<init> #11 PUT_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.<init> #14 RETURN
[ INFO]: jvmrust/ch07/Vector3D.<init> # 6 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.<init> # 7 DLOAD { index: 5 }
[ INFO]: jvmrust/ch07/Vector3D.<init> # 9 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector3D.<init> #12 RETURN
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #30 ASTORE_2
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #31 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #32 LDC2_W { index: 16 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #35 INVOKE_VIRTUAL { index: 18 }
[ INFO]: jvmrust/ch07/Vector2D.multiply # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.multiply # 1 DUP
[ INFO]: jvmrust/ch07/Vector2D.multiply # 2 GET_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.multiply # 5 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.multiply # 6 DMUL
[ INFO]: jvmrust/ch07/Vector2D.multiply # 7 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #10 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.multiply #11 DUP
[ INFO]: jvmrust/ch07/Vector2D.multiply #12 GET_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #15 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.multiply #16 DMUL
[ INFO]: jvmrust/ch07/Vector2D.multiply #17 PUT_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #20 RETURN
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #38 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #39 LDC2_W { index: 19 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #42 INVOKE_VIRTUAL { index: 18 }
[ INFO]: jvmrust/ch07/Vector3D.multiply # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.multiply # 1 DLOAD_1
[ INFO]: jvmrust/ch07/Vector3D.multiply # 2 INVOKE_SPECIAL { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.multiply # 0 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.multiply # 1 DUP
[ INFO]: jvmrust/ch07/Vector2D.multiply # 2 GET_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.multiply # 5 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.multiply # 6 DMUL
[ INFO]: jvmrust/ch07/Vector2D.multiply # 7 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #10 ALOAD_0
[ INFO]: jvmrust/ch07/Vector2D.multiply #11 DUP
[ INFO]: jvmrust/ch07/Vector2D.multiply #12 GET_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #15 DLOAD_1
[ INFO]: jvmrust/ch07/Vector2D.multiply #16 DMUL
[ INFO]: jvmrust/ch07/Vector2D.multiply #17 PUT_FIELD { index: 4 }
[ INFO]: jvmrust/ch07/Vector2D.multiply #20 RETURN
[ INFO]: jvmrust/ch07/Vector3D.multiply # 5 ALOAD_0
[ INFO]: jvmrust/ch07/Vector3D.multiply # 6 DUP
[ INFO]: jvmrust/ch07/Vector3D.multiply # 7 GET_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector3D.multiply #10 DLOAD_1
[ INFO]: jvmrust/ch07/Vector3D.multiply #11 DMUL
[ INFO]: jvmrust/ch07/Vector3D.multiply #12 PUT_FIELD { index: 3 }
[ INFO]: jvmrust/ch07/Vector3D.multiply #15 RETURN
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #45 GET_STATIC { index: 21 }
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
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #45 GET_STATIC { index: 21 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #48 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #49 GET_FIELD { index: 22 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #52 INVOKE_VIRTUAL { index: 23 }
[ INFO]: [Loaded java/lang/AutoCloseable
[ INFO]: [Loaded java/io/Closeable
[ INFO]: [Loaded java/io/Flushable
[ INFO]: [Loaded java/io/OutputStream
[ INFO]: [Loaded java/io/FilterOutputStream
[ INFO]: [Loaded java/lang/Appendable
[ INFO]: [Loaded java/io/PrintStream
4.2
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #55 GET_STATIC { index: 21 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #58 ALOAD_1
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #59 GET_FIELD { index: 24 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #62 INVOKE_VIRTUAL { index: 23 }
4.4
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #65 GET_STATIC { index: 21 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #68 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #69 GET_FIELD { index: 22 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #72 INVOKE_VIRTUAL { index: 23 }
9.3
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #75 GET_STATIC { index: 21 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #78 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #79 GET_FIELD { index: 24 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #82 INVOKE_VIRTUAL { index: 23 }
9.600000000000001
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #85 GET_STATIC { index: 21 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #88 ALOAD_2
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #89 CHECK_CAST { index: 8 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #92 GET_FIELD { index: 25 }
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #95 INVOKE_VIRTUAL { index: 23 }
9.899999999999999
[ INFO]: jvmrust/ch07/InvokeVirtualTest.main #98 RETURN
```
