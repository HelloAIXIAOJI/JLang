{
    "include": ["unknown_module", "io"],
    "const": "这不是一个对象",
    "program": {
        "main": {
            "body": [
                {"未知语句": "测试"},
                {"var": "不是对象"},
                {"var": {"name": "@const.UNDEFINED"}},
                {"echo": {"不是数组": "测试"}},
                {"concat": {
                    "target": 123,
                    "parts": "不是数组"
                }},
                {"if": {
                    "condition": "不是对象",
                    "then": [
                        {"echo": ["测试"]}
                    ]
                }},
                {"call": "不是数组"},
                {"call": []},
                {"call": [123]},
                {"call": ["undefined_func"]},
                {"call": ["math.sqrt", "不是数字"]},
                {"call": ["io.read_file"]},
                {"call": ["io.write_file", "/root/forbidden.txt", "测试"]},
                {"call": ["print_message"]}
            ]
        },
        "print_message": {
            "params": "不是对象",
            "body": "不是数组"
        }
    }
} 