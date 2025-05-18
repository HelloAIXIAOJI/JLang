# JiLang HTTP模块

HTTP模块为JiLang提供了进行网络请求的能力，支持标准的HTTP方法（GET、POST、PUT、DELETE）以及URL编码/解码功能。

## 模块概述

HTTP模块是JiLang的内置模块，提供以下功能：
- 发送HTTP请求（GET、POST、PUT、DELETE）
- 设置请求头和请求体
- 处理HTTP响应
- URL编码和解码
- 超时控制

## 导入模块

在JiLang程序中使用以下语句导入HTTP模块：

```json
{
  "include": ["http"],
  "program": {
    // 程序主体
  }
}
```

## 函数参考

### http.get

发送HTTP GET请求。

**参数：**
- `url`：请求的URL地址（字符串）
- `headers`（可选）：请求头（对象）
- `timeout`（可选）：超时时间，单位为秒（数字）

**返回值：**
返回包含以下字段的对象：
- `status`：HTTP状态码
- `headers`：响应头
- `body`：响应体（自动解析JSON）
- `raw`：原始响应文本
- `error`：如果请求失败，包含错误信息

**示例：**

```json
{
  "http.get": ["https://api.example.com/data"], 
  "output": "response"
}
```

带请求头的示例：

```json
{
  "var": {"headers": {
    "Authorization": "Bearer token123",
    "Accept": "application/json"
  }},
  "http.get": ["https://api.example.com/data", "@var.headers"], 
  "output": "response"
}
```

### http.post

发送HTTP POST请求。

**参数：**
- `url`：请求的URL地址（字符串）
- `body`：请求体数据（任意类型）
- `headers`（可选）：请求头（对象）
- `timeout`（可选）：超时时间，单位为秒（数字）

**返回值：**
与`http.get`相同的响应对象结构。

**示例：**

```json
{
  "var": {"post_data": {
    "name": "JiLang",
    "version": 0.4
  }},
  "http.post": [
    "https://api.example.com/data", 
    "@var.post_data"
  ], 
  "output": "response"
}
```

### http.put

发送HTTP PUT请求。

**参数：**
- `url`：请求的URL地址（字符串）
- `body`：请求体数据（任意类型）
- `headers`（可选）：请求头（对象）
- `timeout`（可选）：超时时间，单位为秒（数字）

**返回值：**
与`http.get`相同的响应对象结构。

**示例：**

```json
{
  "var": {"put_data": {
    "id": 1,
    "name": "Updated Item"
  }},
  "http.put": [
    "https://api.example.com/items/1", 
    "@var.put_data"
  ], 
  "output": "response"
}
```

### http.delete

发送HTTP DELETE请求。

**参数：**
- `url`：请求的URL地址（字符串）
- `headers`（可选）：请求头（对象）
- `timeout`（可选）：超时时间，单位为秒（数字）

**返回值：**
与`http.get`相同的响应对象结构。

**示例：**

```json
{
  "http.delete": ["https://api.example.com/items/1"], 
  "output": "response"
}
```

### http.url_encode

对字符串进行URL编码。

**参数：**
- `text`：需要编码的文本（字符串）

**返回值：**
URL编码后的字符串。

**示例：**

```json
{
  "var": {"raw_text": "JiLang 是一种基于JSON语法的语言"},
  "http.url_encode": ["@var.raw_text"], 
  "output": "encoded"
}
```

### http.url_decode

对URL编码的字符串进行解码。

**参数：**
- `encoded_text`：编码后的文本（字符串）

**返回值：**
解码后的原始字符串，或包含错误信息的对象。

**示例：**

```json
{
  "http.url_decode": ["JiLang%20%E6%98%AF%E4%B8%80%E7%A7%8D%E5%9F%BA%E4%BA%8EJSON%E7%9A%84%E8%AF%AD%E8%A8%80"], 
  "output": "decoded"
}
```

## 错误处理

