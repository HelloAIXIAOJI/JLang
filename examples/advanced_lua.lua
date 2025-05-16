-- advanced_lua.lua - 高级Lua模块示例，展示与JiLang的交互

-- 模块元数据
local module_meta = {
    version = "1.0.0",
    description = "JiLang高级Lua交互示例",
    author = "JiLang"
}

-- 测试JiLang变量访问
local function test_variables()
    -- 从JiLang获取变量
    local result = {}
    result.msg = "从Lua访问JiLang变量"
    
    -- 创建一个JiLang变量
    jilang.set_var("lua_created_var", "这是Lua创建的变量")
    
    -- 读取JiLang变量
    local existing_var = jilang.get_var("lua_created_var")
    result.read_var = existing_var
    
    -- 修改JiLang变量
    jilang.set_var("lua_created_var", "这是修改后的变量值")
    result.modified_var = jilang.get_var("lua_created_var")
    
    return result
end

-- 测试调用JiLang函数
local function test_jilang_call()
    local result = {}
    
    -- 调用echo语句
    jilang.call("echo", {"这是从Lua调用的echo语句\n"})
    
    -- 调用math.add函数并获取结果
    local add_result = jilang.call("math.add", {10, 20})
    result.add_result = add_result
    
    -- 创建数组并获取长度
    local array_result = jilang.call("array.create", {{1, 2, 3, 4, 5}})
    jilang.set_var("lua_array", array_result)
    local length_result = jilang.call("array.length", {"@var.lua_array"})
    result.array_length = length_result
    
    return result
end

-- 复杂数据处理示例
local function process_data(data)
    -- 如果没有提供数据，尝试从JiLang获取
    if not data then
        data = jilang.get_var("input_data")
        if not data then
            return {error = "没有提供数据"}
        end
    end
    
    -- 打印收到的数据
    jilang.print("Lua收到数据: " .. tostring(data))
    
    -- 简单处理：如果是数字，计算平方；如果是字符串，返回长度
    local result = {}
    
    if type(data) == "number" then
        result.original = data
        result.squared = data * data
        result.type = "number"
    elseif type(data) == "string" then
        result.original = data
        result.length = string.len(data)
        result.type = "string"
    elseif type(data) == "table" then
        result.original = "table"
        result.size = #data
        result.processed = {}
        
        -- 处理表中的每个元素
        for i, v in ipairs(data) do
            if type(v) == "number" then
                result.processed[i] = v * 2
            elseif type(v) == "string" then
                result.processed[i] = string.upper(v)
            else
                result.processed[i] = tostring(v)
            end
        end
        result.type = "table"
    else
        result.original = tostring(data)
        result.type = type(data)
    end
    
    -- 将结果存储到JiLang变量
    jilang.set_var("lua_processing_result", result)
    
    return result
end

-- 高级示例：在JiLang和Lua之间来回传递数据
local function interactive_demo()
    -- 创建一个包含任务的JiLang变量
    jilang.set_var("tasks", {
        {name = "任务1", done = false},
        {name = "任务2", done = true},
        {name = "任务3", done = false}
    })
    
    -- 获取任务列表
    local tasks = jilang.get_var("tasks")
    
    -- 处理任务
    local completed = 0
    local pending = 0
    
    for _, task in ipairs(tasks) do
        if task.done then
            completed = completed + 1
        else
            pending = pending + 1
        end
    end
    
    -- 创建报告
    local report = {
        total = #tasks,
        completed = completed,
        pending = pending,
        completion_rate = (completed / #tasks) * 100
    }
    
    -- 将报告保存到JiLang变量
    jilang.set_var("task_report", report)
    
    -- 调用JiLang函数显示报告
    jilang.call("echo", {
        "任务报告：\n",
        "总任务数: ", tostring(report.total), "\n",
        "已完成: ", tostring(report.completed), "\n",
        "待完成: ", tostring(report.pending), "\n",
        "完成率: ", string.format("%.1f%%", report.completion_rate), "\n"
    })
    
    return report
end

-- 模块导出表
return {
    module_meta = module_meta,
    test_variables = test_variables,
    test_jilang_call = test_jilang_call,
    process_data = process_data,
    interactive_demo = interactive_demo
} 