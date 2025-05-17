# IO标准库模块

本文档详细介绍了JiLang中的IO标准库模块，该模块提供了基本的输入/输出和文件系统操作功能。

## 模块概述

IO模块是JiLang内置的标准库模块，提供了文件读写、目录操作、用户输入和JSON处理等功能。作为内置模块，在任何JiLang程序中都可以直接使用，无需额外安装。

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

### 文件操作函数

| 函数名 | 说明 |
|-------|------|
| read_file | 读取文件内容 |
| write_file | 将内容写入文件 |
| append_file | 向文件追加内容 |
| file_exists | 检查文件是否存在 |
| delete_file | 删除指定文件 |
| list_dir | 列出目录内容 |

### 用户交互函数

| 函数名 | 说明 |
|-------|------|
| input | 从用户获取输入 |
| input_number | 获取数字输入并验证范围 |
| input_with_default | 获取带默认值的输入 |
| confirm | 获取用户确认(y/n) |

### JSON操作函数

| 函数名 | 说明 |
|-------|------|
| read_json | 读取并解析JSON文件 |
| write_json | 将对象写入为JSON文件 |

## 文件操作函数

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
将内容写入指定路径的文件，如果文件已存在则覆盖。

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

### append_file

#### 描述
向文件追加内容，如果文件不存在则创建新文件。

#### 参数
- **path**: 要追加的文件路径（字符串）
- **content**: 要追加的内容（字符串）

#### 返回值
成功时返回成功消息，失败时返回错误信息。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"var": {"log_entry": "INFO: 操作完成 [时间戳]\n"}},
        {"io.append_file": ["logs.txt", "@var.log_entry"], "output": "append_result"},
        {"echo": ["追加结果: ", "@var.append_result"]}
      ]
    }
  }
}
```

#### 与write_file的区别

| write_file | append_file |
|------------|-------------|
| 覆盖原有内容 | 保留原有内容并追加 |
| 常用于创建或完全更新文件 | 常用于日志或持续添加记录 |

### file_exists

#### 描述
检查指定路径的文件是否存在。

#### 参数
- **path**: 要检查的文件路径（字符串）

#### 返回值
布尔值：存在返回true，不存在返回false。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.file_exists": ["config.txt"], "output": "config_exists"},
        {"if": {
          "condition": "@var.config_exists",
          "then": [
            {"io.read_file": ["config.txt"], "output": "config"}
          ],
          "else": [
            {"echo": ["配置文件不存在，使用默认配置"]}
          ]
        }}
      ]
    }
  }
}
```

### delete_file

#### 描述
删除指定路径的文件。

#### 参数
- **path**: 要删除的文件路径（字符串）

#### 返回值
成功时返回成功消息，失败时返回错误信息。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.delete_file": ["temp_file.txt"], "output": "delete_result"},
        {"echo": ["删除结果: ", "@var.delete_result"]}
      ]
    }
  }
}
```

#### 安全注意事项
使用此函数时应特别注意：
- 确保删除的是正确的文件路径
- 避免删除重要系统文件
- 建议先使用`file_exists`检查文件是否存在，再执行删除操作

### list_dir

#### 描述
列出指定目录中的文件和子目录。

#### 参数
- **path**: 要列出内容的目录路径（字符串）

#### 返回值
包含"files"和"directories"两个数组的对象：
- **files**: 文件名列表
- **directories**: 子目录名列表

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.list_dir": ["examples"], "output": "dir_content"},
        {"echo": ["文件列表:\n"]},
        {"foreach": {
          "array": "@var.dir_content.files",
          "var": "file",
          "body": [
            {"echo": ["- ", "@var.file", "\n"]}
          ]
        }},
        {"echo": ["\n目录列表:\n"]},
        {"foreach": {
          "array": "@var.dir_content.directories",
          "var": "dir",
          "body": [
            {"echo": ["- ", "@var.dir", "/\n"]}
          ]
        }}
      ]
    }
  }
}
```

## 用户交互函数

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

### input_number

#### 描述
专门用于获取数字输入，支持范围验证。如果用户输入非数字或范围外的值，会提示重新输入。

#### 参数
- **prompt**: 提示信息（字符串）
- **min**: (可选) 最小值（数字），默认为负无穷
- **max**: (可选) 最大值（数字），默认为正无穷

