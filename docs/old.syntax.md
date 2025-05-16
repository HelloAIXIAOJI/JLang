# JiLang 语法参考

# 此文档已不再更新与维护，请参考wiki目录。

这个文档详细描述了JiLang编程语言的语法。JiLang是一种基于JSON的编程语言，它允许您使用熟悉的JSON结构编写程序，同时提供变量、循环、条件、函数等编程功能。

## 目录

1. [基础程序结构](#基础程序结构)
2. [变量和常量](#变量和常量)
3. [数据类型](#数据类型)
4. [运算符](#运算符)
5. [条件语句](#条件语句)
6. [循环](#循环)
7. [函数](#函数)
8. [模块系统](#模块系统)
9. [注释](#注释)
10. [内置功能](#内置功能)
11. [综合示例](#综合示例)

## 1. 程序结构

JiLang 程序是一个 JSON 对象，包含以下顶级字段：

```json
{
    "include": ["模块1", "模块2"],  // 可选，包含需要使用的模块
    "const": {                      // 可选，定义常量
        "常量名": "常量值"
    },
    "program": {                    // 必需，程序主体
        "main": {                   // 必需，主函数
            "body": [               // 必需，语句数组
                // 语句列表
            ]
        },
        "其他函数名": {             // 可选，自定义函数
            "params": {             // 可选，函数参数
                "参数名": "类型"
            },
            "body": [              // 必需，函数体
                // 语句列表
            ]
        }
    }
}
```

## 2. 数据类型

JiLang 支持以下数据类型：

- 字符串：`"hello"`
- 数字：`123`, `3.14`
- 布尔值：`true`, `false`
- 空值：`null`

## 3. 变量和常量

### 3.1 变量定义

```json
{"var": {"变量名": "值"}}
```

变量值可以是任何有效的 JSON 值或变量引用。

### 3.2 变量引用

使用 `@var.变量名` 语法引用变量：

```json
{"echo": ["当前值：", "@var.counter"]}
```

对于嵌套的数据结构，可以使用点号表示路径：

```json
{"echo": ["用户姓名：", "@var.user.profile.name"]}
```

支持任意深度的嵌套访问，只要路径中的每个部分都是有效的对象字段。

### 3.3 常量定义

在程序顶级的 `const` 对象中定义：

```json
"const": {
    "PI": 3.14159,
    "APP_NAME": "MyApp"
}
```

### 3.4 常量引用

使用 `@const.常量名` 语法引用常量：

```json
{"echo": ["应用名称：", "@const.APP_NAME"]}
```

## 4. 语句类型

### 4.1 变量赋值

```json
{"var": {"name": "value"}}
```

### 4.2 输出语句

```json
{"echo": ["文本1", "@var.变量名", "文本2"]}
```

### 4.3 注释

JiLang 支持多种注释方式：

#### 4.3.1 JSON注释对象

1. 单行注释（字符串形式）：
```json
{"comment": "这是一个注释"}
```

2. 多部分注释（数组形式，可以包含变量引用）：
```json
{"comment": ["这是注释的第一部分", "，变量值：", "@var.counter"]}
```

#### 4.3.2 双斜杠注释

JiLang 解释器支持传统的双斜杠注释语法，虽然这不是标准的JSON语法，但它会在解析前自动被移除：

```json
// 这是整行注释
{
    "program": {
        "main": {
            "body": [
                {"echo": ["Hello"]}, // 这是行尾注释
                // 下面是另一个注释行
                {"echo": ["World"]}
            ]
        }
    }
}
```

注释会被解释器忽略，不会影响程序的执行。在调试模式下运行时，`comment`对象形式的注释内容会被显示出来，而`//`形式的注释会在解析前被完全移除。

注释的使用场景：
- 解释代码的目的和功能
- 记录算法步骤
- 标记TODO项或未来需要改进的部分
- 临时禁用某些代码块

示例：
```json
{"comment": "下面计算平方根"},
{"math.sqrt": [16]},
{"comment": ["结果是: ", "@var.result"]}
```

### 4.4 执行系统命令

通过 `exec` 语句可以执行系统命令，并获取其输出结果：

```json
{"exec": {
    "cmd": "命令名称",
    "args": ["参数1", "参数2", "@var.变量参数"],
    "output": "结果变量名"  // 可选，默认为 "result"
}}
```

执行结果会存储在指定的变量中（默认为`result`），结果是一个对象，包含以下字段：
- `stdout`: 标准输出（字符串）
- `stderr`: 标准错误（字符串）
- `status`: 退出状态码（数字）

示例：
```json
// 列出目录内容
{"exec": {
    "cmd": "ls",
    "args": ["-la"],
    "output": "dir_content"
}},
{"echo": ["目录内容：\n", "@var.dir_content.stdout"]}

// 在Windows上执行
{"exec": {
    "cmd": "dir",
    "output": "files"
}},
{"echo": ["文件列表：\n", "@var.files.stdout"]}
```

注意：
1. 在Windows系统上，命令会通过`cmd /C`执行
2. 在Linux/macOS系统上，命令会通过`sh -c`执行
3. 使用此功能时请注意安全风险，避免执行不受信任的命令

### 4.5 字符串拼接

```json
{"concat": {
    "target": "结果变量名",
    "parts": ["字符串1", "@var.变量名", "字符串2"]
}}
```

### 4.6 条件语句

```json
{"if": {
    "condition": {
        "op": "操作符",    // eq, neq, gt, lt, gte, lte
        "left": "值1",
        "right": "值2"
    },
    "then": [
        // 条件为真时执行的语句
    ],
    "else": [
        // 条件为假时执行的语句
    ]
}}
```

支持的比较操作符：
- `eq`: 等于
- `neq`: 不等于
- `gt`: 大于
- `lt`: 小于
- `gte`: 大于等于
- `lte`: 小于等于

### 4.7 Switch语句

```json
{"switch": {
    "expr": "表达式或变量引用",
    "cases": [
        {
            "value": "匹配值1",
            "body": [
                // 匹配时执行的语句
            ]
        },
        {
            "value": "匹配值2",
            "fallthrough": true,  // 可选，默认为false，设为true时会继续执行下一个case
            "body": [
                // 匹配时执行的语句
            ]
        },
        {
            "default": true,      // 默认分支
            "body": [
                // 没有匹配时执行的语句
            ]
        }
    ]
}}
```

说明：
- `expr`: 要匹配的表达式或变量引用
- `cases`: 包含多个case对象的数组
- 每个case对象包含：
  - `value`: 要匹配的值
  - `body`: 匹配成功时执行的语句数组
  - `fallthrough`: 可选布尔值，如果为true，则匹配后会继续执行下一个case，默认为false
- default分支使用`"default": true`标记，通常放在最后，当没有case匹配时执行

示例：
```json
{"var": {"day": "周二"}},
{"switch": {
    "expr": "@var.day",
    "cases": [
        {
            "value": "周一",
            "body": [
                {"echo": ["今天是周一\n"]}
            ]
        },
        {
            "value": "周二",
            "body": [
                {"echo": ["今天是周二\n"]}
            ]
        },
        {
            "default": true,
            "body": [
                {"echo": ["是其他日子\n"]}
            ]
        }
    ]
}}
```

### 4.8 循环语句

#### 4.8.1 while 循环

```json
{"while": {
    "condition": {
        "op": "操作符",
        "left": "值1",
        "right": "值2"
    },
    "body": [
        // 循环体语句
    ]
}}
```

示例：
```json
{"while": {
    "condition": {
        "op": "lt",
        "left": "@var.counter",
        "right": "5"
    },
    "body": [
        {"echo": ["计数：", "@var.counter", "\n"]},
        {"math.add": ["@var.counter", 1]},
        {"var": {"counter": "@var.result"}}
    ]
}}
```

#### 4.8.2 for 循环

支持两种 for 循环语法：

1. 范围语法（推荐）：
```json
{"for": {
    "var": "循环变量名",
    "range": [起始值, 结束值],
    "step": 步长,  // 可选，默认为 1
    "body": [
        // 循环体语句
    ]
}}
```

2. 传统语法：
```json
{"for": {
    "var": "循环变量名",
    "from": 起始值,
    "to": 结束值,
    "step": 步长,  // 可选，默认为 1
    "body": [
        // 循环体语句
    ]
}}
```

示例：
```json
// 正序循环
{"for": {
    "var": "i",
    "range": [1, 5],
    "body": [
        {"echo": ["第 ", "@var.i", " 次迭代\n"]}
    ]
}}

// 倒序循环
{"for": {
    "var": "count",
    "range": [5, 0],
    "step": -1,
    "body": [
        {"echo": ["倒计时：", "@var.count", "\n"]}
    ]
}}
```

### 4.9 函数调用

支持两种函数调用语法：

1. 标准语法：
```json
{"call": ["函数名", {"参数名": "参数值"}]}
```

2. 简化语法（推荐）：
```json
{"函数名": [参数1, 参数2, ...]}
```

### 4.10 模块函数调用

支持两种模块函数调用语法：

1. 标准语法：
```json
{"call": ["模块名.函数名", 参数1, 参数2]}
```

2. 简化语法（推荐）：
```json
{"模块名.函数名": [参数1, 参数2, ...]}
```

示例：
```json
// 标准语法
{"call": ["math.add", 1, 2, 3]}

// 简化语法
{"math.add": [1, 2, 3]}
```

### 4.11 数组操作

JiLang提供一系列内置的数组操作语句，无需依赖外部模块即可处理数组：

#### 4.11.1 创建数组

```json
// 创建空数组
{"array.create": []},

// 创建带初始元素的数组
{"array.create": [1, 2, 3, "四", true, null]},

// 创建指定大小的数组（带初始值）
{"array.create": {"size": 5, "initial": 0}}
```

#### 4.11.2 添加和移除元素

```json
// 添加一个或多个元素到数组末尾
{"array.push": ["@var.myArray", "元素1", "元素2"]},

// 从数组末尾移除元素并返回
{"array.pop": ["@var.myArray"]}
```

#### 4.11.3 获取和设置元素

```json
// 获取指定索引的元素
{"array.get": ["@var.myArray", 2]},

// 设置指定索引的元素（支持自动扩展数组）
{"array.set": ["@var.myArray", 5, "新值"]}
```

#### 4.11.4 获取数组长度

```json
{"array.length": ["@var.myArray"]}
```

#### 4.11.5 获取数组切片

```json
// 获取从索引1到4的切片（不包含索引4）
{"array.slice": ["@var.myArray", 1, 4]},

// 只指定起始索引，到数组末尾
{"array.slice": ["@var.myArray", 2]}
```

这些数组操作都会在`result`变量中返回操作结果，便于链式处理。

## 5. 函数定义

```json
"函数名": {
    "params": {
        "参数名": "类型"
    },
    "body": [
        // 函数体语句
    ]
}
```

每个函数在执行时会创建自己的局部变量作用域。函数执行完成后：
1. 函数内部的局部变量不会保留
2. 函数的返回值会存储在 `result` 变量中
3. 在函数内创建的但不是参数或临时变量的其他变量会保留在全局作用域中

这种机制确保了函数的参数和临时变量不会污染全局作用域，同时也支持递归函数的实现。

注意事项：
1. 函数名不能与内置语句（var, echo, concat, if, call, while, for, comment）冲突
2. 函数名不能与已包含模块的函数名冲突
3. 函数名不能与已定义的其他函数名冲突

示例：
```json
"print_message": {
    "params": {
        "text": "string"
    },
    "body": [
        {"echo": ["收到的消息是：\n"]},
        {"echo": ["@params.text", "\n"]}
    ]
}
```

### 5.1 函数调用

支持两种函数调用语法：

1. 标准语法：
```json
{"call": ["函数名", {"参数名": "参数值"}]}
```

2. 简化语法（推荐）：
```json
{"函数名": [参数1, 参数2, ...]}
```

示例：
```json
// 标准语法
{"call": ["print_message", {"text": "你好"}]}

// 简化语法
{"print_message": ["你好"]}
```

## 6. 命令行参数

JiLang 解释器支持以下命令行参数：

### 6.1 基本用法

```bash
JiLang [选项] <程序文件路径>
```

### 6.2 调试模式

使用 `--debug` 参数可以启用调试模式，显示额外的调试信息：

```bash
JiLang --debug <程序文件路径>
```

在调试模式下，解释器会输出额外的信息，例如：
- for 循环的起始值、结束值和步长
示例：
```bash
# 正常模式（不显示调试信息）
cargo run examples/loop_test.jl

# 调试模式（显示调试信息）
cargo run -- --debug examples/loop_test.jl
```

### 6.3 包含自定义模块

当使用 `include` 包含自定义 `.jl` 模块时，解释器会按以下顺序查找模块文件：

1. 当前目录
2. `examples` 目录

确保您的模块文件名与 `include` 中声明的名称一致，并带有 `.jl` 扩展名。

#### 6.3.1 自定义模块函数调用语法

自定义 `.jl` 模块的函数可以使用与内置模块（如 `math`、`io`）相同的调用语法：

1. 标准语法：
```json
{"call": ["自定义模块名.函数名", 参数1, 参数2]}
```

2. 简化语法（推荐）：
```json
{"自定义模块名.函数名": [参数1, 参数2, ...]}
```

示例：如果您有一个名为 `utils.jl` 的自定义模块包含 `greet` 函数，可以这样调用：

```json
// 标准语法
{"call": ["utils.greet", "世界"]}

// 简化语法
{"utils.greet": ["世界"]}
```

示例结构：
```
my_project/
  ├── main.jl          # 主程序
  ├── utils.jl         # 工具模块
  └── examples/
      └── demo.jl      # 示例程序
```

在 `main.jl` 中使用：
```json
{
    "include": ["utils"],
    "program": {
        "main": {
            "body": [
                // 两种等效的调用方式
                {"call": ["utils.some_function", "参数"]},
                {"utils.some_function": ["参数"]}
            ]
        }
    }
}
```

## 7. 高级特性

### 7.1 递归函数支持

JiLang 完全支持递归函数调用，可以实现复杂的算法，如阶乘、斐波那契数列等。

示例（递归实现阶乘）：
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

### 7.2 多层嵌套数据结构访问

支持访问多层嵌套的对象结构，使用点号分隔路径：

```json
{"var": {"user": {"profile": {"name": "张三", "age": 25}}}},
{"echo": ["用户姓名: ", "@var.user.profile.name", "\n"]},
{"echo": ["用户年龄: ", "@var.user.profile.age", "\n"]}
```

### 7.3 函数局部变量作用域

函数内部的变量遵循局部作用域规则：
1. 函数参数和局部变量在函数执行过程中可用
2. 函数执行完毕后，局部变量会被清理，不会污染全局作用域
3. 函数返回值通过 `result` 变量传递给调用者
4. 函数内部创建的全局变量会保留

```json
{"function_name": [参数1, 参数2]},
{"echo": ["函数返回值: ", "@var.result", "\n"]}
```

### 7.4 调试模式下的注释行为

在调试模式下 (`--debug`) 运行时，注释语句会被显示出来，有助于理解程序执行流程：

```bash
cargo run -- --debug examples/my_program.jl
```

输出示例：
```
执行语句: comment
// 注释: 这是一个调试注释
执行语句: var
...
```

### 7.5 返回值机制

函数执行后，返回值存储在 `result` 变量中，可以直接访问：

```json
{"math.add": [5, 3]},
{"echo": ["计算结果: ", "@var.result", "\n"]}
```

特殊情况下（如为了支持递归），也可以使用自定义的结果变量名（如 `factorial_result`），但建议总是使用标准的 `result` 变量获取返回值。