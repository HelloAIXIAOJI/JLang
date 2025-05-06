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
                        {"var": {"result": 1}},
                        {"echo": [" (基本情况)"]}
                    ],
                    "else": [
                        {"math.subtract": ["@params.n", 1]},
                        {"var": {"n_minus_1": "@var.result"}},
                        {"factorial": ["@var.n_minus_1"]},
                        {"var": {"sub_result": "@var.result"}},
                        {"echo": [" (子问题: ", "@var.n_minus_1", "! = ", "@var.sub_result", ")"]},
                        {"math.multiply": ["@params.n", "@var.sub_result"]},
                        {"echo": [" (计算: ", "@params.n", " × ", "@var.sub_result", " = ", "@var.result", ")"]}
                    ]
                }},
                {"echo": ["\n结果: ", "@params.n", "! = ", "@var.result", "\n"]}
            ]
        },
        "main": {
            "body": [
                {"echo": ["阶乘计算（简化递归）\n\n"]},
                {"factorial": [5]},
                {"echo": ["最终结果: 5! = ", "@var.result", "\n\n"]},
                {"echo": ["测试更多值:\n"]},
                {"for": {
                    "var": "i",
                    "range": [0, 5],
                    "body": [
                        {"factorial": ["@var.i"]},
                        {"echo": ["输出: ", "@var.i", "! = ", "@var.result", "\n\n"]}
                    ]
                }}
            ]
        }
    }
} 