#### 返回值
用户输入的数字值。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.input_number": ["请输入年龄(1-120): ", 1, 120], "output": "age"},
        {"echo": ["您的年龄是: ", "@var.age", "岁\n"]},
        
        {"io.input_number": ["请输入评分(1-5): ", 1, 5], "output": "rating"},
        {"echo": ["您的评分: ", "@var.rating", "星\n"]}
      ]
    }
  }
}
```

### input_with_default

#### 描述
获取带默认值的用户输入。如果用户直接按回车，则使用默认值。

#### 参数
- **prompt**: 提示信息（字符串）
- **default**: 默认值（任意类型）

#### 返回值
用户输入的内容，如果为空则返回默认值。与input类似，会尝试转换为数字。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.input_with_default": ["请输入用户名", "guest"], "output": "username"},
        {"echo": ["用户名: ", "@var.username", "\n"]},
        
        {"io.input_with_default": ["请输入端口号", 8080], "output": "port"},
        {"echo": ["端口: ", "@var.port", "\n"]}
      ]
    }
  }
}
```

### confirm

#### 描述
获取用户确认（是/否）。可以接受y/yes/是表示确认，n/no/否表示拒绝。

#### 参数
- **prompt**: 提示信息（字符串）
- **default_yes**: (可选) 是否默认为"是"（布尔值），默认为false

#### 返回值
布尔值：确认为true，拒绝为false。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.confirm": ["确认删除文件吗?"], "output": "confirm_delete"},
        {"if": {
          "condition": "@var.confirm_delete",
          "then": [
            {"echo": ["文件已删除\n"]}
          ],
          "else": [
            {"echo": ["操作已取消\n"]}
          ]
        }},
        
        {"io.confirm": ["保存更改?(默认是)", true], "output": "save_changes"},
        {"echo": ["保存状态: ", "@var.save_changes", "\n"]}
      ]
    }
  }
}
```

## JSON操作函数

### read_json

#### 描述
读取JSON文件并解析为JiLang对象。

#### 参数
- **path**: 要读取的JSON文件路径（字符串）

#### 返回值
解析后的JSON对象或数组。解析失败时返回错误信息。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"io.read_json": ["config.json"], "output": "config"},
        {"echo": ["配置名称: ", "@var.config.name", "\n"]},
        {"echo": ["配置版本: ", "@var.config.version", "\n"]},
        {"echo": ["数据库URL: ", "@var.config.database.url", "\n"]}
      ]
    }
  }
}
```

### write_json

#### 描述
将JiLang对象序列化为JSON并写入文件。

#### 参数
- **path**: 要写入的文件路径（字符串）
- **data**: 要序列化的数据（对象或数组）
- **pretty**: (可选) 是否格式化输出（布尔值），默认为false

#### 返回值
成功时返回成功消息，失败时返回错误信息。

#### 示例

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"var": {"user_data": {
          "name": "测试用户",
          "age": 30,
          "interests": ["编程", "阅读"],
          "contact": {
            "email": "test@example.com",
            "phone": "123456789"
          }
        }}},
        
        {"io.write_json": ["user.json", "@var.user_data", true], "output": "write_result"},
        {"echo": ["写入结果: ", "@var.write_result", "\n"]}
      ]
    }
  }
}
```

## 组合使用示例

### 文件处理工作流

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"comment": "检查配置文件是否存在"},
        {"io.file_exists": ["config.json"], "output": "config_exists"},
        
        {"if": {
          "condition": "@var.config_exists",
          "then": [
            {"comment": "读取配置文件"},
            {"io.read_json": ["config.json"], "output": "config"},
            {"echo": ["已加载配置: ", "@var.config.app_name", " v", "@var.config.version", "\n"]}
          ],
          "else": [
            {"comment": "创建默认配置"},
            {"var": {"default_config": {
              "app_name": "JiLang应用",
              "version": "1.0.0",
              "debug": false,
              "paths": {
                "data": "./data",
                "logs": "./logs"
              }
            }}},
            {"io.write_json": ["config.json", "@var.default_config", true]},
            {"echo": ["已创建默认配置文件\n"]}
          ]
        }},
        
        {"comment": "初始化日志文件"},
        {"var": {"log_entry": "应用启动: " + {"date": "now"} + "\n"}},
        {"io.append_file": ["logs.txt", "@var.log_entry"]}
      ]
    }
  }
}
```

