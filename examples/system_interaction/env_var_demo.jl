// 环境变量引用示例
// 此文件演示JiLang中的环境变量引用功能
{
    "program": {
        "main": {
            "body": [
                {"echo": ["===== 环境变量引用示例 =====\n\n"]},
                
                // 读取常见的环境变量
                {"echo": ["1. 常见环境变量访问：\n"]},
                {"echo": ["操作系统: ", "@env.OS", "\n"]},
                {"echo": ["系统用户: ", "@env.USERNAME", " (或 ", "@env.USER", ")\n"]},
                {"echo": ["用户主目录: ", "@env.HOME", " (或 ", "@env.USERPROFILE", ")\n"]},
                {"echo": ["系统临时目录: ", "@env.TEMP", " (或 ", "@env.TMP", ")\n"]},
                
                // 将环境变量存储到变量中
                {"echo": ["\n2. 将环境变量存储到变量中：\n"]},
                {"var": {"user_home": "@env.HOME"}},
                {"var": {"system_path": "@env.PATH"}},
                {"var": {"lang": "@env.LANG"}},
                {"echo": ["用户主目录 (从变量): ", "@var.user_home", "\n"]},
                {"echo": ["语言设置 (从变量): ", "@var.lang", "\n"]},
                
                // 处理不存在的环境变量
                {"echo": ["\n3. 处理不存在的环境变量：\n"]},
                {"echo": ["不存在的环境变量: ", "@env.NONEXISTENT_VAR", "\n"]},
                {"var": {"not_exist": "@env.NONEXISTENT_VAR"}},
                {"if": {
                    "condition": {"op": "eq", "left": "@var.not_exist", "right": "null"},
                    "then": [
                        {"echo": ["变量 'not_exist' 值为空 (null)\n"]}
                    ],
                    "else": [
                        {"echo": ["变量 'not_exist' 值不为空: ", "@var.not_exist", "\n"]}
                    ]
                }},
                
                // 在条件语句中使用环境变量
                {"echo": ["\n4. 在条件语句中使用环境变量：\n"]},
                {"if": {
                    "condition": {"op": "eq", "left": "@env.USERNAME", "right": ""},
                    "then": [
                        {"var": {"current_user": "@env.USER"}}
                    ],
                    "else": [
                        {"var": {"current_user": "@env.USERNAME"}}
                    ]
                }},
                {"echo": ["检测到当前用户: ", "@var.current_user", "\n"]},
                
                // 在循环中使用环境变量
                {"echo": ["\n5. 环境变量与路径处理：\n"]},
                {"var": {"path_parts": []}},
                
                {"var": {"path_value": "@env.PATH"}},
                {"comment": ["原始PATH值:", "@var.path_value"]},
                
                {"regex.split": ["@var.path_value", ";|:"]},
                {"var": {"path_parts": "@var.result"}},
                
                {"echo": ["系统路径包含 ", "@var.path_parts.length", " 个部分\n"]},
                {"echo": ["前5个路径部分：\n"]},
                
                {"for": {
                    "var": "i",
                    "range": [0, 4],
                    "body": [
                        {"if": {
                            "condition": {"op": "lt", "left": "@var.i", "right": "@var.path_parts.length"},
                            "then": [
                                {"echo": ["  ", "@var.i", ": ", "@var.path_parts[@var.i]", "\n"]}
                            ]
                        }}
                    ]
                }},
                
                // 检测操作系统类型
                {"echo": ["\n6. 使用环境变量检测操作系统：\n"]},
                {"switch": {
                    "expr": "@env.OS",
                    "cases": [
                        {
                            "value": "Windows_NT",
                            "body": [
                                {"echo": ["检测到 Windows 操作系统\n"]}
                            ]
                        },
                        {
                            "default": true,
                            "body": [
                                {"var": {"uname": "@env.UNAME"}},
                                {"if": {
                                    "condition": {"op": "neq", "left": "@var.uname", "right": "null"},
                                    "then": [
                                        {"echo": ["检测到类UNIX系统: ", "@var.uname", "\n"]}
                                    ],
                                    "else": [
                                        {"echo": ["检测到其他操作系统\n"]}
                                    ]
                                }}
                            ]
                        }
                    ]
                }},
                
                {"echo": ["\n===== 示例结束 =====\n"]}
            ]
        }
    }
} 