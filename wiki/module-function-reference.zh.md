# JiLang模块函数与调用

本文档详细介绍了JiLang中模块函数的定义、调用方式，以及函数返回值的处理机制。

## 模块函数概述

JiLang中的函数可以通过两种方式使用：
1. 内置模块中的函数（如math、io等）
2. 自定义模块中定义的函数

JiLang使用统一的模块系统来管理这些函数，并提供灵活的调用语法。

## 模块导入

在JiLang程序中，使用`include`指令导入需要的模块：

```json
{
  "include": ["math", "io", "utils"],
  "program": {
    // 程序主体
  }
}
```

## 模块函数定义

### 自定义模块函数

在自定义模块（.jl文件）中，函数定义需要放在`program`对象内：

```json
{
  "module_meta": {
    "version": "1.0.0",
    "description": "实用工具模块",
    "author": "JiLang团队"
  },
  "program": {
    "greet": {
      "params": {
        "name": "string"
      },
      "body": [
        {"echo": ["你好，", "@params.name", "！这是来自模块的问候。\n"]}
      ]
    },
    "add": {
      "params": {
        "a": "number",
        "b": "number"
      },
      "body": [
        {"return": {"value": {"op": "add", "left": "@params.a", "right": "@params.b"}}}
      ]
    }
  }
}
```

每个函数定义包含：
- `params`：函数参数定义（对象格式）
- `body`：函数体，包含一系列语句的数组

### 程序内函数定义

在主程序内也可以定义函数，语法相同：

```json
{
  "program": {
    "main": {
      "body": [
        // 主程序语句
      ]
    },
    "calculate": {
      "params": {
        "x": "number",
        "y": "number"
      },
      "body": [
        // 函数体语句
      ]
    }
  }
}
```

## 函数调用方式

JiLang支持多种函数调用语法：

### 1. 简化模块函数调用

最常用的方式是通过点号语法直接调用模块函数：

```json
{"math.sqrt": [16]}
```

这种语法会：
1. 自动将结果存储到`@var.result`
2. 如果指定了`output`参数，还会将结果存储到指定变量

### 2. 使用对象格式调用模块函数

可以使用对象格式提供更多调用选项：

```json
{
  "math.sqrt": {
    "0": 16,
    "output": "square_root"
  }
}
```

这种格式中：
- 使用数字键（"0"、"1"等）表示位置参数
- `output`参数指定结果存储的变量名

### 3. 作为表达式调用函数

可以在变量赋值或其他表达式中调用函数：

```json
{"var": {"result": {"math.add": [5, 3]}}}
```

### 4. 在函数参数中嵌套调用

函数调用可以嵌套在其他函数的参数中：

```json
{"math.pow": [{"math.sqrt": [16]}, 2]}
```

### 5. 调用自定义函数

调用程序中定义的函数与调用模块函数类似：

```json
{"calculate": [5, 3]}
```

或者使用对象格式：

```json
{
  "calculate": {
    "0": 5,
    "1": 3,
    "output": "calc_result"
  }
}
```

## 函数返回值处理

JiLang中函数返回值的处理有几种机制：

### 1. 默认返回值存储

所有函数调用的结果默认存储在`@var.result`变量中，可以立即使用：

```json
{"math.sqrt": [16]},
{"echo": ["平方根是：", "@var.result"]}
```

### 2. 指定输出变量

使用`output`参数指定结果存储到自定义变量：

```json
{
  "math.sqrt": {
    "0": 16,
    "output": "square_root"
  }
},
{"echo": ["平方根是：", "@var.square_root"]}
```

这时结果同时存储在`@var.result`和`@var.square_root`中。

### 3. 使用return语句

在函数内部可以使用`return`语句直接返回值：

```json
{
  "return": {
    "value": "要返回的值"
  }
}
```

这会立即结束函数执行并返回指定值。例如：

```json
"factorial": {
  "params": {
    "n": null
  },
  "body": [
    {
      "if": {
        "condition": {"op": "lte", "left": "@var.n", "right": 1},
        "then": [
          {
            "return": {
              "value": 1
            }
          }
        ]
      }
    },
    // 其他语句...
  ]
}
```

### 4. 最后语句结果

如果函数没有显式`return`语句，函数执行的最后一个语句的结果会作为返回值。

### 5. 变量作为返回值

函数内定义的`result`变量会作为函数返回值：

```json
"double": {
  "params": {
    "x": null
  },
  "body": [
    {"var": {"result": {"op": "mul", "left": "@params.x", "right": 2}}}
  ]
}
```

## 函数作用域

JiLang函数有自己的作用域规则：

1. 函数参数使用`@params.名称`引用
2. 函数内定义的变量仅在函数内可见
3. 函数执行结束后，内部变量会被销毁，但以下情况除外：
   - 函数返回值存储在调用环境中
   - 函数中修改的全局变量保持其更改

## 示例

### 示例1：创建并使用模块函数

utils.jl:
```json
{
  "program": {
    "greet": {
      "params": {
        "name": "string"
      },
      "body": [
        {"echo": ["你好，", "@params.name", "！这是来自utils模块的问候。\n"]}
      ]
    },
    "count": {
      "params": {
        "start": "number",
        "end": "number"
      },
      "body": [
        {"for": {
          "var": "i",
          "range": ["@params.start", "@params.end"],
          "body": [
            {"echo": ["数字：", "@var.i", "\n"]}
          ]
        }}
      ]
    }
  }
}
```

主程序：
```json
{
  "include": ["utils"],
  "program": {
    "main": {
      "body": [
        {"utils.greet": ["世界"]},
        {"utils.count": [1, 5]}
      ]
    }
  }
}
```

### 示例2：带返回值的函数调用

```json
{
  "program": {
    "factorial": {
      "params": {
        "n": null
      },
      "body": [
        {
          "if": {
            "condition": {"op": "lte", "left": "@params.n", "right": 1},
            "then": [
              {
                "return": {
                  "value": 1
                }
              }
            ]
          }
        },
        {
          "var": {
            "recursiveResult": {
              "factorial": [{"op": "sub", "left": "@params.n", "right": 1}]
            }
          }
        },
        {
          "return": {
            "value": {"op": "mul", "left": "@params.n", "right": "@var.recursiveResult"}
          }
        }
      ]
    },
    "main": {
      "body": [
        {
          "factorial": {
            "0": 5,
            "output": "fact_result"
          }
        },
        {"echo": ["5的阶乘是：", "@var.fact_result"]}
      ]
    }
  }
}
```

## 最佳实践

1. **明确参数类型**：在函数定义中指定参数类型，提高代码可读性
2. **使用output参数**：使用output参数将结果存储到有意义的变量名中
3. **使用return语句**：对于复杂函数，使用显式return语句使流程更清晰
4. **避免副作用**：尽量避免函数修改全局状态，除非有明确意图

## 注意事项

1. 函数名不能与内置语句名冲突（如"var"、"echo"、"if"等）
2. 函数调用时参数数量必须与函数定义匹配
3. 递归函数调用支持，但需注意避免无限递归
4. 模块函数与程序函数可以同名，但会优先使用程序中定义的函数 