{
  "program": {
    "main": {
      "body": [
        {"echo": ["===== 测试: 变量嵌套引用 =====", "\n"]},
        
        {"var": {"numbers": [10, 20, 30, 40, 50]}},
        {"var": {"index": 2}},
        {"echo": ["使用直接索引: ", "@var.numbers[2]", "\n"]},
        {"echo": ["使用变量索引: ", "@var.numbers[@var.index]", "\n"]},
        
        {"var": {
          "user": {
            "name": "张三",
            "details": {
              "age": 30,
              "skills": ["编程", "设计", "写作"]
            }
          }
        }},
        {"var": {"skill_index": 1}},
        {"var": {"property": "age"}},
        {"echo": ["用户年龄(直接): ", "@var.user.details.age", "\n"]},
        {"echo": ["用户技能(直接): ", "@var.user.details.skills[1]", "\n"]},
        {"echo": ["用户技能(变量): ", "@var.user.details.skills[@var.skill_index]", "\n"]},
        {"echo": ["用户属性(变量): ", "@var.user.details.@var.property", "\n"]},
        
        {"var": {"key": "details"}},
        {"echo": ["使用变量属性名: ", "@var.user.@var.key.age", "\n"]},
        
        {"echo": ["===== 测试完成 =====", "\n"]}
      ]
    }
  }
} 