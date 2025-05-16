{
    "include": ["math_lua"],
    "program": {
        "main": {
            "body": [
                {"echo": ["=== JiLang Lua模块测试 ===\n"]},
                
                {"comment": "设置一个测试变量供Lua读取"},
                {"var": {"test_var": "JiLang测试变量"}},
                
                {"comment": "访问模块元数据"},
                {"echo": ["模块元数据: \n"]},
                {"echo": ["  版本: ", "@var.module_meta.math_lua.version", "\n"]},
                {"echo": ["  描述: ", "@var.module_meta.math_lua.description", "\n"]},
                {"echo": ["  作者: ", "@var.module_meta.math_lua.author", "\n\n"]},
                
                {"comment": "测试类型转换检查函数"},
                {"math_lua.check_types": [10], "output": "type_check"},
                {"echo": ["类型检查结果: ", "@var.type_check", "\n\n"]},
                
                {"comment": "测试斐波那契函数"},
                {"math_lua.fibonacci": [10], "output": "fib_result"},
                {"echo": ["斐波那契数列第10项: ", "@var.fib_result", "\n"]},
                
                {"comment": "测试阶乘函数"},
                {"math_lua.factorial": [5], "output": "fact_result"},
                {"echo": ["5的阶乘: ", "@var.fact_result", "\n"]},
                
                {"comment": "测试加法函数"},
                {"math_lua.add": [7, 8], "output": "add_result"},
                {"echo": ["7 + 8 = ", "@var.add_result", "\n"]},
                
                {"comment": "测试平方函数"},
                {"math_lua.square": [6], "output": "square_result"},
                {"echo": ["6的平方: ", "@var.square_result", "\n"]},
                
                {"comment": "测试JiLang和Lua交互"},
                {"math_lua.test_jilang_interaction": [], "output": "interaction_result"},
                {"echo": ["交互测试结果: ", "@var.interaction_result", "\n"]},
                {"echo": ["Lua设置的变量: ", "@var.lua_created_var", "\n"]},
                
                {"echo": ["\n=== 测试完成 ===\n"]}
            ]
        }
    }
} 