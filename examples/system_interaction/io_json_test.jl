{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== IO模块JSON功能测试 =====\n"]},
        
        {"var": {"test_data": {
          "name": "测试用户",
          "age": 30,
          "interests": ["编程", "阅读", "旅行"]
        }}},
        
        {"io.write_json": ["examples/system_interaction/test_simple.json", "@var.test_data", true]},
        {"io.read_json": ["examples/system_interaction/test_simple.json"], "output": "json_data"},
        
        {"echo": ["完整数据: ", "@var.json_data", "\n"]},
        {"var": {"username": "@var.json_data.name"}},
        {"echo": ["用户名: ", "@var.username", "\n"]},
        
        {"var": {"user_age": "@var.json_data.age"}},
        {"echo": ["年龄: ", "@var.user_age", "\n"]},
        
        {"var": {"interests": "@var.json_data.interests"}},
        {"echo": ["兴趣: ", "@var.interests", "\n"]},
        
        {"io.delete_file": ["examples/system_interaction/test_simple.json"]},
        {"echo": ["\n===== 测试完成 =====\n"]}
      ]
    }
  }
} 