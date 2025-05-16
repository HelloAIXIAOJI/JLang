 # 变量引用高级功能

本文档介绍JiLang变量引用系统的高级功能和技巧，包括错误处理、变量引用的内部工作原理以及与其他语言特性的交互。

## 错误处理机制

JiLang提供了两种处理变量引用错误的方式：

1. **静默失败**：使用`resolve_value`方法，如果变量不存在则返回`null`
2. **显式错误**：使用`resolve_value_with_error`方法，如果变量不存在则返回特定错误

### 静默失败示例

当使用基本变量引用时，如果引用的变量不存在，通常会返回`null`而不是报错：

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": ["不存在的变量值: ", "@var.不存在", "\n"]},
        {"echo": ["执行会继续，因为错误被静默处理\n"]}
      ]
    }
  }
}
```

### 显式错误处理

可以结合条件语句检查变量是否存在，以便进行适当的错误处理：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"可能不存在": null}},
        {"if": {
          "condition": {
            "op": "eq",
            "left": "@var.可能不存在",
            "right": null
          },
          "then": [
            {"echo": ["警告：变量'可能不存在'的值为null\n"]}
          ],
          "else": [
            {"echo": ["变量值: ", "@var.可能不存在", "\n"]}
          ]
        }}
      ]
    }
  }
}
```

## 不同引用类型的行为差异

不同类型的引用在缺少值时有不同的行为：

1. **变量引用** (`@var.`): 缺少时返回`null`
2. **参数引用** (`@params.`): 在严格模式下，缺少时抛出错误；否则返回`null`
3. **常量引用** (`@const.`): 在严格模式下，缺少时抛出错误；否则返回`null`
4. **环境变量引用** (`@env.`): 缺少时返回`null`

## 变量引用在表达式中的使用

变量引用可以在各种表达式中使用，包括：

### 条件表达式

```json
{
  "if": {
    "condition": {
      "op": "gt",
      "left": "@var.count",
      "right": 10
    },
    "then": [
      {"echo": ["计数大于10\n"]}
    ],
    "else": [
      {"echo": ["计数小于或等于10\n"]}
    ]
  }
}
```

### 循环控制

```json
{
  "var": {"i": 0},
  "while": {
    "condition": {
      "op": "lt",
      "left": "@var.i",
      "right": 5
    },
    "body": [
      {"echo": ["循环计数: ", "@var.i", "\n"]},
      {"math.add": ["@var.i", 1]},
      {"var": {"i": "@var.result"}}
    ]
  }
}
```

### 函数参数

```json
{
  "var": {"x": 5, "y": 3},
  "math.add": ["@var.x", "@var.y"],
  "echo": ["结果: ", "@var.result", "\n"]
}
```

## 变量引用的内部实现细节

在JiLang内部，变量引用通过`VariableReference`结构体处理，主要过程包括：

1. 解析引用字符串，确定引用类型和名称
2. 根据引用类型从相应存储中查找值
3. 对于嵌套属性，使用`get_nested_value`方法递归解析和查找
4. 根据上下文返回值或错误

## 性能考虑

使用变量引用时，应注意以下性能相关事项：

1. **嵌套深度**：过深的嵌套属性访问会增加解析开销
2. **引用缓存**：频繁访问同一引用考虑将值缓存到本地变量
3. **错误处理**：尽量使用条件检查而非异常处理，以提高性能

## 与其他语言特性的交互

### 与函数调用的交互

变量引用可以用作函数参数，也可以存储函数返回值：

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"var": {"num1": 10, "num2": 20}},
        {"math.add": ["@var.num1", "@var.num2"]},
        {"var": {"sum": "@var.result"}},
        {"math.multiply": ["@var.sum", 2]},
        {"echo": ["(", "@var.num1", " + ", "@var.num2", ") * 2 = ", "@var.result", "\n"]}
      ]
    }
  }
}
```

### 与模块系统的交互

变量引用可以访问模块设置的变量和元数据：

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"echo": ["数学模块版本: ", "@var.module_meta.math.version", "\n"]}
      ]
    }
  }
}
```

## 相关链接

- [变量引用基础](variable-references-basics.zh.md) - 了解JiLang中变量引用的基本概念
- [嵌套属性访问](nested-property-access.zh.md) - 学习如何访问嵌套对象的属性