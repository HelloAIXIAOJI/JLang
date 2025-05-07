{
    "program": {
        "main": {
            "body": [
                {"comment": "错误信息示例 - 展示不同种类的错误信息"},
                {"echo": ["=== 错误信息示例 ===\n\n"]},
                
                {"comment": "1. 程序结构错误 (测试运行)"},
                {"echo": ["1. 程序结构错误:\n"]},
                {"echo": ["   - 如果'program.main.body'不是数组\n"]},
                {"echo": ["   - 如果statement不是对象\n"]},
                {"echo": ["   - 如果语句为空\n\n"]},
                
                {"comment": "2. 变量错误"},
                {"echo": ["2. 变量错误:\n"]},
                {"var": {"testVar": 100}},
                {"echo": ["   - 正确的变量: ", "@var.testVar", "\n"]},
                {"echo": ["   - 错误的变量引用: ", "@var.nonExistentVar", "\n\n"]},
                
                {"comment": "3. 函数错误"},
                {"echo": ["3. 函数错误:\n"]},
                {"echo": ["   - 调用不存在的函数:\n"]},
                {"call": ["nonExistentFunction"]},
                {"echo": ["   这行不会执行\n\n"]},
                
                {"comment": "4. 模块错误"},
                {"echo": ["4. 模块错误:\n"]},
                {"echo": ["   - 调用不存在的模块:\n"]},
                {"nonExistentModule.someFunction": []},
                {"echo": ["   这行不会执行\n\n"]},
                
                {"comment": "5. 运行时错误"},
                {"echo": ["5. 运行时错误:\n"]},
                {"echo": ["   - 使用错误的语句语法:\n"]},
                {"unknown_statement": "这是错误的语句"},
                {"echo": ["   这行不会执行\n"]}
            ]
        }
    }
} 