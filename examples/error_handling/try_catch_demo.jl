{
    "program": {
        "main": {
            "body": [
                {"echo": ["=== JiLang 错误处理演示 ===\n\n"]},

                {"comment": "示例1: 基本的try-catch使用"},
                {"echo": ["示例1: 基本的try-catch使用\n"]},
                {"try": {
                    "try": [
                        {"echo": ["尝试执行可能出错的代码...\n"]},
                        {"var": {"result": {"op": "div", "left": 10, "right": 0}}},
                        {"echo": ["这行不会执行，因为上面会产生除零错误\n"]}
                    ],
                    "catch": [
                        {"echo": ["捕获到错误！程序继续执行\n"]}
                    ]
                }},
                {"echo": ["try-catch之后的代码正常执行\n\n"]},

                {"comment": "示例2: 使用error_var捕获错误信息"},
                {"echo": ["示例2: 捕获和显示错误信息\n"]},
                {"try": {
                    "try": [
                        {"echo": ["尝试访问不存在的变量...\n"]},
                        {"echo": ["@var.undefined_variable", "\n"]}
                    ],
                    "catch": [
                        {"echo": ["捕获到错误: ", "@var.error_message", "\n"]}
                    ],
                    "error_var": "error_message"
                }},
                {"echo": ["\n"]},

                {"comment": "示例3: 使用finally块"},
                {"echo": ["示例3: 使用finally块\n"]},
                {"var": {"resource": "已分配资源"}},
                {"try": {
                    "try": [
                        {"echo": ["使用资源: ", "@var.resource", "\n"]},
                        {"var": {"result": {"op": "div", "left": 10, "right": 0}}}
                    ],
                    "catch": [
                        {"echo": ["处理错误\n"]}
                    ],
                    "finally": [
                        {"echo": ["清理资源: ", "@var.resource", "\n"]},
                        {"var": {"resource": null}}
                    ]
                }},
                {"echo": ["资源状态: ", "@var.resource", "\n\n"]},

                {"comment": "示例4: 嵌套的try-catch"},
                {"echo": ["示例4: 嵌套的try-catch\n"]},
                {"try": {
                    "try": [
                        {"echo": ["外层try\n"]},
                        {"try": {
                            "try": [
                                {"echo": ["内层try\n"]},
                                {"echo": ["制造错误...\n"]},
                                {"var": {"x": {"op": "mul", "left": "not_a_number", "right": 5}}}
                            ],
                            "catch": [
                                {"echo": ["内层catch: 捕获到内部错误\n"]},
                                {"echo": ["重新抛出...\n"]},
                                {"var": {"y": {"op": "div", "left": 1, "right": 0}}}
                            ]
                        }}
                    ],
                    "catch": [
                        {"echo": ["外层catch: 捕获到所有错误\n"]}
                    ]
                }},
                
                {"echo": ["\n=== 错误处理演示完成 ===\n"]}
            ]
        }
    }
} 