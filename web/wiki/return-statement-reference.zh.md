# return语句参考

`return`语句用于从函数中返回一个值并结束函数的执行。无论`return`语句在函数体中的位置如何，一旦执行到`return`语句，函数将立即停止执行并返回指定的值。

## 语法

```json
{"return": 返回值}
```

返回值可以是任何有效的JiLang值，包括：
- 字面量值（数字、字符串、布尔值、null）
- 变量引用（使用`@var.变量名`或`@param.参数名`）
- 数组
- 对象
- 函数调用结果（例如`{"math.add": [1, 2]}`）

## 基本用法

### 返回字面量值

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"result": {"get_answer": []}}},
        {"echo": ["答案是: @var.result"]}
      ]
    },
    "get_answer": {
      "params": [],
      "body": [
        {"return": 42}
      ]
    }
  }
}
```

输出：
```
答案是: 42
```

### 返回变量引用

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"name": "张三"}},
        {"var": {"greeting": {"greet": "@var.name"}}},
        {"echo": ["@var.greeting"]}
      ]
    },
    "greet": {
      "params": ["user_name"],
      "body": [
        {"return": {"concat": ["你好，", "@param.user_name", "！"]}}
      ]
    }
  }
}
```

输出：
```
你好，张三！
```

### 提前返回（早期退出）

`return`语句可以用于在满足特定条件时提前退出函数执行：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"result1": {"check_positive": 5}}},
        {"echo": ["结果1: @var.result1", "\n"]},
        {"var": {"result2": {"check_positive": -3}}},
        {"echo": ["结果2: @var.result2"]}
      ]
    },
    "check_positive": {
      "params": ["number"],
      "body": [
        {"if": {
          "condition": {"op": "lt", "left": "@param.number", "right": 0},
          "then": [
            {"return": false}
          ]
        }},
        {"echo": ["检查通过"]},
        {"return": true}
      ]
    }
  }
}
```

输出：
```
结果1: true
结果2: false
```

注意：在`check_positive`函数中，如果数字小于0，函数将立即返回`false`，不会执行后续的`echo`语句。

## 高级用法

### 返回复合数据结构

`return`语句可以返回复杂的数据结构，如对象或数组：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"user_info": {"get_user": 1001}}},
        {"echo": ["用户信息: @var.user_info"]}
      ]
    },
    "get_user": {
      "params": ["user_id"],
      "body": [
        {"return": {
          "id": "@param.user_id",
          "name": "李四",
          "role": "管理员",
          "permissions": ["read", "write", "delete"]
        }}
      ]
    }
  }
}
```

默认情况下，输出只会显示`<object>`或`<array>`，但使用`--print-full`参数可以显示完整内容（详见下文）。

### 返回函数调用结果

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"result": {"calculate": [3, 4]}}},
        {"echo": ["计算结果: @var.result"]}
      ]
    },
    "calculate": {
      "params": ["a", "b"],
      "body": [
        {"var": {"sum": {"math.add": ["@param.a", "@param.b"]}}},
        {"var": {"product": {"math.multiply": ["@param.a", "@param.b"]}}},
        {"return": {
          "sum": "@var.sum",
          "product": "@var.product",
          "difference": {"math.subtract": ["@param.a", "@param.b"]}
        }}
      ]
    }
  }
}
```

## 显示复杂数据类型

从JiLang 0.4.0版本开始，提供了`--print-full`参数来完整显示对象和数组内容。

### 标准输出（不使用`--print-full`）

当函数返回数组或对象时，默认情况下`echo`语句仅显示`<array>`或`<object>`：

```
返回对象测试: <object>
返回数组测试: <array>
嵌套对象返回测试: <object>
```

### 完整输出（使用`--print-full`）

使用`--print-full`参数运行程序时，可以查看完整的数组和对象内容：

```bash
cargo run -- 你的程序.jl --print-full
```

输出示例：
```
返回对象测试: {"id": 1001, "user": admin}
返回数组测试: [1, 2, 3]
嵌套对象返回测试: {"nested": {"value": 999}}
```

这对于调试和查看函数返回的复杂数据结构非常有用。

## 特别说明

1. 如果函数执行完毕但没有遇到`return`语句，JiLang将返回最后一个语句的执行结果，或者名为`result`的变量（如果存在）。

2. `return`语句设置的返回值优先级高于`result`变量。

3. 在函数内部，`return`语句后面的代码不会被执行。

4. `return`语句只能在函数体内使用。在主程序体中使用`return`语句没有效果。

5. 使用`--print-full`参数可以在`echo`语句中显示完整的数组和对象内容，而不是简化的`<array>`和`<object>`标记。

## 技术实现

在JiLang解释器中，`return`语句的执行涉及以下几个关键部分：

1. `execute_return_statement`函数处理`return`语句的执行，解析返回值并设置到上下文中。

2. `Context`结构体中的`is_returning`和`return_value`字段用于跟踪返回状态。

3. `execute_function`函数检查返回状态，当检测到`return`语句执行后，会中断函数体的执行。

4. 对于复杂数据类型的显示，`execute_echo_statement`函数会检查`is_print_full_values()`的返回值来决定是否显示完整内容。

## 最佳实践

1. **明确函数的返回值**：总是使用`return`语句明确指定函数的返回值，而不是依赖隐式返回。

2. **提供有意义的返回值**：函数的返回值应该有明确的含义，便于调用者理解和使用。

3. **谨慎使用提前返回**：提前返回可以简化代码逻辑，但过多的提前返回可能使函数流程变得难以理解。

4. **返回一致的数据结构**：同一个函数在不同情况下应返回一致的数据结构，避免类型不匹配。

5. **使用`--print-full`进行调试**：当函数返回复杂数据结构时，使用`--print-full`参数查看完整内容。

## 常见错误

1. 在`return`语句中引用不存在的变量。

2. 在主程序体（非函数体）中使用`return`语句。

3. 期望函数返回某种类型，但实际返回了不同类型。

4. 尝试使用`echo`语句输出复杂数据结构，但没有使用`--print-full`参数，导致只看到`<array>`或`<object>`。

## 示例程序

完整的示例程序可以参考项目示例目录中的[return_test.jl](mdc:return_test.jl)，这个示例演示了各种类型的返回值和提前返回的功能。

要查看所有返回值的完整内容，可以使用以下命令运行示例：

```bash
cargo run -- return_test.jl --print-full
``` 