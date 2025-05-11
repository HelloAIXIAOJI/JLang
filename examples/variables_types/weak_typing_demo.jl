{
    "include": ["math"],
    "program": {
        "demonstrate_weak_typing": {
            "params": {
                "value": "any"
            },
            "body": [
                {"echo": ["传入值的类型灵活处理：\n"]},
                {"echo": ["原始值: ", "@params.value", "\n"]},
                
                {"comment": "尝试加法转换"},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@params.value",
                        "right": true
                    },
                    "then": [
                        {"var": {"result": "无法将布尔值用于数学运算"}},
                        {"echo": ["加法尝试: ", "@var.result", "\n"]}
                    ],
                    "else": [
                        {"math.add": ["@params.value", 10]},
                        {"echo": ["加法结果: ", "@var.result", "\n"]}
                    ]
                }},
                
                {"comment": "尝试乘法转换"},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@params.value",
                        "right": true
                    },
                    "then": [
                        {"var": {"result": "无法将布尔值用于数学运算"}},
                        {"echo": ["乘法尝试: ", "@var.result", "\n\n"]}
                    ],
                    "else": [
                        {"math.multiply": ["@params.value", 2]},
                        {"echo": ["乘法结果: ", "@var.result", "\n\n"]}
                    ]
                }}
            ]
        },
        "main": {
            "body": [
                {"echo": ["JiLang 弱类型系统演示\n===========================\n\n"]},
                
                {"comment": "1. 变量类型的灵活性"},
                {"echo": ["1. 变量类型的灵活性\n"]},
                {"var": {"flexible": "42"}},
                {"echo": ["初始值 (字符串): '", "@var.flexible", "' (类型: 字符串)\n"]},
                {"math.add": ["@var.flexible", 8]},
                {"var": {"flexible": "@var.result"}},
                {"echo": ["加法后 (数字): ", "@var.flexible", " (类型: 数字)\n"]},
                {"var": {"flexible": true}},
                {"echo": ["重新赋值 (布尔值): ", "@var.flexible", " (类型: 布尔)\n\n"]},
                
                {"comment": "2. 隐式类型转换"},
                {"echo": ["2. 隐式类型转换\n"]},
                {"var": {"num_str": "123", "actual_num": 456}},
                {"math.add": ["@var.num_str", "@var.actual_num"]},
                {"echo": ["字符串 + 数字: '", "@var.num_str", "' + ", "@var.actual_num", " = ", "@var.result", "\n"]},
                
                {"var": {"num_word": "42abc"}},
                {"if": {
                    "condition": {
                        "op": "gt",
                        "left": "@var.num_word",
                        "right": 30
                    },
                    "then": [
                        {"echo": ["'42abc' > 30 (开头数字部分会尝试转换)\n"]}
                    ],
                    "else": [
                        {"echo": ["'42abc' 无法比较\n"]}
                    ]
                }},
                
                {"var": {"empty_array": [], "empty_obj": {}}},
                {"echo": ["空数组尝试数值运算:\n"]},
                {"math.add": ["@var.empty_array", 5]},
                {"echo": ["空数组 + 5 = ", "@var.result", "\n\n"]},
                
                {"comment": "3. 条件语句中的类型转换"},
                {"echo": ["3. 条件语句中的类型转换\n"]},
                {"var": {"str_num": "5", "num": 5}},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.str_num",
                        "right": "@var.num"
                    },
                    "then": [
                        {"echo": ["'5' 等于 5 (不同类型被视为相等)\n"]}
                    ],
                    "else": [
                        {"echo": ["'5' 不等于 5\n"]}
                    ]
                }},
                
                {"var": {"zero_str": "0", "empty_str": ""}},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.zero_str",
                        "right": 0
                    },
                    "then": [
                        {"echo": ["'0' 等于 0\n"]}
                    ]
                }},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.empty_str",
                        "right": 0
                    },
                    "then": [
                        {"echo": ["'' 等于 0\n"]}
                    ],
                    "else": [
                        {"echo": ["'' 不等于 0\n"]}
                    ]
                }},
                
                {"var": {"bool_true": true, "bool_false": false}},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.bool_true",
                        "right": 1
                    },
                    "then": [
                        {"echo": ["true 等于 1\n"]}
                    ],
                    "else": [
                        {"echo": ["true 不等于 1\n"]}
                    ]
                }},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.bool_false",
                        "right": 0
                    },
                    "then": [
                        {"echo": ["false 等于 0\n"]}
                    ],
                    "else": [
                        {"echo": ["false 不等于 0\n"]}
                    ]
                }},
                {"echo": ["\n"]},
                
                {"comment": "4. 函数调用中的类型灵活性"},
                {"echo": ["4. 函数调用中的类型灵活性\n"]},
                {"demonstrate_weak_typing": ["42"]},
                {"demonstrate_weak_typing": [42]},
                {"demonstrate_weak_typing": [true]},
                {"demonstrate_weak_typing": [null]},
                
                {"comment": "5. 字符串拼接与数值转换"},
                {"echo": ["5. 字符串拼接与数值转换\n"]},
                {"var": {"num1": 100, "num2": "50"}},
                {"concat": {
                    "target": "str_result",
                    "parts": ["数字转字符串: ", "@var.num1", " + ", "@var.num2", " = "]
                }},
                {"math.add": ["@var.num1", "@var.num2"]},
                {"concat": {
                    "target": "str_result",
                    "parts": ["@var.str_result", "@var.result"]
                }},
                {"echo": ["@var.str_result", "\n"]},
                
                {"var": {"bool_val": true, "int_val": 1}},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.bool_val",
                        "right": "@var.int_val"
                    },
                    "then": [
                        {"concat": {
                            "target": "bool_result",
                            "parts": ["布尔值比较: ", "@var.bool_val", " == ", "@var.int_val", " 为真"]
                        }}
                    ],
                    "else": [
                        {"concat": {
                            "target": "bool_result",
                            "parts": ["布尔值比较: ", "@var.bool_val", " == ", "@var.int_val", " 为假"]
                        }}
                    ]
                }},
                {"echo": ["@var.bool_result", "\n\n"]},
                
                {"comment": "6. 算术运算的特殊情况"},
                {"echo": ["6. 算术运算的特殊情况\n"]},
                {"var": {"str_num": "123", "non_num": "abc"}},
                
                {"comment": "安全处理无法转换的情况"},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.non_num",
                        "right": "abc"
                    },
                    "then": [
                        {"var": {"result": "无法将非数字字符串转换为数值"}},
                        {"echo": ["数字字符串 * 非数字字符串: '", "@var.str_num", "' * '", "@var.non_num", "' = ", "@var.result", "\n"]}
                    ]
                }},
                
                {"var": {"zero_str": "0"}},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.zero_str",
                        "right": "0"
                    },
                    "then": [
                        {"var": {"result": "除以零会导致特殊值(如NaN或Infinity)"}},
                        {"echo": ["除以零 (作为字符串): 10 / '", "@var.zero_str", "' = ", "@var.result", "\n"]}
                    ]
                }},
                
                {"var": {"null_val": null}},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.null_val",
                        "right": null
                    },
                    "then": [
                        {"var": {"result": "null在运算中通常视为0或特殊值"}},
                        {"echo": ["null + 10 = ", "@var.result", "\n\n"]}
                    ]
                }},
                
                {"echo": ["弱类型演示完成\n"]}
            ]
        }
    }
} 