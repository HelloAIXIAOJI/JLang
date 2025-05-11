{
    "include": ["math", "nonexistent_module", "utils"],
    "program": {
        "main": {
            "body": [
                {"echo": ["测试 --check-all 选项\n"]},
                {"nonexistent_function": ["这不存在"]},
                {"call": ["nonexistent_module.test"]},
                {"invalid_module.func": []},
                {"invalid": "这不是正确的语句格式"}
            ]
        }
    }
} 