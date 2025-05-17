{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== IO模块JSON属性提取测试 =====\n"]},
        
        {"var": {"test_data": {
          "name": "测试用户",
          "age": 30,
          "interests": ["编程", "阅读", "旅行"],
          "profile": {
            "location": "北京",
            "job": "开发者"
          }
        }}},
        
        {"io.write_json": ["examples/system_interaction/test_extract.json", "@var.test_data", true]},
        {"io.read_json": ["examples/system_interaction/test_extract.json"], "output": "json_data"},
        
        {"echo": ["完整数据: ", "@var.json_data", "\n"]},
        
        {"comment": "方法1: 使用io.write_json和io.read_json中转提取数据"},
        {"io.write_json": ["examples/system_interaction/temp.json", "@var.json_data.name", false], "output": "write_result"},
        {"io.read_file": ["examples/system_interaction/temp.json"], "output": "name_raw"},
        {"var": {"name_value": "@var.name_raw"}},
        {"io.delete_file": ["examples/system_interaction/temp.json"]},
        
        {"io.write_json": ["examples/system_interaction/temp.json", "@var.json_data.age", false], "output": "write_result"},
        {"io.read_file": ["examples/system_interaction/temp.json"], "output": "age_raw"},
        {"var": {"age_value": "@var.age_raw"}},
        {"io.delete_file": ["examples/system_interaction/temp.json"]},
        
        {"io.write_json": ["examples/system_interaction/temp.json", "@var.json_data.interests", false], "output": "write_result"},
        {"io.read_file": ["examples/system_interaction/temp.json"], "output": "interests_raw"},
        {"var": {"interests_value": "@var.interests_raw"}},
        {"io.delete_file": ["examples/system_interaction/temp.json"]},
        
        {"echo": ["用户名: ", "@var.name_value", "\n"]},
        {"echo": ["年龄: ", "@var.age_value", "\n"]},
        {"echo": ["兴趣: ", "@var.interests_value", "\n"]},
        
        {"comment": "方法2: 依次提取对象，对嵌套层次一层层访问"},
        {"var": {"profile_raw": "@var.json_data"}},
        {"io.write_json": ["examples/system_interaction/profile.json", "@var.profile_raw.profile", false]},
        {"io.read_json": ["examples/system_interaction/profile.json"], "output": "profile"},
        {"io.delete_file": ["examples/system_interaction/profile.json"]},
        
        {"io.write_json": ["examples/system_interaction/job.json", "@var.profile.job", false]},
        {"io.read_file": ["examples/system_interaction/job.json"], "output": "job_raw"},
        {"var": {"job": "@var.job_raw"}},
        {"io.delete_file": ["examples/system_interaction/job.json"]},
        
        {"echo": ["职业: ", "@var.job", "\n"]},
        
        {"io.delete_file": ["examples/system_interaction/test_extract.json"]},
        {"echo": ["\n===== 测试完成 =====\n"]}
      ]
    }
  }
} 