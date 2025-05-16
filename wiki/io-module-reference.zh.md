# IO标准库模块

本文档详细介绍了JiLang中的IO标准库模块，该模块提供了基本的输入/输出操作功能。

## 模块概述

IO模块是JiLang内置的标准库模块，提供了文件读写和用户输入等功能。作为内置模块，在任何JiLang程序中都可以直接使用，无需额外安装。

## 导入模块

在JiLang程序中使用以下方式导入IO模块：

```json
{
  "include": ["io"],
  "program": {
    // 程序主体
  }
}
```

## 提供的函数

IO模块提供以下核心功能：

| 函数名 | 说明 |
|-------|------|
| read_file | 读取文件内容 |
| write_file | 将内容写入文件 |
| input | 从用户获取输入 |

### read_file

#### 描述
从指定路径读取文件的内容。

#### 参数
- **path**: 要读取的文件路径（字符串）

#### 返回值
文件内容字符串，如果操作失败则返回错误信息。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.read_file": ["config.txt"], "output": "file_content"},
        {"echo": ["文件内容：\n", "@var.file_content"]}
      ]
    }
  }
}
```

#### 错误处理

如果读取文件失败，函数会返回包含错误信息的字符串：

```
Error: 错误详情
```

可以检查返回值是否以"Error:"开头来判断操作是否成功。

### write_file

#### 描述
将内容写入指定路径的文件。

#### 参数
- **path**: 要写入的文件路径（字符串）
- **content**: 要写入的内容（字符串）

#### 返回值
成功时返回成功消息，失败时返回错误信息。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"var": {"data": "这是测试内容\n这是第二行"}},
        {"io.write_file": ["output.txt", "@var.data"], "output": "write_result"},
        {"echo": ["写入结果：", "@var.write_result"]}
      ]
    }
  }
}
```

#### 错误处理

如果写入文件失败，函数会返回包含错误信息的字符串：

```
Error: 错误详情
```

### input

#### 描述
从用户获取输入。可以提供一个可选的提示信息。

#### 参数
- **prompt**: 可选的提示信息（字符串）

#### 返回值
用户输入的内容。如果输入可以解析为数字，则返回数字值，否则返回字符串。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.input": ["请输入您的名字："], "output": "user_name"},
        {"echo": ["欢迎，", "@var.user_name", "！\n"]},
        
        {"io.input": ["请输入一个数字："], "output": "user_number"},
        {"echo": ["您输入的数字加10等于：", {"op": "add", "left": "@var.user_number", "right": 10}]}
      ]
    }
  }
}
```

#### 类型转换

`input`函数会自动尝试将用户输入解析为数字：
- 如果输入能成功解析为数字，则返回数值类型
- 否则返回字符串类型

这使得获取的数值可以直接用于数学运算，无需手动转换。

## 组合使用示例

### 读取、修改并写回文件

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"comment": "读取原始文件"},
        {"io.read_file": ["data.txt"], "output": "original_content"},
        {"echo": ["原始内容:\n", "@var.original_content", "\n"]},
        
        {"comment": "修改内容"},
        {"concat": {
          "target": "modified_content",
          "parts": ["修改时间: ", {"date": "now"}, "\n", "@var.original_content"]
        }},
        
        {"comment": "写回文件"},
        {"io.write_file": ["data_modified.txt", "@var.modified_content"]},
        {"echo": ["文件已保存为 data_modified.txt"]}
      ]
    }
  }
}
```

### 简单的命令行交互

```json
{
  "include": ["io", "math"],
  "program": {
    "main": {
      "body": [
        {"echo": ["简单计算器\n"]},
        {"io.input": ["请输入第一个数字："], "output": "num1"},
        {"io.input": ["请输入第二个数字："], "output": "num2"},
        {"echo": ["可用操作: 1) 加法 2) 减法 3) 乘法 4) 除法\n"]},
        {"io.input": ["请选择操作(1-4)："], "output": "operation"},
        
        {"switch": {
          "value": "@var.operation",
          "cases": {
            "1": [
              {"math.add": ["@var.num1", "@var.num2"], "output": "result"},
              {"echo": ["结果: ", "@var.num1", " + ", "@var.num2", " = ", "@var.result"]}
            ],
            "2": [
              {"math.subtract": ["@var.num1", "@var.num2"], "output": "result"},
              {"echo": ["结果: ", "@var.num1", " - ", "@var.num2", " = ", "@var.result"]}
            ],
            "3": [
              {"math.multiply": ["@var.num1", "@var.num2"], "output": "result"},
              {"echo": ["结果: ", "@var.num1", " * ", "@var.num2", " = ", "@var.result"]}
            ],
            "4": [
              {"math.divide": ["@var.num1", "@var.num2"], "output": "result"},
              {"echo": ["结果: ", "@var.num1", " / ", "@var.num2", " = ", "@var.result"]}
            ]
          },
          "default": [
            {"echo": ["无效的操作选择"]}
          ]
        }}
      ]
    }
  }
}
```

## 最佳实践

1. **错误处理**: 读取和写入文件时始终检查返回值，确认操作是否成功
2. **路径处理**: 使用相对路径时，注意它们是相对于JiLang程序执行的当前目录
3. **输入验证**: 使用`input`获取用户输入后，验证输入内容是否符合预期格式
4. **资源释放**: JiLang自动处理文件资源的释放，不需要手动关闭文件

## 限制与注意事项

1. IO操作受到操作系统权限限制，确保程序有足够的权限读写目标文件
2. 大文件操作可能会影响性能，特别是当文件内容被读入内存时
3. 文件路径支持相对路径和绝对路径，但在不同操作系统上可能需要调整路径格式
4. 为保证安全性，应当验证用户输入的文件路径，避免访问不应访问的系统文件 