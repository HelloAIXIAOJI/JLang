{
    "program": {
        "main": {
            "body": [
                {"echo": ["===== 数组和嵌套属性测试 =====\n"]},
                
                {"comment": "创建一个简单数组"},
                {"var": {"simple_array": [1, 2, 3, 4, 5]}},
                {"echo": ["简单数组: ", "@var.simple_array", "\n"]},
                {"echo": ["数组长度: ", "@var.simple_array.length", "\n"]},
                {"echo": ["第一个元素: ", "@var.simple_array[0]", "\n"]},
                {"echo": ["第三个元素: ", "@var.simple_array[2]", "\n"]},
                
                {"comment": "使用变量作为索引"},
                {"var": {"index": 1}},
                {"echo": ["使用变量索引: ", "@var.simple_array[@var.index]", "\n"]},
                
                {"comment": "创建一个对象"},
                {"var": {"person": {
                    "name": "张三",
                    "age": 30,
                    "address": {
                        "city": "北京",
                        "street": "朝阳区"
                    },
                    "skills": ["编程", "设计", "写作"]
                }}},
                
                {"echo": ["人员姓名: ", "@var.person.name", "\n"]},
                {"echo": ["人员年龄: ", "@var.person.age", "\n"]},
                {"echo": ["人员城市: ", "@var.person.address.city", "\n"]},
                {"echo": ["第一个技能: ", "@var.person.skills[0]", "\n"]},
                
                {"comment": "动态属性访问"},
                {"var": {"prop": "name"}},
                {"var": {"skill_index": 2}},
                {"echo": ["动态属性: @var.person.", "@var.prop", " = ", "@var.person.name", "\n"]},
                {"echo": ["动态索引: @var.person.skills[", "@var.skill_index", "] = ", "@var.person.skills[@var.skill_index]", "\n"]},
                
                {"echo": ["\n===== 测试完成 =====\n"]}
            ]
        }
    }
} 