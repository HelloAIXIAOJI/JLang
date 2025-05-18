# 变量声明与赋值（var语句）

本文档详细介绍了JiLang中的变量声明和赋值系统，包括var语句的语法、用法及最佳实践。

## 基本语法

在JiLang中，变量声明和赋值使用`var`语句，基本语法如下：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"变量名1": 值1, "变量名2": 值2, ...}}
      ]
    }
  }
}
```

其中：
- `var`：关键字，表示这是一个变量声明/赋值操作
- `"变量名"`: 字符串形式的变量名称
- `值`: 可以是字符串、数字、布尔值、数组、对象、变量引用或函数调用

## 基本示例

### 声明并赋值简单类型

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {
          "name": "张三",
          "age": 25,
          "isActive": true,
          "score": 98.5
        }},
        {"echo": ["姓名: ", "@var.name", ", 年龄: ", "@var.age"]}
      ]
    }
  }
}
```

### 声明并赋值复合类型

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {
          "scores": [85, 92, 78, 95],
          "user": {
            "name": "李四",
            "role": "管理员",
            "permissions": ["read", "write", "admin"]
          }
        }},
        {"echo": ["用户: ", "@var.user.name", ", 角色: ", "@var.user.role"]}
      ]
    }
  }
}
```

### 使用变量引用

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"x": 10}},
        {"var": {"y": 20}},
        {"var": {"sum": "@var.x + @var.y"}},
        {"echo": ["@var.x", " + ", "@var.y", " = ", "@var.sum"]}
      ]
    }
  }
}
```

## 赋值表达式

在JiLang中，变量的值可以是多种类型的表达式：

### 使用内置语句

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"result": {"math.add": [10, 20]}}},
        {"echo": ["10 + 20 = ", "@var.result"]}
      ]
    }
  }
}
```

### 使用条件表达式

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"age": 25}},
        {"var": {"status": {"if": {
          "condition": {"op": "gte", "left": "@var.age", "right": 18},
          "then": "成年",
          "else": "未成年"
        }}}},
        {"echo": ["状态: ", "@var.status"]}
      ]
    }
  }
}
```

### 嵌套函数调用

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"rawText": "  Hello, World!  "}},
        {"var": {"processed": {"concat": [
          "处理后: ",
          {"string.trim": ["@var.rawText"]}
        ]}}},
        {"echo": ["@var.processed"]}
      ]
    }
  }
}
```

## 变量作用域

JiLang中的变量有不同的作用域：

### 全局作用域

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"globalVar": "我是全局变量"}},
        {"echo": ["在main中访问: ", "@var.globalVar"]},
        {"function1": []}
      ]
    },
    "function1": {
      "params": [],
      "body": [
        {"echo": ["在function1中访问: ", "@var.globalVar"]}
      ]
    }
  }
}
```

### 函数作用域

```json
{
  "program": {
    "main": {
      "body": [
        {"function1": []},
        {"comment": "尝试访问function1中的局部变量会导致错误"},
        {"if": {
          "condition": {"op": "eq", "left": "@var.exists.localVar", "right": true},
          "then": [
            {"echo": ["localVar存在: ", "@var.localVar"]}
          ],
          "else": [
            {"echo": ["localVar不存在（预期结果）"]}
          ]
        }}
      ]
    },
    "function1": {
      "params": [],
      "body": [
        {"var": {"localVar": "我是函数局部变量"}},
        {"echo": ["在function1中访问: ", "@var.localVar"]}
      ]
    }
  }
}
```

### 参数作用域

```json
{
  "program": {
    "main": {
      "body": [
        {"greet": ["张三"]}
      ]
    },
    "greet": {
      "params": ["name"],
      "body": [
        {"echo": ["你好, ", "@param.name", "!"]},
        {"var": {"formalGreeting": {"concat": ["尊敬的 ", "@param.name", "，您好！"]}}},
        {"echo": ["@var.formalGreeting"]}
      ]
    }
  }
}
```

## 变量的高级用法

### 动态属性访问

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {
          "user": {
            "name": "王五",
            "age": 30,
            "email": "wangwu@example.com"
          },
          "property": "age"
        }},
        {"echo": ["动态访问: ", "@var.user[@var.property]"]}
      ]
    }
  }
}
```

### 数组索引

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"colors": ["红", "绿", "蓝", "黄", "紫"]}},
        {"echo": ["第一个颜色: ", "@var.colors[0]"]},
        {"echo": ["第三个颜色: ", "@var.colors[2]"]},
        {"var": {"index": 4}},
        {"echo": ["动态索引颜色: ", "@var.colors[@var.index]"]}
      ]
    }
  }
}
```

