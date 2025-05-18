# 编写JiLang原生模块

本文档详细介绍了如何创建和使用JiLang原生的`.jl`模块文件。

## 原生模块概述

JiLang原生模块是使用JiLang自身语言编写的模块，以`.jl`为文件扩展名。这些模块可以：

1. 封装可重用的函数代码
2. 定义模块元数据
3. 组织相关功能
4. 在不同JiLang程序中共享

与使用Lua编写的模块不同，JiLang原生模块使用与主程序相同的语法和规则，无需学习新的编程语言。

## 模块基本结构

一个基本的JiLang原生模块具有以下结构：

```json
{
  "module_meta": {
    "version": "1.0.0",
    "description": "模块描述",
    "author": "作者名称"
  },
  "program": {
    "函数名1": {
      "params": {
        "参数1": "类型描述",
        "参数2": "类型描述"
      },
      "body": [
        // 函数体语句
      ]
    },
    "函数名2": {
      "params": {
        "参数1": "类型描述"
      },
      "body": [
        // 函数体语句
      ]
    }
  }
}
```

## 模块元数据

`module_meta`字段用于描述模块的基本信息：

```json
"module_meta": {
  "version": "1.0.0",      // 模块版本
  "description": "一个实用工具模块", // 模块描述
  "author": "JiLang开发者"   // 模块作者
}
```

这些元数据可以在导入模块的程序中通过`@var.module_meta.模块名.属性名`访问，例如：

```json
{"echo": ["模块版本：", "@var.module_meta.utils.version"]}
```

## 函数定义

模块中的函数定义在`program`对象内：

```json
"program": {
  "函数名": {
    "params": {
      "参数名1": "类型描述",
      "参数名2": "类型描述"
    },
    "body": [
      // 函数体语句
    ]
  }
}
```

其中：
- `函数名`：函数的名称，将用于调用该函数
- `params`：函数参数对象，定义函数接受的参数
- `body`：函数体，包含一系列语句

## 参数定义

参数定义支持以下类型描述：

```json
"params": {
  "数值参数": "number",
  "字符串参数": "string",
  "布尔参数": "boolean",
  "数组参数": "array",
  "对象参数": "object",
  "任意类型参数": null
}
```

类型描述主要用于文档目的，JiLang是弱类型语言，实际运行时不会强制类型检查。

## 模块示例

### 示例1：基础工具模块

一个简单的工具模块`utils.jl`：

```json
{
  "module_meta": {
    "version": "1.0.0",
    "description": "JiLang实用工具模块",
    "author": "JiLang团队"
  },
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
        {"echo": ["开始计数：\n"]},
        {"var": {"step": 1}},
        {"if": {
          "condition": {
            "op": "gt",
            "left": "@params.start",
            "right": "@params.end"
          },
          "then": [
            {"var": {"step": -1}}
          ]
        }},
        {"for": {
          "var": "i",
          "range": ["@params.start", "@params.end"],
          "step": "@var.step",
          "body": [
            {"echo": ["数字：", "@var.i", "\n"]}
          ]
        }}
      ]
    }
  }
}
```

### 示例2：数学计算模块

一个简单的数学计算模块`math_utils.jl`：

```json
{
  "module_meta": {
    "version": "1.0.0",
    "description": "JiLang数学计算工具",
    "author": "JiLang团队"
  },
  "program": {
    "factorial": {
      "params": {
        "n": "number"
      },
      "body": [
        {"if": {
          "condition": {"op": "lte", "left": "@params.n", "right": 1},
          "then": [
            {"return": {"value": 1}}
          ]
        }},
        {"var": {"recursiveResult": {
          "factorial": [{"op": "sub", "left": "@params.n", "right": 1}]
        }}},
        {"return": {
          "value": {"op": "mul", "left": "@params.n", "right": "@var.recursiveResult"}
        }}
      ]
    },
    "fibonacci": {
      "params": {
        "n": "number"
      },
      "body": [
        {"if": {
          "condition": {"op": "lte", "left": "@params.n", "right": 1},
          "then": [
            {"return": {"value": "@params.n"}}
          ]
        }},
        {"var": {"a": 0, "b": 1, "temp": 0, "i": 2}},
        {"while": {
          "condition": {"op": "lte", "left": "@var.i", "right": "@params.n"},
          "body": [
            {"var": {"temp": "@var.b"}},
            {"var": {"b": {"op": "add", "left": "@var.a", "right": "@var.b"}}},
            {"var": {"a": "@var.temp"}},
            {"var": {"i": {"op": "add", "left": "@var.i", "right": 1}}}
          ]
        }},
        {"return": {"value": "@var.b"}}
      ]
    }
  }
}
```

## 模块使用

### 导入模块

使用`include`字段导入模块：

```json
{
  "include": ["utils", "math_utils"],
  "program": {
    "main": {
      "body": [
        // 程序语句
      ]
    }
  }
}
```

### 调用模块函数

有多种方式调用模块中的函数：

#### 1. 使用点语法调用

```json
{"utils.greet": ["世界"]}
```

#### 2. 带输出变量的调用

```json
{
  "math_utils.factorial": {
    "0": 5,
    "output": "fact_result"
  }
}
```

#### 3. 在表达式中调用

```json
{"var": {"result": {"math_utils.fibonacci": [10]}}}
```

## 模块文件位置

JiLang解释器按以下顺序查找模块文件：

1. 当前工作目录
2. 当前工作目录下的`modules/`子目录
3. JiLang安装目录下的`modules/`目录

建议将自定义模块放在项目根目录的`modules/`子目录中，以便于管理。

## 返回值处理

JiLang模块函数有多种方式返回值：

### 1. 使用return语句

```json
"factorial": {
  "params": {
    "n": "number"
  },
  "body": [
    // ...
    {"return": {"value": 计算结果}}
  ]
}
```

### 2. 隐式返回最后一个语句的结果

如果函数没有明确的return语句，最后一个语句的结果会被作为返回值。

### 3. 使用result变量

在函数中设置`result`变量，它会自动成为返回值：

```json
"double": {
  "params": {
    "x": "number"
  },
  "body": [
    {"var": {"result": {"op": "mul", "left": "@params.x", "right": 2}}}
  ]
}
```

## 最佳实践

1. **提供完整的模块元数据**：总是包含版本、描述和作者信息
2. **使用有意义的函数名**：函数名应清晰表示其功能
3. **添加参数类型描述**：虽然JiLang是弱类型的，但类型描述可以帮助使用者理解参数
4. **使用函数注释**：在函数体第一行使用comment语句描述函数功能
5. **避免命名冲突**：避免使用与内置语句相同的函数名（如var、echo、if等）
6. **优先使用return语句**：明确使用return语句使函数逻辑更清晰

## 常见问题

### 模块不存在错误

如果遇到"模块不存在"错误，请检查：
- 模块文件名是否正确（应与include中的名称相同，加上.jl扩展名）
- 模块文件是否位于正确的目录
- 模块文件是否为有效的JSON格式

### 函数不存在错误

如果遇到"函数不存在"错误，请检查：
- 函数名拼写是否正确
- 函数是否正确定义在模块的program对象中
- 模块是否正确包含在include列表中

### 参数错误

如果遇到参数相关错误，请检查：
- 调用函数时提供的参数数量是否与函数定义匹配
- 参数的引用方式是否正确（应使用`@params.参数名`）

## 结论

JiLang原生模块提供了一种简单而强大的方式来组织和重用代码。通过创建模块，你可以构建可在多个项目中共享的功能库，同时保持代码的组织性和可维护性。 