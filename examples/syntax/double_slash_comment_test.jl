// 这是一个使用//注释的文件示例
// 这些注释会在JSON解析前自动被移除

{
    // 这是嵌入在JSON中的注释
    "include": ["math"], // 包含数学模块
    "program": {
        "main": {
            "body": [
                // 使用双斜杠注释
                {"echo": ["测试 // 注释:\n\n"]},
                
                {"var": {"number": 25}}, // 设置一个变量
                {"echo": ["计算", "@var.number", "的平方根\n"]},
                
                // 使用数学模块计算平方根
                {"math.sqrt": ["@var.number"]},
                {"var": {"sqrt_result": "@var.result"}},
                
                // 以下使用传统的JSON对象注释方式
                {"comment": "使用comment对象的注释"},
                {"echo": ["平方根结果: ", "@var.sqrt_result", "\n"]},
                
                // 混合使用注释方式
                {"comment": ["变量值:", "@var.sqrt_result"]}, // 行尾注释
                
                {"echo": ["\n程序执行完毕\n"]}
            ]
        }
    }
} 