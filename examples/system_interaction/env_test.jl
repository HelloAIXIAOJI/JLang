// 环境变量测试文件
// 此文件用于测试dotenv文件中的环境变量
{
    "program": {
        "main": {
            "body": [
                {"echo": ["===== JiLang 环境变量测试 =====\n\n"]},
                
                // 测试基本变量
                {"echo": ["1. 基本测试变量:\n"]},
                {"echo": ["测试变量: ", "@env.JLANG_TEST_VAR", "\n"]},
                {"echo": ["测试数字: ", "@env.JLANG_TEST_NUMBER", "\n"]},
                {"echo": ["空变量: '", "@env.JLANG_TEST_EMPTY", "'\n"]},
                
                // 测试多语言支持
                {"echo": ["\n2. 多语言支持测试:\n"]},
                {"echo": ["Unicode: ", "@env.JLANG_TEST_UNICODE", "\n"]},
                {"echo": ["Emoji: ", "@env.JLANG_TEST_EMOJI", "\n"]},
                
                // 测试特殊字符
                {"echo": ["\n3. 特殊字符测试:\n"]},
                {"var": {"special": "@env.JLANG_TEST_SPECIAL"}},
                {"echo": ["特殊字符: ", "@var.special", "\n"]},
                
                // 测试路径解析
                {"echo": ["\n4. 路径测试:\n"]},
                {"var": {"path": "@env.JLANG_TEST_PATH"}},
                {"regex.split": ["@var.path", ":"]},
                {"var": {"path_parts": "@var.result"}},
                {"echo": ["Unix路径元素数量: ", "@var.path_parts.length", "\n"]},
                
                {"var": {"win_path": "@env.JLANG_TEST_WINDOWS_PATH"}},
                {"regex.split": ["@var.win_path", ";"]},
                {"var": {"win_path_parts": "@var.result"}},
                {"echo": ["Windows路径元素数量: ", "@var.win_path_parts.length", "\n"]},
                
                // 测试JSON解析
                {"echo": ["\n5. JSON测试:\n"]},
                {"var": {"json_str": "@env.JLANG_TEST_JSON"}},
                {"echo": ["原始JSON: ", "@var.json_str", "\n"]},
                
                // 注意：JiLang目前不支持直接解析JSON字符串为对象
                // 这里我们只验证能正确获取原始字符串
                
                // 测试数组分割
                {"echo": ["\n6. 数组测试:\n"]},
                {"var": {"array_str": "@env.JLANG_TEST_ARRAY"}},
                {"regex.split": ["@var.array_str", ","]},
                {"var": {"array_items": "@var.result"}},
                {"echo": ["数组元素数量: ", "@var.array_items.length", "\n"]},
                {"echo": ["第一个元素: ", "@var.array_items[0]", "\n"]},
                {"echo": ["最后一个元素: ", "@var.array_items[4]", "\n"]},
                
                // 测试配置值转换
                {"echo": ["\n7. 配置值测试:\n"]},
                {"var": {"debug": "@env.JLANG_APP_DEBUG"}},
                {"var": {"port": "@env.JLANG_APP_PORT"}},
                {"var": {"host": "@env.JLANG_APP_HOST"}},
                {"var": {"env": "@env.JLANG_APP_ENV"}},
                
                {"if": {
                    "condition": {"op": "eq", "left": "@var.debug", "right": "true"},
                    "then": [
                        {"echo": ["调试模式已启用\n"]}
                    ],
                    "else": [
                        {"echo": ["调试模式已禁用\n"]}
                    ]
                }},
                
                {"echo": ["应用将运行在 ", "@var.host", ":", "@var.port", " (", "@var.env", "环境)\n"]},
                
                // 测试不存在的环境变量
                {"echo": ["\n8. 不存在的环境变量测试:\n"]},
                {"var": {"not_exist": "@env.JLANG_NOT_EXIST"}},
                {"if": {
                    "condition": {"op": "eq", "left": "@var.not_exist", "right": "null"},
                    "then": [
                        {"echo": ["不存在的环境变量正确返回null\n"]}
                    ],
                    "else": [
                        {"echo": ["错误: 不存在的环境变量返回了: ", "@var.not_exist", "\n"]}
                    ]
                }},
                
                {"echo": ["\n===== 测试完成 =====\n"]}
            ]
        }
    }
} 