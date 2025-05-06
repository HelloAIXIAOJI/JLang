{
    "include": ["math"],
    "program": {
        "factorial": {
            "params": {
                "n": "number"
            },
            "body": [
                {"echo": ["计算阶乘: ", "@params.n", "!\n"]},
                {"if": {
                    "condition": {
                        "op": "lte",
                        "left": "@params.n",
                        "right": 1
                    },
                    "then": [
                        {"var": {"result": 1}},
                        {"echo": ["基本情况: ", "@params.n", "! = 1\n"]}
                    ],
                    "else": [
                        {"var": {"i": 1, "result": 1}},
                        
                        {"while": {
                            "condition": {
                                "op": "lte",
                                "left": "@var.i",
                                "right": "@params.n"
                            },
                            "body": [
                                {"echo": ["迭代: i=", "@var.i", ", result=", "@var.result", "\n"]},
                                
                                {"math.multiply": ["@var.result", "@var.i"]},
                                {"var": {"temp_result": "@var.result"}},
                                
                                {"math.add": ["@var.i", 1]},
                                {"var": {"temp_i": "@var.result"}},
                                
                                {"var": {"result": "@var.temp_result", "i": "@var.temp_i"}},
                                {"echo": ["更新后: i=", "@var.i", ", result=", "@var.result", "\n"]}
                            ]
                        }}
                    ]
                }},
                {"echo": ["计算完成: ", "@params.n", "! = ", "@var.result", "\n"]}
            ]
        },
        "main": {
            "body": [
                {"echo": ["阶乘计算（迭代实现）\n\n"]},
                
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