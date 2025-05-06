# JsonLang Examples

This document describes the example programs available in the `examples/` directory to help you learn JsonLang's features.

## Basic Examples

### [hello_world.jl](../examples/hello_world.jl)
A simple "Hello World" program that demonstrates basic output with the `echo` statement and basic variable usage.

### [test.json](../examples/test.json)
A basic program that demonstrates printing "Hello, JsonLang!" and a simple counting loop.

## Control Flow Examples

### [loop_test.jl](../examples/loop_test.jl)
Demonstrates the use of both `for` and `while` loops in JsonLang.

### [weak_typing_demo.jl](../examples/weak_typing_demo.jl)
A comprehensive demonstration of JsonLang's weak typing system, showing automatic type conversions between different data types in various operations.

## Function Examples

### [simplified_syntax_test.jl](../examples/simplified_syntax_test.jl)
Shows the simplified syntax for function calls in JsonLang.

### [digui.jl](../examples/digui.jl), [digui2.jl](../examples/digui2.jl), [digui3.jl](../examples/digui3.jl), [digui4.jl](../examples/digui4.jl), [digui5.jl](../examples/digui5.jl)
Various examples of recursive function implementations.

### [fibonacci.jl](../examples/fibonacci.jl)
Implementation of the Fibonacci sequence calculation using recursive functions.

## Module System Examples

### [module_test.jl](../examples/module_test.jl)
Demonstrates how to use the module system in JsonLang with standard module call syntax.

### [simplified_module_test.jl](../examples/simplified_module_test.jl)
Shows how to use the module system with simplified call syntax.

### [utils.jl](../examples/utils.jl)
An example of a utility module that can be imported by other programs.

### [use_module.jl](../examples/use_module.jl)
Demonstrates importing and using custom modules.

## Language Features

### [comment_test.jl](../examples/comment_test.jl)
Shows how to use single-line and multi-part comments in JsonLang.

### [error_test.jl](../examples/error_test.jl)
Demonstrates error handling and common error patterns in JsonLang.

## System Integration

### [exec_demo.jl](../examples/exec_demo.jl)
Shows how to execute system commands from JsonLang.

### [exec_unix_demo.jl](../examples/exec_unix_demo.jl)
Unix-specific example of executing system commands.

## Advanced Examples

### [taow.jl](../examples/taow.jl)
A more complex program demonstrating multiple JsonLang features working together.

### [uni.jl](../examples/uni.jl)
Demonstrates Unicode string handling in JsonLang.

## Running the Examples

Run any example using the JsonLang interpreter:

```bash
jsonlang examples/hello_world.jl
```

To see additional debug information, use:

```bash
jsonlang --debug examples/loop_test.jl
``` 