### 变量覆盖

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"counter": 1}},
        {"echo": ["初始值: ", "@var.counter"]},
        {"var": {"counter": 2}},
        {"echo": ["更新后值: ", "@var.counter"]},
        {"var": {"counter": {"op": "add", "left": "@var.counter", "right": 10}}},
        {"echo": ["再次更新后值: ", "@var.counter"]}
      ]
    }
  }
}
```

## 特殊变量引用

### 结果引用

```json
{
  "program": {
    "main": {
      "body": [
        {"math.add": [10, 20]},
        {"echo": ["计算结果: ", "@var.result"]},
        {"string.uppercase": ["hello"]},
        {"echo": ["转换结果: ", "@var.result"]}
      ]
    }
  }
}
```

### 存在性检查

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {
          "condition": {"op": "eq", "left": "@var.exists.undefinedVar", "right": false},
          "then": [
            {"echo": ["undefinedVar不存在"]},
            {"var": {"undefinedVar": "现在已定义"}},
            {"if": {
              "condition": {"op": "eq", "left": "@var.exists.undefinedVar", "right": true},
              "then": [
                {"echo": ["undefinedVar现在存在"]}
              ]
            }}
          ]
        }}
      ]
    }
  }
}
```

## 一次声明多个变量

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {
          "firstName": "张",
          "lastName": "三",
          "fullName": {"concat": ["@var.firstName", "@var.lastName"]},
          "age": 25,
          "isAdult": {"op": "gte", "left": "@var.age", "right": 18}
        }},
        {"echo": ["姓名: ", "@var.fullName", ", 是否成年: ", "@var.isAdult"]}
      ]
    }
  }
}
```

## 变量命名规则

JiLang的变量命名应遵循以下规则：

1. 可以使用字母、数字、下划线
2. 不能以数字开头
3. 区分大小写（`name`和`Name`是不同的变量）
4. 避免使用保留字（如`var`、`if`、`while`等）作为变量名

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {
          "validName": "有效变量名",
          "valid_name_2": "另一个有效变量名",
          "InvalidName": "区分大小写的变量名"
        }},
        {"echo": ["@var.validName", ", ", "@var.valid_name_2", ", ", "@var.InvalidName"]}
      ]
    }
  }
}
```

## 错误处理

### 引用不存在的变量

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {
          "condition": {"op": "eq", "left": "@var.exists.missingVar", "right": false},
          "then": [
            {"echo": ["安全检查: missingVar不存在"]}
          ],
          "else": [
            {"echo": ["错误: 这不应该发生"]}
          ]
        }},
        {"comment": "直接访问不存在的变量会产生错误"},
        {"try": {
          "body": [
            {"echo": ["这会产生错误: ", "@var.missingVar"]}
          ],
          "catch": [
            {"echo": ["捕获到错误: 变量missingVar不存在"]}
          ]
        }}
      ]
    }
  }
}
```

### 类型错误

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"text": "这是文本"}},
        {"try": {
          "body": [
            {"var": {"result": {"math.add": ["@var.text", 10]}}},
            {"echo": ["这不会执行"]}
          ],
          "catch": [
            {"echo": ["捕获到错误: 无法将文本与数字相加"]}
          ]
        }}
      ]
    }
  }
}
```

## 最佳实践

### 使用有意义的变量名

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "好的命名实践"},
        {"var": {
          "userName": "admin",
          "itemCount": 5,
          "isEnabled": true
        }},
        {"comment": "避免的命名实践"},
        {"var": {
          "x": "admin",
          "y": 5,
          "z": true
        }}
      ]
    }
  }
}
```

### 变量初始化

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {
          "counter": 0,
          "message": "",
          "items": [],
          "config": {}
        }},
        {"echo": ["变量已初始化"]}
      ]
    }
  }
}
```

### 变量的组织结构

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "相关变量组织在对象中"},
        {"var": {
          "user": {
            "name": "张三",
            "age": 25,
            "preferences": {
              "theme": "dark",
              "fontSize": "medium",
              "notifications": true
            }
          }
        }},
        {"echo": ["用户主题: ", "@var.user.preferences.theme"]}
      ]
    }
  }
}
```

## 技术实现

JiLang中的`var`语句处理流程：

1. 解析var语句中的所有键值对
2. 对每个值进行求值（处理变量引用和函数调用）
3. 将求值结果存储到变量上下文中
4. var语句返回最后一个赋值的值

这个过程由解释器中的`execute_var_statement`函数实现。 