HTTP模块的函数会在遇到错误时返回包含`error`字段的对象。建议在使用HTTP模块时添加错误处理逻辑。

示例：

```json
{
  "http.get": ["https://non-existent-domain.com"], 
  "output": "response"
},
{
  "if": {
    "condition": {"op": "has_key", "object": "@var.response", "key": "error"},
    "then": [
      {"echo": ["请求失败: ", "@var.response.error", "\n"]}
    ],
    "else": [
      {"echo": ["请求成功: ", "@var.response.body", "\n"]}
    ]
  }
}
```

## 请求头处理

HTTP模块允许以对象形式指定请求头。常见的请求头包括：

- `Content-Type`：指定请求体的格式（如`application/json`）
- `Authorization`：提供身份验证信息
- `User-Agent`：标识客户端
- `Accept`：指定期望的响应格式

请求头示例：

```json
{
  "var": {"headers": {
    "Content-Type": "application/json",
    "Authorization": "Bearer your-token-here",
    "User-Agent": "JiLang-Client/0.4",
    "Accept": "application/json"
  }},
  "http.post": [
    "https://api.example.com/data", 
    {"key": "value"},
    "@var.headers"
  ], 
  "output": "response"
}
```

## JSON响应处理

HTTP模块会自动尝试将响应解析为JSON。如果解析成功，响应的`body`字段将包含解析后的JSON对象，否则包含原始文本。

可以使用JiLang的对象访问语法来访问JSON响应中的属性：

```json
{
  "http.get": ["https://api.example.com/user/1"], 
  "output": "user_response"
},
{
  "echo": ["用户名: ", "@var.user_response.body.name", "\n"]},
  "echo": ["邮箱: ", "@var.user_response.body.email", "\n"]}
}
```

## 完整示例

以下是一个使用HTTP模块进行API交互的完整示例：

```json
{
  "include": ["http", "io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["=== HTTP API 客户端 ===\n"]},
        
        {"var": {"base_url": "https://jsonplaceholder.typicode.com"}},
        
        {"comment": "获取所有帖子"},
        {"http.get": ["@var.base_url/posts"], "output": "all_posts"},
        {"echo": ["获取到 ", "@var.all_posts.body.length", " 篇帖子\n"]},
        
        {"comment": "获取特定帖子"},
        {"http.get": ["@var.base_url/posts/1"], "output": "post"},
        {"echo": ["帖子标题: ", "@var.post.body.title", "\n"]},
        
        {"comment": "创建新帖子"},
        {"var": {"new_post": {
          "title": "JiLang HTTP模块测试",
          "body": "这是使用JiLang HTTP模块创建的帖子",
          "userId": 1
        }}},
        {"http.post": ["@var.base_url/posts", "@var.new_post"], "output": "created_post"},
        {"echo": ["创建的帖子ID: ", "@var.created_post.body.id", "\n"]},
        
        {"comment": "更新帖子"},
        {"var": {"updated_post": {
          "id": 1,
          "title": "更新的标题",
          "body": "这是更新后的内容",
          "userId": 1
        }}},
        {"http.put": ["@var.base_url/posts/1", "@var.updated_post"], "output": "put_result"},
        {"echo": ["更新结果: ", "@var.put_result.body", "\n"]},
        
        {"comment": "删除帖子"},
        {"http.delete": ["@var.base_url/posts/1"], "output": "delete_result"},
        {"echo": ["删除状态码: ", "@var.delete_result.status", "\n"]}
      ]
    }
  }
}
```

## 最佳实践

1. **设置超时**：对于可能需要较长时间的请求，设置合理的超时时间。
2. **错误处理**：始终检查请求是否成功，处理可能的错误。
3. **使用变量**：将重复使用的URL、请求头等保存为变量。
4. **处理大型响应**：对于可能返回大量数据的API请求，考虑如何有效处理。
5. **安全性**：敏感信息（如API密钥、认证令牌）应妥善保存。 