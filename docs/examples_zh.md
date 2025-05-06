# JsonLang 示例程序

本文档描述了 `examples/` 目录中可用的示例程序，帮助您学习JsonLang的功能。

## 基础示例

### [hello_world.jl](../examples/hello_world.jl)
一个简单的"Hello World"程序，演示了使用 `echo` 语句的基本输出和基本变量用法。

### [test.json](../examples/test.json)
一个基础程序，演示打印"Hello, JsonLang!"和一个简单的计数循环。

## 控制流示例

### [loop_test.jl](../examples/loop_test.jl)
演示在JsonLang中使用 `for` 和 `while` 循环。

### [weak_typing_demo.jl](../examples/weak_typing_demo.jl)
全面演示JsonLang的弱类型系统，展示了不同数据类型在各种操作中的自动类型转换。

## 函数示例

### [simplified_syntax_test.jl](../examples/simplified_syntax_test.jl)
展示了JsonLang中函数调用的简化语法。

### [digui.jl](../examples/digui.jl), [digui2.jl](../examples/digui2.jl), [digui3.jl](../examples/digui3.jl), [digui4.jl](../examples/digui4.jl), [digui5.jl](../examples/digui5.jl)
各种递归函数实现的示例。

### [fibonacci.jl](../examples/fibonacci.jl)
使用递归函数实现斐波那契数列计算。

## 模块系统示例

### [module_test.jl](../examples/module_test.jl)
演示如何使用标准模块调用语法的JsonLang模块系统。

### [simplified_module_test.jl](../examples/simplified_module_test.jl)
展示如何使用简化调用语法的模块系统。

### [utils.jl](../examples/utils.jl)
可以被其他程序导入的实用工具模块示例。

### [use_module.jl](../examples/use_module.jl)
演示导入和使用自定义模块。

## 语言特性

### [comment_test.jl](../examples/comment_test.jl)
展示如何在JsonLang中使用单行和多部分注释。

### [error_test.jl](../examples/error_test.jl)
演示JsonLang中的错误处理和常见错误模式。

## 系统集成

### [exec_demo.jl](../examples/exec_demo.jl)
展示如何从JsonLang执行系统命令。

### [exec_unix_demo.jl](../examples/exec_unix_demo.jl)
执行系统命令的Unix特定示例。

## 高级示例

### [taow.jl](../examples/taow.jl)
一个更复杂的程序，展示多个JsonLang功能共同工作。

### [uni.jl](../examples/uni.jl)
演示JsonLang中的Unicode字符串处理。

## 运行示例

使用JsonLang解释器运行任何示例：

```bash
jsonlang examples/hello_world.jl
```

要查看额外的调试信息，请使用：

```bash
jsonlang --debug examples/loop_test.jl
``` 