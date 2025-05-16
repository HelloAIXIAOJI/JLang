# Echo语句（输出语句）

`echo`语句是JiLang中用于输出内容到控制台的基本语句，同时可以将输出内容存储到变量中以便后续使用。

## 语法格式

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": ["文本1", "文本2", "..."]}
      ]
    }
  }
}
```

若需要将输出结果额外存储到指定变量，可使用带output参数的格式：

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": {
          "0": "文本1",
          "1": "文本2",
          "output": "变量名"
        }}
      ]
    }
  }
}
```

## 功能特点

- **输出结果自动存储** - 输出的完整文本会自动存储到`@var.result`变量中
- **输出多个内容** - 可以在一条语句中输出多个值
- **变量引用解析** - 自动解析和输出变量值
- **支持各种数据类型** - 自动将数字、布尔值等转换为字符串
- **可选的输出变量** - 可通过`output`参数将结果存储到指定变量

## 基本用法

### 输出简单文本

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": ["Hello, World!"]},
        {"echo": ["上一条输出是: ", "@var.result"]}  // 输出: 上一条输出是: Hello, World!
      ]
    }
  }
}
```

### 输出变量内容

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"name": "张三", "age": 25}},
        {"echo": ["姓名: ", "@var.name", ", 年龄: ", "@var.age"]}
      ]
    }
  }
}
```

### 指定输出变量

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": {
          "0": "当前时间是: ",
          "1": {"time.now": []},
          "output": "timestamp_message" 
        }},
        {"echo": ["存储的消息: ", "@var.timestamp_message"]}
      ]
    }
  }
}
```

## 数据类型处理

`echo`语句会自动处理不同的数据类型并转换为字符串输出：

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": ["数字: ", 42]},
        {"echo": ["布尔值: ", true]},
        {"echo": ["JSON对象: ", {"name": "小明", "grade": "A"}]},
        {"echo": ["数组: ", [1, 2, 3, 4]]}
      ]
    }
  }
}
```

## 格式化技巧

### 使用换行符

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": ["第一行\n第二行\n第三行"]}
      ]
    }
  }
}
```

### 制表符对齐

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": ["名称\t价格\t数量"]},
        {"echo": ["商品A\t10.5\t20"]},
        {"echo": ["商品B\t25.0\t5"]}
      ]
    }
  }
}
```

### 多行输出

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"header": "=== 用户信息 ==="}},
        {"var": {"name": "张三"}},
        {"var": {"email": "zhangsan@example.com"}},
        {"echo": [
          "@var.header", "\n",
          "姓名: ", "@var.name", "\n",
          "邮箱: ", "@var.email"
        ]}
      ]
    }
  }
}
```

## 性能考虑

由于每次`echo`调用都会立即输出到控制台，在大量输出时请考虑合并多个输出以提高效率：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"buffer": ""}},
        {"for": {
          "var": "i",
          "in": [1, 2, 3, 4, 5],
          "body": [
            {"var": {"buffer": {"concat": ["@var.buffer", "项目 ", "@var.i", "\n"]}}}
          ]
        }},
        {"echo": ["@var.buffer"]}  // 一次性输出所有内容
      ]
    }
  }
}
```

## 结合其他语句使用

### 条件输出

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"score": 85}},
        {"if": {
          "condition": {"op": "gte", "left": "@var.score", "right": 60},
          "then": [
            {"echo": ["考试通过，得分: ", "@var.score"]}
          ],
          "else": [
            {"echo": ["考试未通过，得分: ", "@var.score"]}
          ]
        }}
      ]
    }
  }
}
```

### 循环输出

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"items": ["苹果", "香蕉", "橙子"]}},
        {"echo": ["购物清单:"]},
        {"for": {
          "var": "item",
          "in": "@var.items",
          "body": [
            {"echo": ["- ", "@var.item"]}
          ]
        }}
      ]
    }
  }
}
```

## 调试技巧

`echo`语句是调试JiLang程序的有力工具：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"x": 10}},
        {"var": {"y": 20}},
        {"echo": ["DEBUG: x=", "@var.x", ", y=", "@var.y"]},
        {"var": {"result": {"math.add": ["@var.x", "@var.y"]}}},
        {"echo": ["DEBUG: result=", "@var.result"]}
      ]
    }
  }
}
```

## 技术实现

JiLang中的`echo`语句实现在解释器的`basic.rs`文件中。当执行`echo`语句时，系统会：

1. 解析参数数组中的每个元素
2. 将变量引用解析为其实际值
3. 将各种数据类型转换为字符串
4. 输出到控制台
5. 将完整输出存储到`result`变量
6. 若指定了`output`参数，则额外存储到该变量

## 最佳实践

- 使用简洁明了的输出信息
- 对于调试输出，添加明显标记如"DEBUG:"前缀
- 长输出考虑使用格式化（换行、缩进等）提高可读性
- 利用`result`变量捕获输出以便后续处理
- 如需将输出存储到特定变量，使用`output`参数而非额外的变量赋值语句

## 与其他语句的区别

| 语句 | 主要功能 | 输出存储 | 特点 |
|-----|---------|---------|-----|
| `echo` | 控制台输出 | 自动存储到`result` | 立即可见的输出 |
| `concat` | 字符串连接 | 自动存储到`result`或指定变量 | 无控制台输出 |
| `var` | 变量定义 | 无输出功能 | 仅存储值 |
| `comment` | 代码注释 | 无输出 | 调试模式可见 | 