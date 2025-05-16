-- math_lua.lua
-- JiLang示例Lua模块

-- 模块元数据
local module_meta = {
    version = "1.0.2",
    description = "Lua数学模块示例",
    author = "JiLang"
}

-- 斐波那契数列计算（迭代实现）
local function fibonacci(n)
    -- 处理参数为表的情况（JSON数组转换）
    if type(n) == "table" then
        -- 获取第一个元素
        n = n[1]
    end
    
    -- 确保n是数字
    n = tonumber(n)
    if n == nil then
        return 0
    end
    
    -- 确保n是非负整数
    n = math.floor(n)
    if n < 0 then n = 0 end
    
    -- 基本情况
    if n == 0 then return 0 end
    if n == 1 then return 1 end
    
    -- 迭代计算
    local a, b = 0, 1
    for i = 2, n do
        a, b = b, a + b
    end
    
    return b
end

-- 阶乘计算（迭代实现）
local function factorial(n)
    -- 处理参数为表的情况（JSON数组转换）
    if type(n) == "table" then
        -- 获取第一个元素
        n = n[1]
    end
    
    n = tonumber(n) or 0
    n = math.floor(n)
    if n < 0 then n = 0 end
    
    if n <= 1 then return 1 end
    
    local result = 1
    for i = 2, n do
        result = result * i
    end
    
    return result
end

-- 加法函数
local function add(a, b)
    -- 处理参数为表的情况（JSON数组转换）
    if type(a) == "table" then
        -- 获取第一个和第二个元素
        a, b = a[1], a[2]
    end
    
    a = tonumber(a) or 0
    b = tonumber(b) or 0
    
    return a + b
end

-- 乘法函数
local function multiply(a, b)
    -- 处理参数为表的情况（JSON数组转换）
    if type(a) == "table" then
        -- 获取第一个和第二个元素
        a, b = a[1], a[2]
    end
    
    a = tonumber(a) or 0
    b = tonumber(b) or 0
    
    return a * b
end

-- 计算平方
local function square(n)
    -- 处理参数为表的情况（JSON数组转换）
    if type(n) == "table" then
        -- 获取第一个元素
        n = n[1]
    end
    
    n = tonumber(n) or 0
    
    return n * n
end

-- 测试JiLang交互
local function test_jilang_interaction()
    -- 设置JiLang变量
    jilang.set_var("lua_created_var", "这是Lua创建的变量")
    
    -- 获取JiLang变量
    local jl_var = jilang.get_var("test_var")
    
    -- 调用JiLang的echo语句
    jilang.call("echo", {"这是从Lua调用的echo语句\n"})
    
    return {
        message = "Lua和JiLang交互成功",
        jilang_var = jl_var
    }
end

-- 检测类型转换
local function check_types(n)
    -- 处理参数为表的情况（JSON数组转换）
    if type(n) == "table" then
        -- 获取第一个元素
        n = n[1]
    end
    
    return {
        original = n,
        type = type(n),
        tostring = tostring(n),
        plus_one = tonumber(n) and (tonumber(n) + 1) or "不是数字"
    }
end

-- 模块导出表
return {
    -- 导出元数据
    module_meta = module_meta,
    
    -- 导出函数
    fibonacci = fibonacci,
    factorial = factorial,
    add = add,
    multiply = multiply,
    square = square,
    test_jilang_interaction = test_jilang_interaction,
    check_types = check_types
} 