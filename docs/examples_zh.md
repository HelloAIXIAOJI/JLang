# JiLang 示例程序

本文档描述了 `examples/` 目录中的示例程序，帮助您学习 JiLang 的功能特性。

## 基础示例

### [hello_world.jl](../examples/hello_world.jl)
一个简单的 JiLang 程序，演示如何输出信息和使用变量。

### [test.json](../examples/test.json)
一个基础程序，演示打印 "Hello, JiLang!" 和简单的计数循环。

## 控制流示例

### [loop_test.jl](../examples/loop_test.jl)
演示在 JiLang 中使用 `for` 和 `while` 循环。

### [weak_typing_demo.jl](../examples/weak_typing_demo.jl)
全面展示 JiLang 的弱类型系统，展示在各种操作中不同数据类型之间的自动类型转换。

## 函数示例

### [simplified_syntax_test.jl](../examples/simplified_syntax_test.jl)
展示 JiLang 中函数调用的简化语法，使用直接语法 `{"function": [args]}` 而不是 `{"call": ["function", args]}`。

### [digui.jl](../examples/digui.jl), [digui2.jl](../examples/digui2.jl), [digui3.jl](../examples/digui3.jl), [digui4.jl](../examples/digui4.jl), [digui5.jl](../examples/digui5.jl)
递归函数的多个示例，展示如何在 JiLang 中实现递归。

### [fibonacci.jl](../examples/fibonacci.jl)
使用递归计算斐波那契数列的示例。

## 模块系统

### [use_module.jl](../examples/use_module.jl)
演示如何使用 JiLang 的 `include` 指令导入其他模块。

### [utils.jl](../examples/utils.jl)
一个可重用的工具函数模块，在其他示例中使用。

### [module_test.jl](../examples/module_test.jl)
演示如何使用 JiLang 中的模块系统，使用标准模块调用语法：`{"call": ["module.function", args]}`。

### [simplified_module_test.jl](../examples/simplified_module_test.jl)
展示 JiLang 中的简化模块调用语法：`{"module.function": [args]}`。

### [comment_test.jl](../examples/comment_test.jl)
展示如何在 JiLang 中使用单行和多部分注释。

### [error_test.jl](../examples/error_test.jl)
演示 JiLang 中的错误处理和常见错误模式。

## 系统集成

### [exec_demo.jl](../examples/exec_demo.jl)
展示如何从 JiLang 执行系统命令。

### [exec_unix_demo.jl](../examples/exec_unix_demo.jl)
特定于 Unix 系统的命令执行示例。

## 高级示例

### [taow.jl](../examples/taow.jl)
一个更复杂的程序，展示多个 JiLang 特性共同工作，包括嵌套数据结构和函数。

### [uni.jl](../examples/uni.jl)
展示在 JiLang 中使用 Unicode 字符的能力。

## 最新特性示例

### [array_demo.jl](../examples/array_demo.jl)
演示 JiLang 中使用数组的各种方法，包括创建、访问和操作数组。

### [object_demo.jl](../examples/object_demo.jl)
展示 JiLang 中对象的创建和操作，包括属性访问和修改。

### [regex_demo.jl](../examples/regex_demo.jl)
演示如何在 JiLang 中使用正则表达式功能。

### [switch_demo.jl](../examples/switch_demo.jl)
展示 JiLang 中 switch 语句的增强功能。

### [error_handling_demo.jl](../examples/error_handling_demo.jl)
展示 JiLang 中的错误处理改进。

## 运行示例

使用 JiLang 解释器运行任何示例：

```bash
JiLang examples/hello_world.jl
```

要查看额外的调试信息，使用：

```bash
JiLang --debug examples/loop_test.jl
``` 