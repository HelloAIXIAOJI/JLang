# JiLang变量与类型系统

JiLang采用灵活的弱类型系统，允许在不同类型之间自动转换。本文档详细介绍JiLang的变量、常量、类型系统和作用域规则。

## 变量声明与赋值

在JiLang中，使用`var`语句声明和赋值变量：

```json
{"var": {"变量名": 值}}
```

### 基本变量声明示例

```json
{"var": {"name": "张三"}}            // 字符串
{"var": {"age": 25}}                 // 数字
{"var": {"is_active": true}}         // 布尔值
{"var": {"data": null}}              // 空值
{"var": {"points": [10, 20, 30]}}    // 数组
{"var": {"user": {                   // 对象
    "name": "李四",
    "age": 30
}}}
```

### 变量重新赋值

JiLang允许给已存在的变量赋予新值，包括不同类型的值：

```json
{"var": {"counter": 0}}              // 初始为数字
{"var": {"counter": "重置"}}         // 重新赋值为字符串
{"var": {"counter": true}}           // 重新赋值为布尔值
```

### 通过操作结果赋值

某些操作会生成结果，可以将结果赋值给变量：

```json
{"math.add": [5, 3]}                 // 结果存储在@var.result
{"var": {"sum": "@var.result"}}      // 将结果赋值给sum变量
```

## 常量定义

常量在程序顶层的`const`对象中定义，一旦定义就不能修改：

```json
{
    "const": {
        "PI": 3.14159,
        "APP_NAME": "JiLang应用",
        "MAX_USERS": 100,
        "ENABLED_FEATURES": ["login", "profile", "messaging"]
    },
    "program": {
        // 程序主体...
    }
}
```

## 变量引用

JiLang提供多种方式引用变量：

### 标准引用

使用`@var.`前缀：

```json
{"echo": ["用户名: ", "@var.name"]}
```

### 简写引用

使用`$`符号：

```json
{"echo": ["年龄: ", "$age"]}
```

### 中文引用

使用`￥`符号：

```json
{"echo": ["城市: ", "￥city"]}
```

### 混合使用

可以在同一程序中混合使用不同的引用方式，但建议保持一致的风格：

```json
{"echo": ["信息: ", "@var.name", " 今年 ", "$age", " 岁，住在 ", "￥city"]}
```

## 数据类型

JiLang支持所有JSON基本数据类型：

### 基本类型

- **字符串**：用双引号括起的文本，如`"Hello"`
- **数字**：整数或浮点数，如`42`或`3.14`
- **布尔值**：`true`或`false`
- **null**：表示空值或未定义的值

### 复合类型

- **数组**：有序值的列表，如`[1, 2, 3]`
- **对象**：键值对的集合，如`{"name": "张三", "age": 25}`

## 弱类型系统

JiLang使用弱类型系统，具有以下特点：

### 1. 类型自动转换

在不同类型之间自动进行转换：

```json
{"var": {"x": "42"}}                 // 字符串"42"
{"math.add": ["@var.x", 8]}          // 字符串自动转换为数字
{"var": {"x": "@var.result"}}        // 现在x是数字50
```

### 2. 类型比较规则

不同类型值的比较遵循以下规则：

- 字符串数字`"5"`与数字`5`比较时视为相等
- 空字符串`""`在某些上下文中可能转换为`0`或`false`
- 布尔值`true`通常等同于`1`，`false`等同于`0`
- `null`在比较时有特殊行为，通常与`0`、`false`或空字符串相等

示例：
```json
{"if": {
    "condition": {
        "op": "eq", 
        "left": "5", 
        "right": 5
    },
    "then": [
        {"echo": ["字符串'5'等于数字5\n"]}
    ]
}}
```

### 3. 算术运算中的类型转换

- 字符串数字自动转换为数字进行计算
- 布尔值在算术运算中，`true`视为`1`，`false`视为`0`
- 无法转换为数字的字符串在算术运算中会导致错误或特殊值

```json
{"math.add": ["42", 8]}              // 字符串"42"转换为数字，结果为50
{"math.mul": [true, 5]}              // 布尔值true转换为1，结果为5
```

### 4. 字符串操作中的转换

非字符串值在字符串操作中会自动转换为字符串：

```json
{"concat": {
    "target": "message",
    "parts": ["计数: ", 42, ", 状态: ", true]
}}
// 结果: "计数: 42, 状态: true"
```

## 变量作用域

JiLang的变量遵循以下作用域规则：

### 全局变量

在`main`函数或任何函数外部声明的变量可以在整个程序中访问：

```json
{
    "program": {
        "var": {"global_counter": 0},  // 全局变量
        "main": {
            "body": [
                {"echo": ["全局计数: ", "@var.global_counter"]}
            ]
        }
    }
}
```

### 局部变量

在函数内部声明的变量只在该函数内可见：

