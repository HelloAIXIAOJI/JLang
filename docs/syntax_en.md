# JsonLang Syntax Documentation

## 1. Program Structure

A JsonLang program is a JSON object with the following top-level fields:

```json
{
    "include": ["module1", "module2"],  // Optional, includes required modules
    "const": {                          // Optional, defines constants
        "CONSTANT_NAME": "value"
    },
    "program": {                        // Required, program body
        "main": {                       // Required, main function
            "body": [                   // Required, statement array
                // Statements
            ]
        },
        "custom_function": {            // Optional, custom functions
            "params": {                 // Optional, function parameters
                "param_name": "type"
            },
            "body": [                   // Required, function body
                // Statements
            ]
        }
    }
}
```

## 2. Data Types

JsonLang supports the following data types:

- Strings: `"hello"`
- Numbers: `123`, `3.14`
- Booleans: `true`, `false`
- Null: `null`
- Objects: `{"key": "value"}`
- Arrays: `[1, 2, 3]`

## 3. Variables and Constants

### 3.1 Variable Definition

```json
{"var": {"variable_name": "value"}}
```

Variable values can be any valid JSON value or variable reference.

### 3.2 Variable Reference

Use the `@var.variable_name` syntax to reference variables:

```json
{"echo": ["Current value: ", "@var.counter"]}
```

For nested data structures, use dot notation:

```json
{"echo": ["User name: ", "@var.user.profile.name"]}
```

Supports arbitrarily deep nesting, as long as each part of the path is a valid object field.

### 3.3 Constant Definition

Define constants in the top-level `const` object:

```json
"const": {
    "PI": 3.14159,
    "APP_NAME": "MyApp"
}
```

### 3.4 Constant Reference

Use the `@const.constant_name` syntax to reference constants:

```json
{"echo": ["Application name: ", "@const.APP_NAME"]}
```

## 4. Statement Types

### 4.1 Variable Assignment

```json
{"var": {"name": "value"}}
```

### 4.2 Output Statement

```json
{"echo": ["Text1", "@var.variable_name", "Text2"]}
```

### 4.3 Comments

JsonLang supports comments through the `comment` statement in two forms:

1. Single-line comment (string form):
```json
{"comment": "This is a comment"}
```

2. Multi-part comment (array form, can include variable references):
```json
{"comment": ["This is part of the comment", ", value: ", "@var.counter"]}
```

Comments are ignored by the interpreter and don't affect program execution. When running in debug mode, comment content is displayed.

Comment use cases:
- Explaining code purpose and functionality
- Documenting algorithm steps
- Marking TODOs or future improvements
- Temporarily disabling code blocks

Example:
```json
{"comment": "Calculate square root below"},
{"math.sqrt": [16]},
{"comment": ["Result is: ", "@var.result"]}
```

### 4.4 System Command Execution

Execute system commands and get their output using the `exec` statement:

```json
{"exec": {
    "cmd": "command_name",
    "args": ["arg1", "arg2", "@var.variable_arg"],
    "output": "result_variable_name"  // Optional, defaults to "result"
}}
```

The execution result is stored in the specified variable (default is `result`), and contains:
- `stdout`: Standard output (string)
- `stderr`: Standard error (string)
- `status`: Exit status code (number)

Example:
```json
// List directory contents
{"exec": {
    "cmd": "ls",
    "args": ["-la"],
    "output": "dir_content"
}},
{"echo": ["Directory contents:\n", "@var.dir_content.stdout"]}

// Execute on Windows
{"exec": {
    "cmd": "dir",
    "output": "files"
}},
{"echo": ["File list:\n", "@var.files.stdout"]}
```

Note:
1. On Windows, commands are executed via `cmd /C`
2. On Linux/macOS, commands are executed via `sh -c`
3. Be aware of security risks when using this feature; avoid executing untrusted commands

### 4.5 String Concatenation

```json
{"concat": {
    "target": "result_variable_name",
    "parts": ["String1", "@var.variable_name", "String2"]
}}
```

### 4.6 Conditional Statements

```json
{"if": {
    "condition": {
        "op": "operator",    // eq, neq, gt, lt, gte, lte
        "left": "value1",
        "right": "value2"
    },
    "then": [
        // Statements executed if condition is true
    ],
    "else": [
        // Statements executed if condition is false
    ]
}}
```

Supported comparison operators:
- `eq`: Equal
- `neq`: Not equal
- `gt`: Greater than
- `lt`: Less than
- `gte`: Greater than or equal
- `lte`: Less than or equal

### 4.7 Loop Structures

#### 4.7.1 While Loop

```json
{"while": {
    "condition": {
        "op": "operator",
        "left": "value1",
        "right": "value2"
    },
    "body": [
        // Loop body statements
    ]
}}
```

Example:
```json
{"while": {
    "condition": {
        "op": "lt",
        "left": "@var.counter",
        "right": 5
    },
    "body": [
        {"echo": ["Count: ", "@var.counter", "\n"]},
        {"math.add": ["@var.counter", 1]},
        {"var": {"counter": "@var.result"}}
    ]
}}
```

#### 4.7.2 For Loop

JsonLang supports two for loop syntaxes:

