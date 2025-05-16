# JiLang中的Lua与Lua模块开发

本文档详细介绍了JiLang中的Lua集成功能，以及如何使用Lua语言开发JiLang模块。

## Lua集成概述

JiLang提供了与Lua语言的深度集成，允许开发者：

1. 使用Lua编写JiLang模块
2. 从JiLang调用Lua函数
3. 从Lua访问JiLang变量和函数
4. 实现JiLang与Lua之间的数据交换

这种集成为JiLang提供了强大的扩展能力，使开发者能够利用Lua的灵活性和丰富的生态系统来增强JiLang的功能。

## Lua模块基础

### 创建Lua模块

Lua模块是扩展名为`.lua`的文件，可以直接通过JiLang的`include`指令加载。一个基本的Lua模块结构如下：

```lua
-- 模块元数据（可选但推荐）
local module_meta = {
    version = "1.0.0",
    description = "模块描述",
    author = "作者名称"
}

-- 模块函数定义
local function add(a, b)
    -- 处理参数为表的情况（JSON数组转换）
    if type(a) == "table" then
        a, b = a[1], a[2]
    end
    
    return (tonumber(a) or 0) + (tonumber(b) or 0)
}

local function multiply(a, b)
    if type(a) == "table" then
        a, b = a[1], a[2]
    end
    
    return (tonumber(a) or 0) * (tonumber(b) or 0)
}

-- 导出模块内容
return {
    -- 导出元数据
    module_meta = module_meta,
    
    -- 导出函数
    add = add,
    multiply = multiply
}
```

### 导入和使用Lua模块

在JiLang程序中，使用`include`指令导入Lua模块：

```json
{
  "include": ["math_lua"],
  "program": {
    "main": {
      "body": [
        {"math_lua.add": [5, 3], "output": "result"},
        {"echo": ["结果：", "@var.result"]}
      ]
    }
  }
}
```

JiLang解释器会自动查找名为`math_lua.lua`的文件，并将其加载为模块。

## Lua模块详细实现

### 模块元数据

每个Lua模块可以定义`module_meta`表来提供模块的元信息：

```lua
local module_meta = {
    version = "1.0.0",        -- 模块版本
    description = "模块描述", -- 模块描述
    author = "作者名称"       -- 模块作者
}
```

在JiLang中，可以通过以下方式访问模块元数据：

```json
{"echo": ["模块版本: ", "@var.module_meta.math_lua.version"]}
```

### 参数处理

JiLang传递给Lua函数的参数通常是以数组形式传递的，需要在Lua函数中正确处理：

```lua
local function some_function(arg)
    -- 处理数组参数情况
    if type(arg) == "table" then
        -- 提取参数
        local first_param = arg[1]
        local second_param = arg[2]
        -- 处理逻辑...
    else
        -- 处理单个值参数情况
        -- 处理逻辑...
    end
    
    -- 返回结果
    return result
end
```

### 与JiLang交互

Lua模块可以通过`jilang`命名空间与JiLang环境交互：

#### 1. 获取JiLang变量

```lua
-- 从JiLang获取变量
local value = jilang.get_var("变量名")
```

#### 2. 设置JiLang变量

```lua
-- 设置JiLang变量
jilang.set_var("变量名", 值)
```

#### 3. 调用JiLang语句或函数

```lua
-- 调用echo语句
jilang.call("echo", {"这是从Lua调用的echo语句\n"})

-- 调用math.add函数
local result = jilang.call("math.add", {10, 20})
```

#### 4. 输出信息

```lua
-- 输出调试信息
jilang.print("这是一条调试信息")
```

## 数据类型转换

JiLang和Lua之间的数据类型自动转换：

| JiLang类型 | Lua类型 |
|------------|---------|
| 数字       | number  |
| 字符串     | string  |
| 布尔值     | boolean |
| 数组       | table   |
| 对象       | table   |
| null       | nil     |

注意：
- JiLang的对象会转换为Lua的表
- Lua的表会转换为JiLang的数组或对象，取决于表的结构

## 完整示例

### 示例1：基本Lua模块

