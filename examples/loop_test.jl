{
    "include": ["math"],
    "program": {
        "main": {
            "body": [
                {"echo": ["测试 while 循环：\n"]},
                {"var": {"counter": 0}},
                {"while": {
                    "condition": {
                        "op": "lt",
                        "left": "@var.counter",
                        "right": "3"
                    },
                    "body": [
                        {"echo": ["计数：", "@var.counter", "\n"]},
                        {"call": ["math.add", "@var.counter", 1]},
                        {"var": {"counter": "@var.result"}}
                    ]
                }},
                {"echo": ["\n测试 for 循环：\n"]},
                {"for": {
                    "var": "i",
                    "range": [1, 6],
                    "body": [
                        {"echo": ["第 ", "@var.i", " 次迭代\n"]}
                    ]
                }},
                {"echo": ["\n测试倒计时：\n"]},
                {"for": {
                    "var": "count",
                    "range": [5, 0],
                    "step": -1,
                    "body": [
                        {"echo": ["倒计时：", "@var.count", "\n"]}
                    ]
                }}
            ]
        }
    }
} 