{
    "include": ["math"],
    "program": {
        "main": {
            "body": [
                {"comment": "无限循环示例"},
                {"echo": ["开始无限循环...\n"]},
                {"var": {"counter": 1}},
                {"while": {
                    "condition": {
                        "op": "eq",
                        "left": true,
                        "right": true
                    },
                    "body": [
                        {"comment": ["当前循环次数: ", "@var.counter"]},
                        {"echo": ["循环执行第 ", "@var.counter", " 次\n"]},
                        {"math.add": ["@var.counter", 1]},
                        {"var": {"counter": "@var.result"}},
                        
                        {"comment": "注意：此循环永远不会结束，需要手动终止程序"},
                        {"echo": ["按 Ctrl+C 可以终止程序\n"]}
                    ]
                }}
            ]
        }
    }
}