### 交互式数据收集

```json
{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["=== 用户信息收集 ===\n"]},
        
        {"io.input_with_default": ["请输入姓名", "匿名用户"], "output": "user_name"},
        {"io.input_number": ["请输入年龄", 1, 120], "output": "user_age"},
        {"io.input": ["请输入邮箱"], "output": "user_email"},
        
        {"io.confirm": ["信息是否正确?"], "output": "is_confirmed"},
        
        {"if": {
          "condition": "@var.is_confirmed",
          "then": [
            {"var": {"user_data": {
              "name": "@var.user_name",
              "age": "@var.user_age",
              "email": "@var.user_email",
              "created_at": {"date": "now"}
            }}},
            {"io.write_json": ["user_profile.json", "@var.user_data", true]},
            {"echo": ["用户信息已保存\n"]}
          ],
          "else": [
            {"echo": ["操作已取消\n"]}
          ]
        }}
      ]
    }
  }
}
```

### 目录浏览和文件管理

```json
{
  "include": ["io"],
  "program": {
    "browse_dir": {
      "params": ["dir_path"],
      "body": [
        {"io.list_dir": ["@param.dir_path"], "output": "dir_content"},
        
        {"echo": ["\n当前目录: ", "@param.dir_path", "\n"]},
        {"echo": ["文件列表:\n"]},
        
        {"foreach": {
          "array": "@var.dir_content.files",
          "var": "file",
          "body": [
            {"echo": ["- ", "@var.file", "\n"]}
          ]
        }},
        
        {"echo": ["\n目录列表:\n"]},
        {"foreach": {
          "array": "@var.dir_content.directories",
          "var": "dir",
          "body": [
            {"echo": ["- ", "@var.dir", "/\n"]}
          ]
        }},
        
        {"echo": ["\n"]},
        {"io.input": ["输入目录名以浏览，或输入'/back'返回: "], "output": "selection"},
        
        {"if": {
          "condition": {"op": "neq", "left": "@var.selection", "right": "/back"},
          "then": [
            {"var": {"new_path": "@param.dir_path" + "/" + "@var.selection"}},
            {"io.file_exists": ["@var.new_path"], "output": "is_dir_exists"},
            {"if": {
              "condition": "@var.is_dir_exists",
              "then": [
                {"browse_dir": ["@var.new_path"]}
              ],
              "else": [
                {"echo": ["目录不存在\n"]},
                {"browse_dir": ["@param.dir_path"]}
              ]
            }}
          ]
        }}
      ]
    },
    "main": {
      "body": [
        {"echo": ["=== 简易文件浏览器 ===\n"]},
        {"browse_dir": ["."]}
      ]
    }
  }
}
```

## 最佳实践

1. **文件路径处理**：
   - 使用相对路径时要注意当前工作目录
   - 检查文件是否存在再进行操作，避免错误
   - 使用`try-catch`语句块处理可能的文件操作异常

2. **用户输入处理**：
   - 为用户输入提供明确的提示信息
   - 使用`input_number`和范围验证来获取有效的数值输入
   - 对关键操作使用`confirm`获取用户确认

3. **JSON处理**：
   - 写入JSON文件时建议设置`pretty=true`以提高可读性
   - 处理JSON数据时，确保访问存在的属性以避免错误
   - 使用变量暂存多次使用的嵌套对象属性

4. **错误处理**：
   - 检查返回值中是否包含"Error:"前缀来判断操作是否成功
   - 为关键IO操作添加错误处理逻辑
   - 记录错误信息到日志文件以便调试

## 限制与注意事项

1. IO操作受到操作系统权限限制，确保程序有足够的权限读写目标文件
2. 大文件操作可能会影响性能，特别是当文件内容被完全读入内存时
3. 文件路径支持相对路径和绝对路径，但在不同操作系统上可能需要调整路径格式
4. 为保证安全性，应当验证用户输入的文件路径，避免访问敏感系统文件
5. JSON文件处理仅支持UTF-8编码，确保文件使用正确的编码格式 