{
    "include": ["math"],
    "program": {
        "fibonacci": {
            "params": {
                "n": "number"
            },
            "body": [
                {"echo": ["计算斐波那契数 F(", "@params.n", ")"]},
                {"if": {
                    "condition": {
                        "op": "lte",
                        "left": "@params.n",
                        "right": 1
                    },
                    "then": [
                        {"var": {"result": "@params.n"}},
                        {"echo": [" (基本情况)"]}
                    ],
                    "else": [
                        {"math.subtract": ["@params.n", 1]},
                        {"var": {"n_minus_1": "@var.result"}},
                        
                        {"fibonacci": ["@var.n_minus_1"]},
                        {"var": {"fib_n_minus_1": "@var.result"}},
                        
                        {"math.subtract": ["@params.n", 2]},
                        {"var": {"n_minus_2": "@var.result"}},
                        
                        {"fibonacci": ["@var.n_minus_2"]},
                        {"var": {"fib_n_minus_2": "@var.result"}},
                        
                        {"echo": [" (F(", "@var.n_minus_1", ")=", "@var.fib_n_minus_1", " + F(", "@var.n_minus_2", ")=", "@var.fib_n_minus_2", ")"]},
                        
                        {"math.add": ["@var.fib_n_minus_1", "@var.fib_n_minus_2"]},
                        {"var": {"result": "@var.result"}}
                    ]
                }},
                {"echo": ["\n结果: F(", "@params.n", ") = ", "@var.result"]}
            ]
        },
        "main": {
            "body": [
                {"echo": ["斐波那契数列计算（递归实现）\n\n"]},
                
                {"var": {"number": 8}},
                {"echo": ["计算斐波那契数 F(", "@var.number", ") 的结果：\n"]},
                
                {"fibonacci": ["@var.number"]},
                {"echo": ["\n\n最终结果: F(", "@var.number", ") = ", "@var.result", "\n\n"]},
                
                {"echo": ["测试斐波那契数列 F(0) 到 F(10)：\n"]},
                {"for": {
                    "var": "i",
                    "range": [0, 10],
                    "body": [
                        {"fibonacci": ["@var.i"]},
                        {"echo": ["\n"]}
                    ]
                }}
            ]
        }
    }
} 