`math_lua.lua`:
```lua
-- 模块元数据
local module_meta = {
    version = "1.0.0",
    description = "Lua数学模块示例",
    author = "JiLang"
}

-- 计算斐波那契数列
local function fibonacci(n)
    if type(n) == "table" then
        n = n[1]
    end
    
    n = tonumber(n)
    if n == nil then return 0 end
    
    n = math.floor(n)
    if n < 0 then n = 0 end
    
    if n == 0 then return 0 end
    if n == 1 then return 1 end
    
    local a, b = 0, 1
    for i = 2, n do
        a, b = b, a + b
    end
    
    return b
end

-- 模块导出
return {
    module_meta = module_meta,
    fibonacci = fibonacci
}
```

使用示例：
```json
{
  "include": ["math_lua"],
  "program": {
    "main": {
      "body": [
        {"echo": ["斐波那契数列计算：\n"]},
        {"math_lua.fibonacci": [10], "output": "fib_result"},
        {"echo": ["第10项结果：", "@var.fib_result"]}
      ]
    }
  }
}
```

### 示例2：与JiLang交互的Lua模块

`interactive.lua`:
```lua
-- 模块元数据
local module_meta = {
    version = "1.0.0",
    description = "JiLang交互示例",
    author = "JiLang"
}

-- 处理数据并与JiLang交互
local function process_data(data)
    -- 获取输入数据
    if type(data) == "table" then
        data = data[1]
    end
    
    -- 创建JiLang变量
    jilang.set_var("lua_processing", true)
    
    -- 调用JiLang函数
    local result = jilang.call("math.add", {10, 20})
    
    -- 输出调试信息
    jilang.print("Lua处理中...")
    
    -- 构建响应
    local response = {
        original_data = data,
        processed = true,
        jilang_result = result
    }
    
    -- 设置响应到JiLang变量
    jilang.set_var("lua_response", response)
    
    return response
end

-- 导出模块
return {
    module_meta = module_meta,
    process_data = process_data
}
```

## 最佳实践

1. **始终提供模块元数据**：增强模块可发现性和文档
2. **健壮的参数处理**：确保函数能处理不同形式的输入
3. **类型检查与转换**：显式处理类型转换以避免错误
4. **错误处理**：使用适当的错误处理机制
5. **避免全局变量**：使用本地变量避免污染全局命名空间

## 调试技巧

1. 使用`jilang.print()`输出调试信息
2. 将中间结果存储到JiLang变量中便于检查
3. 在JiLang程序中使用`--debug`模式运行可查看更多信息

```bash
cargo run -- --debug path/to/program.jl
```

## 注意事项

1. Lua模块文件名必须与`include`中指定的名称相符（加上`.lua`扩展名）
2. JiLang会在当前目录、`modules/`目录和标准库路径中查找Lua模块
3. Lua模块中函数名不能与JiLang内置语句或其他模块函数冲突
4. 大型数据结构在JiLang和Lua之间传递可能有性能影响

## 高级应用

### 使用Lua标准库

Lua模块可以利用Lua标准库提供强大功能：

```lua
-- 使用Lua字符串库
local function process_string(str)
    if type(str) == "table" then
        str = str[1]
    end
    
    local result = {
        upper = string.upper(str),
        lower = string.lower(str),
        length = string.len(str),
        reverse = string.reverse(str)
    }
    
    return result
end
```

### 递归算法

```lua
-- 递归计算阶乘
local function factorial(n)
    if type(n) == "table" then
        n = n[1]
    end
    
    n = tonumber(n) or 0
    if n <= 1 then return 1 end
    
    return n * factorial(n - 1)
end
```

### 复杂数据处理

```lua
-- 数据分析示例
local function analyze_data(data)
    if type(data) ~= "table" then
        return {error = "需要数组数据"}
    end
    
    local sum = 0
    local min = data[1] or 0
    local max = data[1] or 0
    
    for _, value in ipairs(data) do
        sum = sum + (tonumber(value) or 0)
        min = math.min(min, value)
        max = math.max(max, value)
    end
    
    return {
        sum = sum,
        average = #data > 0 and (sum / #data) or 0,
        min = min,
        max = max,
        count = #data
    }
end
```

## 结论

Lua模块为JiLang提供了强大的扩展能力，使开发者能够使用熟悉的Lua语言编写复杂功能。通过JiLang与Lua的无缝集成，可以构建更灵活、更强大的应用程序，充分发挥两种语言的优势。 