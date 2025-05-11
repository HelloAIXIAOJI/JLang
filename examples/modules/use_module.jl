{
    "include": ["utils", "math"],
    "program": {
        "main": {
            "body": [
                {"echo": ["测试自定义模块功能：\n\n"]},
                {"utils.greet": ["世界"]},
                {"echo": ["\n"]},
                {"var": {"start1": 1, "end1": 5}},
                {"utils.count": ["@var.start1", "@var.end1"]},
                {"echo": ["\n使用内置模块和自定义模块的组合：\n"]},
                {"math.add": [10, 5]},
                {"var": {"result1": "@var.result"}},
                {"echo": ["数学计算结果：", "@var.result1", "\n"]},
                {"var": {"start2": 15, "end2": 20}},
                {"utils.count": ["@var.start2", "@var.end2"]}
            ]
        }
    }
} 