1. Range syntax (recommended):
```json
{"for": {
    "var": "loop_variable",
    "range": [start_value, end_value],
    "step": step_size,  // Optional, defaults to 1
    "body": [
        // Loop body statements
    ]
}}
```

2. Traditional syntax:
```json
{"for": {
    "var": "loop_variable",
    "from": start_value,
    "to": end_value,
    "step": step_size,  // Optional, defaults to 1
    "body": [
        // Loop body statements
    ]
}}
```

Example:
```json
// Forward loop
{"for": {
    "var": "i",
    "range": [1, 5],
    "body": [
        {"echo": ["Iteration ", "@var.i", "\n"]}
    ]
}}

// Reverse loop
{"for": {
    "var": "count",
    "range": [5, 0],
    "step": -1,
    "body": [
        {"echo": ["Countdown: ", "@var.count", "\n"]}
    ]
}}
```

### 4.8 Function Calls

Two function call syntaxes are supported:

1. Standard syntax:
```json
{"call": ["function_name", {"param_name": "param_value"}]}
```

2. Simplified syntax (recommended):
```json
{"function_name": [param1, param2, ...]}
```

### 4.9 Module Function Calls

Two module function call syntaxes are supported:

1. Standard syntax:
```json
{"call": ["module_name.function_name", param1, param2]}
```

2. Simplified syntax (recommended):
```json
{"module_name.function_name": [param1, param2, ...]}
```

Example:
```json
// Standard syntax
{"call": ["math.add", 1, 2, 3]}

// Simplified syntax
{"math.add": [1, 2, 3]}
```

## 5. Function Definition

```json
"function_name": {
    "params": {
        "param_name": "type"
    },
    "body": [
        // Function body statements
    ]
}
```

When a function executes, it creates its own local variable scope. After execution:
1. Local variables are not preserved
2. The function's return value is stored in the `result` variable
3. Non-parameter or non-temporary variables created in the function are preserved in the global scope

This mechanism ensures that function parameters and temporary variables don't pollute the global scope, while also supporting recursive function implementation.

Note:
1. Function names cannot conflict with built-in statements (var, echo, concat, if, call, while, for, comment, exec)
2. Function names cannot conflict with module function names
3. Function names cannot conflict with other defined function names

Example:
```json
"print_message": {
    "params": {
        "text": "string"
    },
    "body": [
        {"echo": ["Received message:\n"]},
        {"echo": ["@params.text", "\n"]}
    ]
}
```

## 6. Command Line Arguments

The JsonLang interpreter supports the following command line arguments:

### 6.1 Basic Usage

```bash
jsonlang [options] <program_file_path>
```

### 6.2 Debug Mode

Use the `--debug` parameter to enable debug mode, which displays additional debug information:

```bash
jsonlang --debug <program_file_path>
```

In debug mode, the interpreter outputs additional information, such as:
- for loop start, end values, and step size

Example:
```bash
# Normal mode (no debug info)
jsonlang examples/loop_test.jl

# Debug mode (shows debug info)
jsonlang --debug examples/loop_test.jl
```

## 7. Advanced Features

### 7.1 Recursive Functions

JsonLang fully supports recursive function calls, enabling complex algorithms like factorial, Fibonacci sequences, etc.

Example (recursive factorial implementation):
```json
{
    "program": {
        "factorial": {
            "params": {
                "n": "number"
            },
            "body": [
                {"if": {
                    "condition": {
                        "op": "lte",
                        "left": "@params.n",
                        "right": 1
                    },
                    "then": [
                        {"var": {"result": 1}}
                    ],
                    "else": [
                        {"math.subtract": ["@params.n", 1]},
                        {"var": {"n_minus_1": "@var.result"}},
                        {"factorial": ["@var.n_minus_1"]},
                        {"var": {"sub_result": "@var.result"}},
                        {"math.multiply": ["@params.n", "@var.sub_result"]},
                        {"var": {"result": "@var.result"}}
                    ]
                }}
            ]
        },
        "main": {
            "body": [
                {"factorial": [5]},
                {"echo": ["5! = ", "@var.result", "\n"]}
            ]
        }
    }
}
```

### 7.2 Weak Typing System

JsonLang features a weak typing system that automatically converts between types:

- String numbers ("42") convert to actual numbers in math operations
- Boolean values convert to 1 (true) or 0 (false) in numeric contexts
- Empty arrays and objects convert to 0 in numeric contexts
- Strings that can parse as numbers are treated as numbers in comparisons

Example:
```json
{"var": {"str_num": "42", "actual_num": 8}},
{"math.add": ["@var.str_num", "@var.actual_num"]},
{"echo": ["Result: ", "@var.result", "\n"]}  // Outputs "Result: 50"
```

### 7.3 Modules

JsonLang supports both built-in modules and custom modules:

#### 7.3.1 Built-in Modules

- `math`: Mathematical operations (add, subtract, multiply, divide, etc.)
- `io`: Input/output operations (read_file, write_file, input)

#### 7.3.2 Custom Modules

You can create your own `.jl` files as modules and include them in your programs.

Example:
```json
{
    "include": ["utils", "math"],
    "program": {
        "main": {
            "body": [
                {"utils.greet": ["World"]},
                {"math.add": [10, 5]},
                {"echo": ["Result: ", "@var.result", "\n"]}
            ]
        }
    }
}
``` 