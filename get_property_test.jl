{
  "program": {
    "main": {
      "body": [
        {"echo": ["===== 测试: get_property动态属性访问 =====", "\n"]},
        
        {"comment": "创建测试对象和数组"},
        {"var": {"numbers": [10, 20, 30, 40, 50]}},
        {"var": {"index": 2}},
        {"var": {
          "user": {
            "name": "张三",
            "details": {
              "age": 30,
              "skills": ["编程", "设计", "写作"]
            }
          }
        }},
        {"var": {"property": "age"}},
        {"var": {"skill_index": 1}},
        {"var": {"key": "details"}},
        
        {"comment": "测试直接属性访问"},
        {"get_property": {
          "object": "@var.user",
          "path": "name",
          "output": "user_name"
        }},
        {"echo": ["用户名称: ", "@var.user_name", "\n"]},
        
        {"comment": "测试嵌套属性访问 - 字符串路径"},
        {"get_property": {
          "object": "@var.user",
          "path": "details.age",
          "output": "user_age"
        }},
        {"echo": ["用户年龄: ", "@var.user_age", "\n"]},
        
        {"comment": "测试嵌套属性访问 - 数组路径"},
        {"get_property": {
          "object": "@var.user",
          "path": ["details", "skills"],
          "output": "user_skills"
        }},
        {"echo": ["用户技能: ", "@var.user_skills", "\n"]},
        
        {"comment": "测试数组访问"},
        {"get_property": {
          "object": "@var.numbers",
          "path": ["@var.index"],
          "output": "number_at_index"
        }},
        {"echo": ["索引", "@var.index", "处的数字: ", "@var.number_at_index", "\n"]},
        
        {"comment": "测试使用变量访问对象属性"},
        {"get_property": {
          "object": "@var.user",
          "path": ["@var.key", "@var.property"],
          "output": "dynamic_age"
        }},
        {"echo": ["动态访问年龄: ", "@var.dynamic_age", "\n"]},
        
        {"comment": "测试动态访问数组元素"},
        {"get_property": {
          "object": "@var.user",
          "path": ["details", "skills", "@var.skill_index"],
          "output": "dynamic_skill"
        }},
        {"echo": ["动态访问技能: ", "@var.dynamic_skill", "\n"]},
        
        {"comment": "测试数组长度"},
        {"get_property": {
          "object": "@var.numbers",
          "path": "length",
          "output": "numbers_length"
        }},
        {"echo": ["数组长度: ", "@var.numbers_length", "\n"]},
        
        {"comment": "测试多层次嵌套访问"},
        {"get_property": {
          "object": "@var.user",
          "path": ["details", "skills", "length"],
          "output": "skills_count"
        }},
        {"echo": ["技能数量: ", "@var.skills_count", "\n"]},
        
        {"echo": ["\n===== 测试完成 =====\n"]}
      ]
    }
  }
} 