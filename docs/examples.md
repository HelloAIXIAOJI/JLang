# JiLang Examples

This document contains examples of JiLang programs that demonstrate various language features. These examples are designed to help you understand how to use JiLang effectively.

## Hello World

The simplest JiLang program:

```json
{
    "program": {
        "main": {
            "body": [
                {"echo": ["Hello, World!\n"]}
            ]
        }
    }
}
```

To run this program, save it to a file (e.g., `hello.jl`) and execute:

```bash
JiLang hello.jl
```

## Variables and Constants

```json
{
    "const": {
        "PI": 3.14159,
        "APP_NAME": "My JiLang App"
    },
    "program": {
        "main": {
            "body": [
                {"var": {"x": 10, "y": 20}},
                {"echo": ["x = ", "@var.x", ", y = ", "@var.y", "\n"]},
                {"echo": ["PI = ", "@const.PI", "\n"]},
                {"echo": ["App: ", "@const.APP_NAME", "\n"]},
                {"var": {"x": 100}},
                {"echo": ["New x = ", "@var.x", "\n"]}
            ]
        }
    }
}
```

## Conditional Statements

```json
{
    "program": {
        "main": {
            "body": [
                {"var": {"age": 25}},
                {"if": {
                    "condition": {
                        "op": "gte",
                        "left": "@var.age",
                        "right": 18
                    },
                    "then": [
                        {"echo": ["You are an adult.\n"]}
                    ],
                    "else": [
                        {"echo": ["You are a minor.\n"]}
                    ]
                }}
            ]
        }
    }
}
```

## Loops

```json
{
    "program": {
        "main": {
            "body": [
                {"var": {"counter": 1}},
                {"while": {
                    "condition": {
                        "op": "lte",
                        "left": "@var.counter",
                        "right": 5
                    },
                    "body": [
                        {"echo": ["Counter: ", "@var.counter", "\n"]},
                        {"var": {"counter": {"op": "add", "left": "@var.counter", "right": 1}}}
                    ]
                }}
            ]
        }
    }
}
```

For more examples, see the `examples` directory in the JiLang repository.

## Basic Examples

### [hello_world.jl](../examples/hello_world.jl)
A simple "Hello World" program that demonstrates basic output with the `echo` statement and basic variable usage.

### [test.json](../examples/test.json)
A basic program that demonstrates printing "Hello, JiLang!" and a simple counting loop.

## Control Flow Examples

### [loop_test.jl](../examples/loop_test.jl)
Demonstrates the use of both `for` and `while` loops in JiLang.

### [weak_typing_demo.jl](../examples/weak_typing_demo.jl)
A comprehensive demonstration of JiLang's weak typing system, showing automatic type conversions between different data types in various operations.

## Function Examples

### [simplified_syntax_test.jl](../examples/simplified_syntax_test.jl)
Shows the simplified syntax for function calls in JiLang, using direct syntax `{"function": [args]}` instead of `{"call": ["function", args]}`.

### [digui.jl](../examples/digui.jl), [digui2.jl](../examples/digui2.jl), [digui3.jl](../examples/digui3.jl), [digui4.jl](../examples/digui4.jl), [digui5.jl](../examples/digui5.jl)
Various examples of recursive function implementations, particularly focusing on the factorial calculation.

### [fibonacci.jl](../examples/fibonacci.jl)
Implementation of the Fibonacci sequence calculation using recursive functions.

## Module System Examples

### [module_test.jl](../examples/module_test.jl)
Demonstrates how to use the module system in JiLang with standard module call syntax: `{"call": ["module.function", args]}`.

### [simplified_module_test.jl](../examples/simplified_module_test.jl)
Shows how to use the module system with simplified call syntax: `{"module.function": [args]}`.

### [utils.jl](../examples/utils.jl)
An example of a utility module that can be imported by other programs.

### [use_module.jl](../examples/use_module.jl)
Demonstrates importing and using custom modules.

## Language Features

### [comment_test.jl](../examples/comment_test.jl)
Shows how to use single-line and multi-part comments in JiLang.

### [error_test.jl](../examples/error_test.jl)
Demonstrates error handling and common error patterns in JiLang.

## System Integration

### [exec_demo.jl](../examples/exec_demo.jl)
Shows how to execute system commands from JiLang.

### [exec_unix_demo.jl](../examples/exec_unix_demo.jl)
Unix-specific example of executing system commands.

## Advanced Examples

### [taow.jl](../examples/taow.jl)
A more complex program demonstrating multiple JiLang features working together, including nested data structures and functions.

### [uni.jl](../examples/uni.jl)
Demonstrates an infinite loop implementation, useful for understanding how to create and (manually) terminate long-running processes.

## Running the Examples

Run any example using the JiLang interpreter:

```bash
JiLang examples/hello_world.jl
```

To see additional debug information, use:

```bash
JiLang --debug examples/loop_test.jl
``` 