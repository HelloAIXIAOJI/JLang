{
    "include": ["math"],
    "program": {
        "main": {
            "body": [
                {"echo": ["测试直接嵌套函数调用:\n"]},
                {"var": {"test_result": {"math.add": [5, 3]}}},
                {"echo": ["结果是: ", "@var.test_result", "\n"]}
            ]
        }
    }
} 