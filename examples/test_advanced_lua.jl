{
    "include": ["advanced_lua", "math"],
    "program": {
        "main": {
            "body": [
                {"comment": "测试高级Lua模块交互"},
                {"echo": ["=== 高级Lua模块交互测试 ===\n\n"]},
                
                {"comment": "测试变量交互"},
                {"echo": ["* 测试Lua与JiLang变量交互\n"]},
                {"advanced_lua.test_variables": [], "output": "var_test_result"},
                {"echo": ["变量交互测试结果: ", "@var.var_test_result", "\n\n"]},
                
                {"comment": "测试JiLang函数调用"},
                {"echo": ["* 测试Lua调用JiLang函数\n"]},
                {"advanced_lua.test_jilang_call": [], "output": "call_test_result"},
                {"echo": ["函数调用测试结果:\n"]},
                {"echo": ["  - 加法结果: ", "@var.call_test_result.add_result", "\n"]},
                {"echo": ["  - 数组长度: ", "@var.call_test_result.array_length", "\n\n"]},
                
                {"comment": "测试数据处理"},
                {"echo": ["* 测试Lua数据处理\n"]},
                {"var": {"input_data": "这是测试数据"}},
                {"advanced_lua.process_data": ["@var.input_data"], "output": "process_result"},
                {"echo": ["字符串处理结果:\n"]},
                {"echo": ["  - 原始值: ", "@var.process_result.original", "\n"]},
                {"echo": ["  - 长度: ", "@var.process_result.length", "\n\n"]},
                
                {"comment": "测试数组处理"},
                {"var": {"input_data": [1, 2, 3, "hello", "world"]}},
                {"advanced_lua.process_data": ["@var.input_data"], "output": "array_process_result"},
                {"echo": ["数组处理结果:\n"]},
                {"echo": ["  - 数组大小: ", "@var.array_process_result.size", "\n"]},
                {"echo": ["  - 处理后数组: ", "@var.array_process_result.processed", "\n\n"]},
                
                {"comment": "测试交互式演示"},
                {"echo": ["* 交互式任务管理演示\n"]},
                {"advanced_lua.interactive_demo": [], "output": "demo_result"},
                {"echo": ["交互式演示已完成。任务报告存储在变量 'task_report' 中\n"]},
                {"echo": ["报告摘要: 完成率 ", "@var.task_report.completion_rate", "%\n\n"]},
                
                {"comment": "测试完成"},
                {"echo": ["=== 测试完成 ===\n"]}
            ]
        }
    }
} 