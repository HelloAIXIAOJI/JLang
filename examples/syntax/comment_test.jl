{
    "include": ["math"],
    "program": {
        "main": {
            "body": [
                {"comment": "程序开始"},
                {"echo": ["测试注释功能：\n\n"]},
                
                {"var": {"number": 16, "author": "JsonLang开发者"}},
                {"echo": ["初始数值: ", "@var.number", "\n"]},
                
                {"comment": ["接下来计算", "@var.number", "的平方根"]},
                {"math.sqrt": ["@var.number"]},
                {"var": {"sqrt_result": "@var.result"}},
                {"echo": ["平方根结果: ", "@var.sqrt_result", "\n"]},
                
                {"comment": ["由", "@var.author", "编写的测试程序"]},
                {"echo": ["作者: ", "@var.author", "\n"]},
                
                {"comment": "循环测试"},
                {"var": {"counter": 1}},
                {"while": {
                    "condition": {
                        "op": "lte",
                        "left": "@var.counter",
                        "right": 3
                    },
                    "body": [
                        {"comment": ["第", "@var.counter", "次循环"]},
                        {"echo": ["循环计数: ", "@var.counter", "\n"]},
                        {"math.add": ["@var.counter", 1]},
                        {"var": {"counter": "@var.result"}}
                    ]
                }},
                
                {"comment": ["程序结束，平方根结果: ", "@var.sqrt_result"]},
                {"echo": ["\n程序执行完毕\n"]}
            ]
        }
    }
} 