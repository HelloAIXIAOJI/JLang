{
    "program": {
        "main": {
            "body": [
                {"comment": "正则表达式示例程序"},
                {"echo": ["=== 正则表达式示例 ===\n\n"]},
                
                {"comment": "1. 正则表达式匹配测试"},
                {"echo": ["1. 正则表达式匹配:\n"]},
                
                {"comment": "简单匹配数字"},
                {"regex.match": ["\\d+", "年龄是25岁"]},
                {"var": {"match1": "@var.result"}},
                {"echo": ["   - 数字匹配: ", "@var.match1", "\n"]},
                
                {"comment": "匹配带捕获组的模式"},
                {"regex.match": ["(\\d+)岁", "年龄是25岁"]},
                {"var": {"match2": "@var.result"}},
                {"echo": ["   - 带捕获组匹配: ", "@var.match2", "\n"]},
                {"var": {"match2_full": "@var.match2[0]"}},
                {"var": {"match2_group1": "@var.match2[1]"}},
                {"echo": ["   - 完整匹配: ", "@var.match2_full", "\n"]},
                {"echo": ["   - 捕获组1: ", "@var.match2_group1", "\n"]},
                
                {"comment": "匹配多个捕获组"},
                {"regex.match": ["用户(\\d+)_(\\w+)", "ID是用户12345_admin的账号"]},
                {"var": {"match3": "@var.result"}},
                {"echo": ["   - 多捕获组匹配: ", "@var.match3", "\n"]},
                {"var": {"match3_id": "@var.match3[1]"}},
                {"var": {"match3_type": "@var.match3[2]"}},
                {"echo": ["   - 用户ID: ", "@var.match3_id", "\n"]},
                {"echo": ["   - 用户类型: ", "@var.match3_type", "\n"]},
                
                {"comment": "无匹配的情况"},
                {"regex.match": ["\\d+岁", "年龄不详"]},
                {"echo": ["   - 无匹配结果: ", "@var.result", "\n\n"]},
                
                {"comment": "2. 正则表达式测试"},
                {"echo": ["2. 正则表达式测试 (regex.test):\n"]},
                
                {"comment": "测试邮箱格式"},
                {"var": {"email1": "user@example.com"}},
                {"regex.test": ["^[\\w.%+-]+@[\\w.-]+\\.[a-zA-Z]{2,}$", "@var.email1"]},
                {"echo": ["   - ", "@var.email1", " 是有效邮箱: ", "@var.result", "\n"]},
                
                {"var": {"email2": "invalid-email"}},
                {"regex.test": ["^[\\w.%+-]+@[\\w.-]+\\.[a-zA-Z]{2,}$", "@var.email2"]},
                {"echo": ["   - ", "@var.email2", " 是有效邮箱: ", "@var.result", "\n"]},
                
                {"comment": "测试手机号格式"},
                {"var": {"phone": "13800138000"}},
                {"regex.test": ["^1[3-9]\\d{9}$", "@var.phone"]},
                {"echo": ["   - ", "@var.phone", " 是有效手机号: ", "@var.result", "\n\n"]},
                
                {"comment": "3. 正则表达式替换"},
                {"echo": ["3. 正则表达式替换 (regex.replace):\n"]},
                
                {"comment": "基本替换"},
                {"var": {"text1": "我的电话是13912345678，请保密"}},
                {"regex.replace": ["1\\d{10}", "@var.text1", "***********"]},
                {"echo": ["   - 隐藏手机号: ", "@var.result", "\n"]},
                
                {"comment": "使用捕获组引用"},
                {"var": {"text2": "商品价格: 99.99元"}},
                {"regex.replace": ["(\\d+\\.\\d+)元", "@var.text2", "$1 CNY"]},
                {"echo": ["   - 货币单位替换: ", "@var.result", "\n"]},
                
                {"comment": "日期格式转换"},
                {"var": {"date": "2023-03-15"}},
                {"regex.replace": ["(\\d{4})-(\\d{2})-(\\d{2})", "@var.date", "$3/$2/$1"]},
                {"echo": ["   - 日期格式转换: ", "@var.result", "\n\n"]},
                
                {"comment": "4. 正则表达式分割"},
                {"echo": ["4. 正则表达式分割 (regex.split):\n"]},
                
                {"comment": "简单分割"},
                {"var": {"csvLine": "张三,李四,王五,赵六"}},
                {"regex.split": [",", "@var.csvLine"]},
                {"echo": ["   - 按逗号分割: ", "@var.result", "\n"]},
                
                {"comment": "高级分割 - 支持多种分隔符"},
                {"var": {"mixedSeparators": "苹果,香蕉;橙子 葡萄|草莓"}},
                {"regex.split": ["[,;\\s|]+", "@var.mixedSeparators"]},
                {"echo": ["   - 多分隔符分割: ", "@var.result", "\n"]},
                
                {"comment": "分割并处理每个部分"},
                {"var": {"dataLine": "id=1001,name=产品A,price=99.8"}},
                {"regex.split": [",", "@var.dataLine"]},
                {"var": {"parts": "@var.result"}},
                {"echo": ["   - 分割数据: ", "@var.parts", "\n"]},
                {"echo": ["   - 处理每个部分:\n"]},
                
                {"for": {
                    "var": "part",
                    "in": "@var.parts",
                    "body": [
                        {"regex.match": ["(.+)=(.+)", "@var.part"]},
                        {"if": {
                            "cond": "@var.result",
                            "then": [
                                {"echo": ["     · 键: ", "@var.result[1]", ", 值: ", "@var.result[2]", "\n"]}
                            ]
                        }}
                    ]
                }},
                {"echo": ["\n"]},
                
                {"comment": "5. 实际应用场景"},
                {"echo": ["5. 实际应用场景:\n"]},
                
                {"comment": "数据验证"},
                {"var": {"userInput": [
                    "user1@example.com", 
                    "13800138000", 
                    "invalid-email", 
                    "2023-01-01", 
                    "123456"
                ]}},
                {"echo": ["   - 数据验证结果:\n"]},
                
                {"comment": "邮箱验证"},
                {"var": {"emailPattern": "^[\\w.%+-]+@[\\w.-]+\\.[a-zA-Z]{2,}$"}},
                
                {"for": {
                    "var": "input",
                    "in": "@var.userInput",
                    "body": [
                        {"regex.test": ["@var.emailPattern", "@var.input"]},
                        {"if": {
                            "cond": "@var.result",
                            "then": [
                                {"echo": ["     · ", "@var.input", " 是有效邮箱\n"]}
                            ]
                        }}
                    ]
                }},
                
                {"comment": "从文本中提取信息"},
                {"var": {"log": "用户12345于2023-03-15 14:30:25登录系统，IP地址为192.168.1.1"}},
                {"echo": ["   - 从日志中提取信息:\n"]},
                
                {"regex.match": ["用户(\\d+)于([\\d-]+ [\\d:]+)登录系统，IP地址为([\\d\\.]+)", "@var.log"]},
                {"if": {
                    "cond": "@var.result",
                    "then": [
                        {"echo": ["     · 用户ID: ", "@var.result[1]", "\n"]},
                        {"echo": ["     · 登录时间: ", "@var.result[2]", "\n"]},
                        {"echo": ["     · IP地址: ", "@var.result[3]", "\n"]}
                    ]
                }},
                
                {"comment": "总结"},
                {"echo": ["\n=== 总结 ===\n"]},
                {"echo": ["正则表达式功能提供了强大的字符串处理能力，适用于数据校验、信息提取和文本处理等场景\n"]}
            ]
        }
    }
} 