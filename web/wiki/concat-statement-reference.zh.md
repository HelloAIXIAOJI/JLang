# 字符串连接（concat语句）

`concat`语句用于将多个值连接成一个字符串，是JiLang中处理字符串操作的核心功能。

## 语法格式

JiLang中`concat`语句支持两种格式：

### 1. 数组格式（推荐）

简洁直观的语法，类似echo语句：

```json
{
  "program": {
    "main": {
      "body": [
        {"concat": ["值1", "值2", "值3", ...]}
      ]
    }
  }
}
```

使用数组格式时，结果会自动存储在`@var.result`变量中。

### 2. 对象格式（传统）

完整的参数化语法，可指定目标变量：

```json
{
  "program": {
    "main": {
      "body": [
        {"concat": {
          "target": "目标变量名",
          "parts": ["值1", "值2", "值3", ...],
          "output": "可选的额外输出变量名"
        }}
      ]
    }
  }
}
```

## 两种格式的区别

| 特性 | 数组格式 | 对象格式 |
|------|---------|----------|
| 语法复杂度 | 简单 | 较复杂 |
| 结果存储位置 | 固定为`@var.result` | 可通过`target`自定义 |
| 额外输出 | 不支持 | 可用`output`参数指定 |
| 适用场景 | 内联使用、简单连接 | 需要自定义变量名时 |

## 基本用法

### 数组格式示例

```json
{
  "program": {
    "main": {
      "body": [
        {"concat": ["Hello, ", "World!"]},
        {"echo": ["结果：", "@var.result"]}  // 输出: 结果：Hello, World!
      ]
    }
  }
}
```

### 对象格式示例

```json
{
  "program": {
    "main": {
      "body": [
        {"concat": {
          "target": "greeting",
          "parts": ["Hello, ", "World!"]
        }},
        {"echo": ["结果：", "@var.greeting"]}  // 输出: 结果：Hello, World!
      ]
    }
  }
}
```

### 在变量定义中使用

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"name": "张三"}},
        {"var": {"greeting": {"concat": ["你好，", "@var.name", "！"]}}},
        {"echo": ["@var.greeting"]}  // 输出: 你好，张三！
      ]
    }
  }
}
```

## 数据类型处理

`concat`语句会自动将各种数据类型转换为字符串：

| 数据类型 | 转换结果 | 示例 |
|---------|---------|------|
| 字符串 | 原样保留 | `"text"` → `"text"` |
| 数字 | 字符串表示 | `42` → `"42"` |
| 布尔值 | `"true"`或`"false"` | `true` → `"true"` |
| 数组 | 元素以逗号连接 | `[1,2,3]` → `"1,2,3"` |
| 对象 | JSON字符串 | `{"a":1}` → `"{"a":1}"` |
| null | 空字符串 | `null` → `""` |

示例：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"result": {"concat": [
          "字符串: ", "text", 
          ", 数字: ", 42, 
          ", 布尔值: ", true, 
          ", 数组: ", [1, 2, 3],
          ", null值: ", null
        ]}}},
        {"echo": ["@var.result"]}
      ]
    }
  }
}
```

## 高级用法

### 嵌套调用

`concat`语句可以嵌套使用，处理复杂的字符串构建：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"first_name": "John"}},
        {"var": {"last_name": "Doe"}},
        {"var": {"greeting": {"concat": [
          "Hello, ", 
          {"concat": ["Mr. ", "@var.first_name", " ", "@var.last_name"]}
        ]}}},
        {"echo": ["@var.greeting"]}  // 输出: Hello, Mr. John Doe
      ]
    }
  }
}
```

### 在条件表达式中使用

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"username": "admin"}},
        {"if": {
          "condition": {"op": "eq", "left": {"concat": ["用户:", "@var.username"]}, "right": "用户:admin"},
          "then": [
            {"echo": ["权限验证成功"]}
          ],
          "else": [
            {"echo": ["权限验证失败"]}
          ]
        }}
      ]
    }
  }
}
```

## 常见应用场景

### 构建URL

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"base_url": "https://api.example.com"}},
        {"var": {"endpoint": "/users"}},
        {"var": {"user_id": 12345}},
        {"var": {"query": "?format=json"}},
        {"var": {"full_url": {"concat": ["@var.base_url", "@var.endpoint", "/", "@var.user_id", "@var.query"]}}},
        {"echo": ["完整URL: ", "@var.full_url"]}
      ]
    }
  }
}
```

### 构建文件路径

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"directory": "/home/user/documents"}},
        {"var": {"filename": "report"}},
        {"var": {"extension": ".pdf"}},
        {"var": {"filepath": {"concat": ["@var.directory", "/", "@var.filename", "@var.extension"]}}},
        {"echo": ["文件路径: ", "@var.filepath"]}
      ]
    }
  }
}
```

