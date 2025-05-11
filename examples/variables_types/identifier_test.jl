{
    "include": ["math"],
    "const": {
        "VERSION": "1.0",
        "APP_NAME": "标识符测试"
    },
    "program": {
        "main": {
            "body": [
                {"echo": ["=== 标识符系统测试 ===\n"]},
                
                {"comment": "测试基本变量引用"},
                {"var": {"name": "测试用户"}},
                {"echo": ["用户名: ", "@var.name", "\n"]},
                
                {"comment": "测试常量引用"},
                {"echo": ["应用名: ", "@const.APP_NAME", "\n"]},
                {"echo": ["版本号: ", "@const.VERSION", "\n"]},
                
                {"comment": "测试函数参数引用"},
                {"print_data": ["测试内容"]},
                
                {"comment": "测试嵌套对象属性访问"},
                {"var": {"user": {
                    "profile": {
                        "name": "张三",
                        "age": 25,
                        "skills": ["编程", "设计"]
                    },
                    "isActive": true
                }}},
                {"echo": ["用户姓名: ", "@var.user.profile.name", "\n"]},
                {"echo": ["用户年龄: ", "@var.user.profile.age", "\n"]},
                
                {"comment": "测试数组索引访问"},
                {"var": {"numbers": [10, 20, 30, 40, 50]}},
                {"echo": ["第三个数字: ", "@var.numbers[2]", "\n"]},
                
                {"comment": "测试嵌套数组访问"},
                {"echo": ["第一个技能: ", "@var.user.profile.skills[0]", "\n"]},
                
                {"comment": "测试结果变量"},
                {"math.add": [5, 10]},
                {"echo": ["计算结果: ", "@var.result", "\n"]},
                
                {"echo": ["\n=== 测试完成 ===\n"]}
            ]
        },
        "print_data": {
            "params": {
                "data": "string"
            },
            "body": [
                {"echo": ["函数参数: ", "@params.data", "\n"]}
            ]
        }
    }
} 