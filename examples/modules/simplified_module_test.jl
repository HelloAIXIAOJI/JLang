{
    "include": ["utils", "math", "io"],
    "program": {
        "main": {
            "body": [
                {"echo": ["测试简化语法调用自定义模块和内置模块函数：\n\n"]},
                
                {"echo": ["1. 使用简化语法调用自定义模块函数：\n"]},
                {"utils.greet": ["JsonLang用户"]},
                
                {"echo": ["\n2. 使用简化语法调用内置模块数学函数：\n"]},
                {"math.add": [10, 5]},
                {"var": {"result1": "@var.result"}},
                {"echo": ["加法结果：", "@var.result1", "\n"]},
                
                {"echo": ["\n3. 使用简化语法调用自定义模块的计数函数：\n"]},
                {"utils.count": [1, 5]},
                
                {"echo": ["\n4. 使用简化语法写入文件：\n"]},
                {"var": {"output_file": "@var.filename", "content": "这是通过简化语法写入的内容"}},
                {"io.write_file": ["@var.output_file", "@var.content"]},
                {"echo": ["文件写入结果：", "@var.result", "\n"]},
                
                {"echo": ["\n5. 结合多个模块函数调用：\n"]},
                {"math.pow": [2, 8]},
                {"var": {"pow_result": "@var.result"}},
                {"utils.count": [1, "@var.pow_result"]}
            ]
        }
    }
} 