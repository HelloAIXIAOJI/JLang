{
    "program": {
        "main": {
            "body": [
                {"echo": ["===== 复杂数组和对象操作测试 =====\n"]},
                
                {"comment": "创建一个包含多种类型的数组"},
                {"var": {"mixed_array": [10, "hello", true, {"key": "value"}, [1, 2, 3]]}},
                {"echo": ["完整数组: ", "@var.mixed_array", "\n"]},
                {"echo": ["数组长度: ", "@var.mixed_array.length", "\n"]},
                {"echo": ["数字元素: ", "@var.mixed_array[0]", "\n"]},
                {"echo": ["字符串元素: ", "@var.mixed_array[1]", "\n"]},
                {"echo": ["布尔元素: ", "@var.mixed_array[2]", "\n"]},
                {"echo": ["对象元素: ", "@var.mixed_array[3]", "\n"]},
                {"echo": ["嵌套数组: ", "@var.mixed_array[4]", "\n"]},
                {"echo": ["对象属性: ", "@var.mixed_array[3].key", "\n"]},
                {"echo": ["嵌套数组元素: ", "@var.mixed_array[4][1]", "\n"]},
                
                {"comment": "创建一个嵌套对象"},
                {"var": {"company": {
                    "name": "示例公司",
                    "founded": 2010,
                    "departments": [
                        {
                            "name": "研发部",
                            "staff": 50,
                            "projects": ["项目A", "项目B"]
                        },
                        {
                            "name": "市场部",
                            "staff": 30,
                            "campaigns": {"online": 5, "offline": 3}
                        }
                    ],
                    "location": {
                        "country": "中国",
                        "city": "上海",
                        "address": {
                            "street": "开发路",
                            "number": 123
                        }
                    }
                }}},
                
                {"echo": ["\n公司名称: ", "@var.company.name", "\n"]},
                {"echo": ["成立年份: ", "@var.company.founded", "\n"]},
                {"echo": ["部门数量: ", "@var.company.departments.length", "\n"]},
                {"echo": ["第一个部门: ", "@var.company.departments[0].name", "\n"]},
                {"echo": ["第一个部门员工数: ", "@var.company.departments[0].staff", "\n"]},
                {"echo": ["第一个部门项目数: ", "@var.company.departments[0].projects.length", "\n"]},
                {"echo": ["第一个部门第二个项目: ", "@var.company.departments[0].projects[1]", "\n"]},
                {"echo": ["第二个部门: ", "@var.company.departments[1].name", "\n"]},
                {"echo": ["第二个部门线上活动数: ", "@var.company.departments[1].campaigns.online", "\n"]},
                {"echo": ["公司地址: ", "@var.company.location.address.street", " ", "@var.company.location.address.number", "\n"]},
                
                {"comment": "动态访问嵌套结构"},
                {"var": {"dept_index": 0}},
                {"var": {"project_index": 1}},
                {"var": {"address_field": "street"}},
                
                {"echo": ["\n使用变量访问:\n"]},
                {"echo": ["部门: ", "@var.company.departments[@var.dept_index].name", "\n"]},
                {"echo": ["项目: ", "@var.company.departments[@var.dept_index].projects[@var.project_index]", "\n"]},
                {"echo": ["地址字段: ", "@var.company.location.address.", "@var.address_field", " = ", "@var.company.location.address.street", "\n"]},
                
                {"echo": ["\n===== 测试完成 =====\n"]}
            ]
        }
    }
} 