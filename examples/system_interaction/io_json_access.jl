{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== IO模块JSON属性访问测试 =====\n"]},
        
        {"var": {"test_data": {
          "name": "测试用户",
          "age": 30,
          "interests": ["编程", "阅读", "旅行"],
          "profile": {
            "location": "北京",
            "job": "开发者"
          },
          "scores": [85, 92, 78, 96]
        }}},
        
        {"echo": ["原始JSON对象: ", "@var.test_data", "\n\n"]},
        
        {"echo": ["=== 使用json_get获取属性 ===\n"]},
        {"io.json_get": ["@var.test_data", "name"], "output": "name_value"},
        {"echo": ["用户名: ", "@var.name_value", "\n"]},
        
        {"io.json_get": ["@var.test_data", "age"], "output": "age_value"},
        {"echo": ["年龄: ", "@var.age_value", "\n"]},
        
        {"io.json_get": ["@var.test_data", "interests"], "output": "interests_value"},
        {"echo": ["兴趣爱好: ", "@var.interests_value", "\n"]},
        
        {"io.json_get": ["@var.test_data", "profile.location"], "output": "location_value"},
        {"echo": ["位置: ", "@var.location_value", "\n"]},
        
        {"io.json_get": ["@var.test_data", "profile.job"], "output": "job_value"},
        {"echo": ["职业: ", "@var.job_value", "\n"]},
        
        {"io.json_get": ["@var.test_data", "scores.2"], "output": "third_score"},
        {"echo": ["第三个分数: ", "@var.third_score", "\n\n"]},
        
        {"echo": ["=== 使用json_set设置属性 ===\n"]},
        {"io.json_set": ["@var.test_data", "name", "新用户名称"], "output": "updated_data1"},
        {"echo": ["修改名称后: ", "@var.updated_data1", "\n"]},
        
        {"io.json_set": ["@var.updated_data1", "profile.location", "上海"], "output": "updated_data2"},
        {"echo": ["修改位置后: ", "@var.updated_data2", "\n"]},
        
        {"io.json_set": ["@var.updated_data2", "scores.1", 100], "output": "updated_data3"},
        {"echo": ["修改分数后: ", "@var.updated_data3", "\n"]},
        
        {"io.json_set": ["@var.updated_data3", "profile.skills", ["Rust", "JavaScript", "Python"]], "output": "updated_data4"},
        {"echo": ["添加技能后: ", "@var.updated_data4", "\n"]},
        
        {"echo": ["\n=== 错误处理测试 ===\n"]},
        {"io.json_get": ["@var.test_data", "nonexistent"], "output": "missing_result"},
        {"echo": ["不存在的属性: ", "@var.missing_result", "\n"]},
        
        {"io.json_get": ["@var.test_data", "profile.nonexistent"], "output": "missing_nested_result"},
        {"echo": ["不存在的嵌套属性: ", "@var.missing_nested_result", "\n"]},
        
        {"echo": ["\n===== 测试完成 =====\n"]}
      ]
    }
  }
} 