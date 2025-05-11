{
    "program": {
        "main": {
            "body": [
                {"comment": "对象操作示例程序"},
                {"echo": ["=== 对象操作示例 ===\n\n"]},
                
                {"comment": "1. 创建对象"},
                {"echo": ["1. 创建对象:\n"]},
                
                {"comment": "创建空对象"},
                {"object.create": []},
                {"var": {"emptyObj": "@var.result"}},
                {"echo": ["   - 空对象: ", "@var.emptyObj", "\n"]},
                
                {"comment": "创建带初始属性的对象"},
                {"object.create": {
                    "name": "张三",
                    "age": 25,
                    "email": "zhangsan@example.com"
                }},
                {"var": {"user": "@var.result"}},
                {"echo": ["   - 用户对象: ", "@var.user", "\n\n"]},
                
                {"comment": "2. 获取对象属性"},
                {"echo": ["2. 获取对象属性:\n"]},
                {"object.get": ["@var.user", "name"]},
                {"echo": ["   - 用户名: ", "@var.result", "\n"]},
                
                {"object.get": ["@var.user", "age"]},
                {"echo": ["   - 年龄: ", "@var.result", "\n"]},
                
                {"comment": "获取不存在的属性返回null"},
                {"object.get": ["@var.user", "address"]},
                {"echo": ["   - 地址: ", "@var.result", "\n\n"]},
                
                {"comment": "3. 设置对象属性"},
                {"echo": ["3. 设置对象属性:\n"]},
                
                {"comment": "修改现有属性"},
                {"object.set": ["@var.user", "age", 26]},
                {"echo": ["   - 修改年龄: ", "@var.user", "\n"]},
                
                {"comment": "添加新属性"},
                {"object.set": ["@var.user", "address", "北京市海淀区"]},
                {"echo": ["   - 添加地址: ", "@var.user", "\n\n"]},
                
                {"comment": "4. 检查属性存在"},
                {"echo": ["4. 检查属性是否存在:\n"]},
                
                {"object.has": ["@var.user", "email"]},
                {"echo": ["   - email属性", "@var.result", "存在", "\n"]},
                
                {"object.has": ["@var.user", "phone"]},
                {"echo": ["   - phone属性", "@var.result", "不存在", "\n\n"]},
                
                {"comment": "5. 获取对象的键和值"},
                {"echo": ["5. 获取对象的键和值:\n"]},
                
                {"object.keys": ["@var.user"]},
                {"var": {"userKeys": "@var.result"}},
                {"echo": ["   - 所有键: ", "@var.userKeys", "\n"]},
                
                {"object.values": ["@var.user"]},
                {"var": {"userValues": "@var.result"}},
                {"echo": ["   - 所有值: ", "@var.userValues", "\n\n"]},
                
                {"comment": "6. 遍历对象"},
                {"echo": ["6. 使用键数组遍历对象:\n"]},
                
                {"for": {
                    "var": "key",
                    "in": "@var.userKeys",
                    "body": [
                        {"object.get": ["@var.user", "@var.key"]},
                        {"echo": ["   - ", "@var.key", ": ", "@var.result", "\n"]}
                    ]
                }},
                {"echo": ["\n"]},
                
                {"comment": "7. 删除对象属性"},
                {"echo": ["7. 删除对象属性:\n"]},
                
                {"object.delete": ["@var.user", "email"]},
                {"echo": ["   - 删除email属性: ", "@var.result", "\n"]},
                {"echo": ["   - 操作后对象: ", "@var.user", "\n"]},
                
                {"comment": "尝试删除不存在的属性"},
                {"object.delete": ["@var.user", "phone"]},
                {"echo": ["   - 删除不存在的phone属性: ", "@var.result", "\n\n"]},
                
                {"comment": "8. 对象作为复杂数据结构"},
                {"echo": ["8. 嵌套对象和数组:\n"]},
                
                {"comment": "创建包含数组的对象"},
                {"var": {"scores": [85, 92, 78, 96]}},
                {"object.create": {
                    "name": "李四",
                    "courses": ["语文", "数学", "英语", "物理"],
                    "scores": "@var.scores"
                }},
                {"var": {"student": "@var.result"}},
                {"echo": ["   - 学生对象: ", "@var.student", "\n"]},
                
                {"comment": "创建嵌套对象"},
                {"object.create": {
                    "name": "公司A",
                    "location": "上海",
                    "employees": ["@var.user", "@var.student"]
                }},
                {"var": {"company": "@var.result"}},
                {"echo": ["   - 公司对象: ", "@var.company", "\n\n"]},
                
                {"comment": "总结"},
                {"echo": ["=== 总结 ===\n"]},
                {"echo": ["对象操作功能使我们能够轻松处理JSON对象，便于构建复杂数据结构和逻辑\n"]}
            ]
        }
    }
} 