### 格式化输出表格

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"header": {"concat": [
          "+", "-".repeat(10), "+", "-".repeat(15), "+", "-".repeat(8), "+\n",
          "| 产品名称 ", "| 描述        ", "| 价格  ", "|\n",
          "+", "-".repeat(10), "+", "-".repeat(15), "+", "-".repeat(8), "+"
        ]}}},
        {"echo": ["@var.header"]}
      ]
    }
  }
}
```

## 性能优化与最佳实践

### 1. 避免多次连接

在需要连接多个值时，尽量在一次操作中完成：

✅ **推荐:**
```json
{"var": {"result": {"concat": ["A", "B", "C", "D"]}}}
```

❌ **不推荐:**
```json
{"var": {"temp1": {"concat": ["A", "B"]}}},
{"var": {"temp2": {"concat": ["@var.temp1", "C"]}}},
{"var": {"result": {"concat": ["@var.temp2", "D"]}}}
```

### 2. 预处理复杂数据

连接复杂数据前先转换为所需格式：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"user": {"name": "张三", "age": 30}}},
        
        // 先转换为需要的格式
        {"var": {"formatted_user": {"concat": ["姓名: ", "@var.user.name", ", 年龄: ", "@var.user.age"]}}},
        
        // 然后在报告中使用
        {"var": {"report": {"concat": ["用户信息: ", "@var.formatted_user"]}}}
      ]
    }
  }
}
```

### 3. 适当使用变量存储中间结果

对于重复使用的连接结果，存储为变量可提高效率并增强可读性：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"header": {"concat": ["姓名", ",", "年龄", ",", "职位"]}}},
        {"var": {"row1": {"concat": ["张三", ",", "30", ",", "工程师"]}}},
        {"var": {"row2": {"concat": ["李四", ",", "28", ",", "设计师"]}}},
        {"var": {"csv": {"concat": ["@var.header", "\n", "@var.row1", "\n", "@var.row2"]}}},
        {"echo": ["CSV数据:\n", "@var.csv"]}
      ]
    }
  }
}
```

## 错误处理与调试

### 常见错误

1. **引用不存在的变量**

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"result": {"concat": ["Value: ", "@var.undefined_var"]}}},
        {"echo": ["@var.result"]}  // 将产生错误
      ]
    }
  }
}
```

2. **防御式编程**

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {
          "condition": {"op": "eq", "left": "@var.exists.user_name", "right": true},
          "then": [
            {"var": {"greeting": {"concat": ["你好, ", "@var.user_name"]}}}
          ],
          "else": [
            {"var": {"greeting": "你好, 访客"}}
          ]
        }},
        {"echo": ["@var.greeting"]}
      ]
    }
  }
}
```

## 技术实现

JiLang中`concat`语句的实现位于`basic.rs`文件，处理流程为：

1. 检测输入格式（数组或对象）
2. 解析并处理每个连接部分
   - 直接值保持不变
   - 变量引用解析为实际值
   - 不同数据类型转换为字符串表示
3. 将处理后的所有部分连接为单个字符串
4. 根据格式存储结果
   - 数组格式：存储到`result`变量
   - 对象格式：存储到`target`指定的变量

## 与其他语句的结合使用

`concat`语句可以与其他JiLang语句灵活组合，创建强大的字符串处理流程：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"items": ["苹果", "香蕉", "橙子"]}},
        {"var": {"count": 0}},
        {"var": {"inventory": "库存清单:\n"}},
        
        {"for": {
          "var": "item",
          "in": "@var.items",
          "body": [
            {"var": {"count": {"math.add": ["@var.count", 1]}}},
            {"var": {"inventory": {"concat": [
              "@var.inventory", 
              "@var.count", ". ", "@var.item", "\n"
            ]}}}
          ]
        }},
        
        {"echo": ["@var.inventory"]}
      ]
    }
  }
}
```

## 总结

`concat`语句是JiLang中最常用的字符串处理工具之一，通过灵活的两种语法格式，可以满足从简单到复杂的各种字符串连接需求。在选择语法格式时：

- 对于简单内联使用，优先选择**数组格式**
- 需要自定义变量名或额外输出时，使用**对象格式**

通过熟练掌握`concat`语句，可以在JiLang中高效处理各种文本和字符串操作需求。 