{
    "include": ["math"],
    "program": {
        "factorial_helper": {
            "params": {
                "n": "number"
            },
            "body": [
                {"if": {
                    "condition": {
                        "op": "lte",
                        "left": "@params.n",
                        "right": 1
                    },
                    "then": [
                        {"var": {"result": 1}}
                    ],
                    "else": [
                        {"math.subtract": ["@params.n", 1]},
                        {"var": {"n_minus_1": "@var.result"}},
                        {"factorial_helper": ["@var.n_minus_1"]},
                        {"var": {"sub_result": "@var.result"}},
                        {"math.multiply": ["@params.n", "@var.sub_result"]},
                        {"var": {"result": "@var.result"}}
                    ]
                }}
            ]
        },
        "factorial": {
            "params": {
                "n": "number"
            },
            "body": [
                {"echo": ["计算阶乘: ", "@params.n", "!"]},
                {"factorial_helper": ["@params.n"]},
                {"echo": ["计算完成: ", "@params.n", "! = ", "@var.result"]}
            ]
        },
        "main": {
            "body": [
                {"echo": ["阶乘计算示例（递归实现）\n\n"]},
                {"var": {"number": 5}},
                {"echo": ["计算 ", "@var.number", "! 的结果：\n"]},
                {"factorial": ["@var.number"]},
                {"echo": ["\n结果: ", "@var.number", "! = ", "@var.result", "\n\n"]},
                
                {"echo": ["测试其他输入值：\n"]},
                {"for": {
                    "var": "i",
                    "range": [0, 5],
                    "step": 1,
                    "body": [
                        {"factorial": ["@var.i"]},
                        {"echo": ["\n结果: ", "@var.i", "! = ", "@var.result", "\n"]}
                    ]
                }}
            ]
        }
    }
} 