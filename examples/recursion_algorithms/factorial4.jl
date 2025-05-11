{
    "include": ["math"],
    "program": {
        "factorial": {
            "params": {
                "n": "number"
            },
            "body": [
                {"echo": ["计算阶乘: ", "@params.n", "!"]},
                {"if": {
                    "condition": {
                        "op": "lte",
                        "left": "@params.n",
                        "right": 1
                    },
                    "then": [
                        {"var": {"factorial_result": 1}},
                        {"echo": [" (基本情况)"]}
                    ],
                    "else": [
                        {"math.subtract": ["@params.n", 1]},
                        {"var": {"n_minus_1": "@var.result"}},
                        
                        {"factorial": ["@var.n_minus_1"]},
                        {"var": {"factorial_of_n_minus_1": "@var.factorial_result"}},
                        {"echo": [" (递归: ", "@var.n_minus_1", "! = ", "@var.factorial_of_n_minus_1", ")"]},
                        
                        {"math.multiply": ["@params.n", "@var.factorial_of_n_minus_1"]},
                        {"var": {"factorial_result": "@var.result"}}
                    ]
                }},
                {"echo": ["\n结果: ", "@params.n", "! = ", "@var.factorial_result"]}
            ]
        },
        "main": {
            "body": [
                {"echo": ["阶乘计算（改进的递归实现）\n\n"]},
                
                {"var": {"number": 5}},
                {"echo": ["计算 ", "@var.number", "! 的结果：\n"]},
                
                {"factorial": ["@var.number"]},
                {"echo": ["\n\n最终结果: ", "@var.number", "! = ", "@var.factorial_result", "\n\n"]},
                
                {"echo": ["测试其他输入值：\n"]},
                {"for": {
                    "var": "i",
                    "range": [0, 5],
                    "body": [
                        {"factorial": ["@var.i"]},
                        {"echo": ["\n"]}
                    ]
                }}
            ]
        }
    }
} 