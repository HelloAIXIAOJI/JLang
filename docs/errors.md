# JsonLang 错误处理说明

## 1. 语法错误

### 1.1 JSON 解析错误

当 JSON 格式不正确时，会出现解析错误：

```json
{
    "program": {  // 缺少闭合括号
```

错误信息：`JSON 解析错误: EOF while parsing an object at line 1 column 256`

常见原因：
- 缺少引号或括号
- 多余的逗号
- 无效的转义字符
- JSON 格式不完整

### 1.2 程序结构错误

当程序结构不符合要求时会出现错误：

1. 程序格式错误：
   ```json
   {
       "program": "不是对象"  // program 必须是对象
   }
   ```
   错误信息：`程序结构错误: 程序必须是一个 JSON 对象`

2. 常量定义错误：
   ```json
   {
       "const": "这不是一个对象"  // const 必须是对象
   }
   ```
   错误信息：`程序结构错误: 'const' 必须是一个对象`

3. 函数结构错误：
   ```json
   {
       "program": {
           "main": {
               "body": "不是数组"  // body 必须是数组
           }
       }
   }
   ```
   错误信息：`程序结构错误: 函数 'body' 必须是一个数组`

4. 函数参数错误：
   ```json
   {
       "program": {
           "print_message": {
               "params": "不是对象",  // params 必须是对象
               "body": []
           }
       }
   }
   ```
   错误信息：`程序结构错误: 函数参数必须是一个对象`

## 2. 运行时错误

### 2.1 变量错误

1. 未定义变量：
   ```json
   {"echo": ["@var.undefined_var"]}
   ```
   结果：输出原始文本 `@var.undefined_var`

2. 未定义常量：
   ```json
   {"var": {"name": "@const.UNDEFINED"}}
   ```
   结果：变量值为 `@const.UNDEFINED`

3. 常量重定义：
   ```json
   {"var": {"VERSION": "2.0.0"}}  // VERSION 是常量
   ```
   错误信息：`变量错误: 无法修改常量 'VERSION'`

### 2.2 语句错误

1. 未知语句类型：
   ```json
   {"未知语句": "测试"}
   ```
   错误信息：`运行时错误: 未知的语句类型: 未知语句`

2. 变量定义格式错误：
   ```json
   {"var": "不是对象"}
   ```
   错误信息：`运行时错误: 'var' 语句的参数必须是一个对象`

3. 输出语句格式错误：
   ```json
   {"echo": {"不是数组": "测试"}}
   ```
   结果：不输出任何内容（echo 需要数组参数）

4. 字符串拼接错误：
   ```json
   {"concat": {
       "target": 123,        // target 必须是字符串
       "parts": "不是数组"   // parts 必须是数组
   }}
   ```
   错误信息：`运行时错误: 'concat' 语句的参数格式错误`

5. 条件语句格式错误：
   ```json
   {"if": {
       "condition": "不是对象",  // condition 必须是对象
       "then": []
   }}
   ```
   结果：条件判断为 false

### 2.3 函数错误

1. 函数调用格式错误：
   ```json
   {"call": "不是数组"}
   ```
   错误信息：`运行时错误: 'call' 语句的参数必须是一个数组`

2. 空函数调用：
   ```json
   {"call": []}
   ```
   错误信息：`函数错误: 函数调用缺少函数名`

3. 无效函数名：
   ```json
   {"call": [123]}
   ```
   错误信息：`函数错误: 函数名必须是字符串`

4. 未定义函数：
   ```json
   {"call": ["undefined_func"]}
   ```
   错误信息：`函数错误: 未找到函数 'undefined_func'`

### 2.4 模块错误

1. 未知模块：
   ```json
   {
       "include": ["unknown_module"]
   }
   ```
   错误信息：`模块错误: 未知的模块: unknown_module`

2. 未包含模块：
   ```json
   {"call": ["math.sqrt", 16]}  // 未在 include 中声明 math
   ```
   错误信息：`模块错误: 未找到模块 'math'`

3. 未知模块函数：
   ```json
   {"call": ["io.unknown", "arg"]}
   ```
   错误信息：`模块错误: 模块 'io' 中未找到函数 'unknown'`

4. 模块函数参数错误：
   ```json
   {"call": ["math.sqrt", "不是数字"]}
   ```
   结果：返回 0（数学函数对无效输入的处理）

### 2.5 IO 错误

1. 文件读取错误：
   ```json
   {"call": ["io.read_file"]}  // 缺少参数
   ```
   错误信息：`Error: No file path provided`

2. 文件不存在：
   ```json
   {"call": ["io.read_file", "不存在的文件.txt"]}
   ```
   错误信息：`Error: No such file or directory`

3. 文件写入权限错误：
   ```json
   {"call": ["io.write_file", "/root/forbidden.txt", "测试"]}
   ```
   错误信息：`Error: Permission denied`

## 3. 错误处理最佳实践

1. **程序结构验证**：
   - 确保所有必需字段存在且格式正确
   - 验证函数定义的完整性
   - 检查数组和对象的使用是否正确

2. **变量和常量使用**：
   - 在使用变量前确保已定义
   - 不要尝试修改常量
   - 注意变量引用的格式（@var.、@const.）

3. **函数调用**：
   - 确保函数已定义
   - 检查参数数量和类型
   - 注意模块函数的调用格式

4. **模块使用**：
   - 在 include 中声明所需模块
   - 确认模块函数名称正确
   - 提供正确的参数类型

5. **IO 操作**：
   - 总是提供文件路径
   - 确保有适当的文件权限
   - 处理可能的 IO 错误

## 4. 调试技巧

1. **使用 echo 调试**：
   ```json
   {"echo": ["调试: ", "@var.debug_value", "\n"]}
   ```

2. **分步验证**：
   ```json
   {"var": {"step": "1"}},
   {"echo": ["步骤 ", "@var.step", " 完成\n"]},
   {"var": {"step": "2"}},
   {"echo": ["步骤 ", "@var.step", " 完成\n"]}
   ```

3. **条件检查**：
   ```json
   {"if": {
       "condition": {
           "op": "eq",
           "left": "@var.value",
           "right": "expected"
       },
       "then": [
           {"echo": ["值正确: ", "@var.value", "\n"]}
       ],
       "else": [
           {"echo": ["值错误: ", "@var.value", "\n"]}
       ]
   }}
   ```

4. **错误处理流程**：
   - 先检查 JSON 语法
   - 验证程序结构
   - 检查模块依赖
   - 验证变量和函数定义
   - 测试具体功能 