```json
{
    "program": {
        "test_function": {
            "body": [
                {"var": {"local_var": "仅函数内可见"}},
                {"echo": ["局部变量: ", "@var.local_var"]}
            ]
        },
        "main": {
            "body": [
                {"test_function": []},
                // 无法访问local_var
            ]
        }
    }
}
```

### 函数参数

函数参数使用`@params.`前缀访问，只在函数内部可见：

```json
{
    "program": {
        "greet": {
            "params": {
                "name": "string"
            },
            "body": [
                {"echo": ["你好，", "@params.name", "！"]}
            ]
        }
    }
}
```

## 嵌套数据结构访问

JiLang支持访问嵌套的对象属性和数组元素：

### 对象属性访问

使用点号（.）访问对象属性：

```json
{"var": {"user": {"name": "张三", "age": 25}}}
{"echo": ["用户名: ", "@var.user.name"]}
```

### 数组索引访问

使用方括号（[]）访问数组元素（索引从0开始）：

```json
{"var": {"numbers": [10, 20, 30, 40, 50]}}
{"echo": ["第三个数字: ", "@var.numbers[2]"]}  // 输出30
```

### 混合访问

可以混合使用点号和方括号访问复杂的嵌套结构：

```json
{"var": {"users": [
    {"name": "张三", "skills": ["Java", "Python"]},
    {"name": "李四", "skills": ["C++", "JavaScript"]}
]}}
{"echo": ["第二个用户的第一个技能: ", "@var.users[1].skills[0]"]}  // 输出C++
```

## 特殊变量

JiLang有一些特殊变量用于存储操作结果和状态：

### 1. 结果变量

许多操作会自动将结果存储在`@var.result`变量中：

```json
{"math.add": [5, 3]}  // 结果存储在@var.result中
{"echo": ["结果: ", "@var.result"]}  // 输出8
```

### 2. 环境变量

使用`@env.`前缀访问系统环境变量：

```json
{"echo": ["当前用户: ", "@env.USER"]}
{"echo": ["主目录: ", "@env.HOME"]}
```

## 最佳实践

以下是JiLang变量使用的最佳实践：

1. **保持一致的命名风格**：使用描述性的变量名，遵循一致的命名约定
2. **保持一致的引用风格**：在整个程序中使用相同的变量引用前缀（`@var.`、`$`或`￥`）
3. **变量初始化**：始终在使用变量前对其进行初始化
4. **注意类型转换**：虽然JiLang自动进行类型转换，但要注意可能的意外结果
5. **适当使用对象和数组**：利用复合数据结构组织相关数据
6. **避免重复使用变量名**：不同作用域中避免使用相同的变量名，以减少混淆

## 变量系统示例

以下是一个综合示例，展示JiLang变量系统的多个方面：

```json
{
    "include": ["math"],
    "const": {
        "APP_VERSION": "1.0.0",
        "MAX_ITEMS": 100
    },
    "program": {
        "main": {
            "body": [
                {"var": {"user": {
                    "name": "张三",
                    "age": 25,
                    "scores": [85, 92, 78]
                }}},
                
                {"echo": ["应用版本: ", "@const.APP_VERSION", "\n"]},
                {"echo": ["用户名: ", "@var.user.name", "\n"]},
                {"echo": ["用户年龄: ", "$user.age", "\n"]},
                
                {"var": {"avg_score": 0}},
                {"calculate_average": ["@var.user.scores"]},
                {"var": {"avg_score": "@var.result"}},
                
                {"echo": ["平均分数: ", "@var.avg_score", "\n"]},
                
                {"if": {
                    "condition": {
                        "op": "gt",
                        "left": "@var.avg_score",
                        "right": 80
                    },
                    "then": [
                        {"echo": ["成绩优秀！\n"]}
                    ],
                    "else": [
                        {"echo": ["继续努力！\n"]}
                    ]
                }}
            ]
        },
        "calculate_average": {
            "params": {
                "scores": "array"
            },
            "body": [
                {"var": {"sum": 0}},
                {"var": {"i": 0}},
                {"array.length": ["@params.scores"]},
                {"var": {"len": "@var.result"}},
                
                {"while": {
                    "condition": {
                        "op": "lt",
                        "left": "@var.i",
                        "right": "@var.len"
                    },
                    "body": [
                        {"array.get": ["@params.scores", "@var.i"]},
                        {"var": {"score": "@var.result"}},
                        {"math.add": ["@var.sum", "@var.score"]},
                        {"var": {"sum": "@var.result"}},
                        
                        {"math.add": ["@var.i", 1]},
                        {"var": {"i": "@var.result"}}
                    ]
                }},
                
                {"math.divide": ["@var.sum", "@var.len"]},
                {"var": {"result": "@var.result"}}
            ]
        }
    }
}
```

## 下一步学习

- [引用符号](reference_symbols.md) - 详细了解变量引用符号
- [控制结构](control_flow.md) - 学习条件语句和循环结构
- [函数与模块](functions.md) - 探索函数定义和模块系统
- [基本语法](basic.md) - 回顾JiLang的基本语法概念 