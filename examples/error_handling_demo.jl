{
    "program": {
        "main": {
            "body": [
                {"comment": "错误处理示例程序"},
                {"echo": ["=== 错误处理演示 ===\n\n"]},
                
                {"comment": "1. 正常语句执行"},
                {"var": {"message": "这是一个正常的变量"}},
                {"echo": ["1. 正常语句: ", "@var.message", "\n\n"]},
                
                {"comment": "2. 可能触发的错误 - 在容错模式下继续执行"},
                {"echo": ["2. 尝试执行可能出错的操作:\n"]},
                
                {"comment": "创建一个数组用于演示"},
                {"var": {"myArray": [1, 2, 3]}},
                {"echo": ["   - 创建了数组: ", "@var.myArray", "\n"]},
                
                {"comment": "正确的数组操作"},
                {"array.get": ["@var.myArray", 1]},
                {"echo": ["   - 正确获取数组元素: ", "@var.result", "\n"]},
                
                {"comment": "错误的数组操作 - 使用不存在的变量 (在容错模式下会继续)"},
                {"array.get": ["@var.nonExistentArray", 0]},
                {"echo": ["   这行在非容错模式下不会执行\n"]},
                
                {"comment": "3. 继续执行后的语句"},
                {"echo": ["\n3. 在容错模式下继续执行:\n"]},
                {"var": {"continueMessage": "程序仍在继续!"}},
                {"echo": ["   - ", "@var.continueMessage", "\n\n"]},
                
                {"comment": "4. 对象嵌套访问错误"},
                {"echo": ["4. 尝试访问不存在的嵌套属性:\n"]},
                {"var": {"user": {"name": "测试用户", "age": 25}}},
                {"echo": ["   - 用户信息: ", "@var.user", "\n"]},
                {"echo": ["   - 尝试访问不存在的属性...\n"]},
                
                {"comment": "这会导致错误，但在容错模式下继续"},
                {"concat": {
                    "target": "errorDemo",
                    "parts": ["@var.user.address.city"]
                }},
                {"echo": ["   这行在非容错模式下不会执行\n"]},
                
                {"comment": "5. 函数调用错误"},
                {"echo": ["\n5. 尝试调用不存在的函数:\n"]},
                {"call": ["nonExistentFunction"]},
                {"echo": ["   这行在非容错模式下不会执行\n"]},
                
                {"comment": "6. 语法错误示例"},
                {"echo": ["\n6. 注意: 语法错误和程序结构错误无法在运行时处理\n"]},
                {"echo": ["   这些错误必须在运行前修复，例如:\n"]},
                {"echo": ["   - JSON解析错误\n"]},
                {"echo": ["   - 缺少必要的程序结构\n"]},
                {"echo": ["   - 不完整的语句定义\n\n"]},
                
                {"comment": "总结"},
                {"echo": ["=== 总结 ===\n"]},
                {"echo": ["使用 --ignore-non-critical-errors 运行时，程序将忽略非关键错误并继续执行\n"]},
                {"echo": ["使用 --check 运行时，程序将仅检查错误，不执行任何语句\n"]}
            ]
        }
    }
} 