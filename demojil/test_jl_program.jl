{
    "include": ["testmodule", "testmodule"],
    "program": {
        "main": {
            "body": [
                {"comment": "测试.jl程序调用不同扩展名的模块"},
                {"echo": ["这是JL格式的程序文件\n"]},
                {"echo": ["=== 测试调用JL模块 ===\n"]},
                {"var": {"result": {"testmodule.greet": {"name": "来自JL程序"}}}},
                {"echo": ["=== 测试调用JIL模块 ===\n"]},
                {"var": {"result": {"testmodule.greet": {"name": "来自JL程序"}}}}
            ]
        }
    }
} 