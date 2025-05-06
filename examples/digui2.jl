{
    "include": ["math"],
    "program": {
        "factorial": {
            "params": {
                "n": "number"
            },
            "body": [
                {"echo": ["计算 ", "@params.n", "! 的阶乘，非递归版本\n"]},
                {"var": {"result": 1}},
                {"for": {
                    "var": "i",
                    "range": [1, "@params.n"],
                    "body": [
                        {"echo": ["当前 i = ", "@var.i", ", result = ", "@var.result", "\n"]},
                        {"math.multiply": ["@var.result", "@var.i"]},
                        {"var": {"result": "@var.result"}},
                        {"echo": ["新的 result = ", "@var.result", "\n"]}
                    ]
                }},
                {"echo": ["计算完成，", "@params.n", "! = ", "@var.result", "\n"]}
            ]
        },
        "main": {
            "body": [
                {"echo": ["阶乘计算示例（迭代版本）\n\n"]},
                
                {"var": {"number": 5}},
                {"echo": ["计算 ", "@var.number", "! 的结果：\n"]},
                
                {"factorial": ["@var.number"]},
                {"echo": ["最终结果: ", "@var.number", "! = ", "@var.result", "\n\n"]},
                
                {"echo": ["测试不同的输入值：\n"]},
                {"for": {
                    "var": "i",
                    "range": [0, 10],
                    "body": [
                        {"factorial": ["@var.i"]},
                        {"echo": ["最终结果: ", "@var.i", "! = ", "@var.result", "\n\n"]}
                    ]
                }}
            ]
        }
    }
} 