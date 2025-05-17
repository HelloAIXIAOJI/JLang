{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== IO模块增强功能测试 =====\n"]},
        
        {"comment": "1. 文件存在检查"},
        {"echo": ["检查文件是否存在测试:\n"]},
        {"io.file_exists": ["examples/system_interaction/io_module_test.jl"], "output": "file_check"},
        {"echo": ["当前测试文件是否存在: ", "@var.file_check", "\n"]},
        {"io.file_exists": ["不存在的文件.txt"], "output": "not_exist_check"},
        {"echo": ["不存在的文件检查结果: ", "@var.not_exist_check", "\n\n"]},
        
        {"comment": "2. 文件写入和追加测试"},
        {"echo": ["文件写入和追加测试:\n"]},
        {"var": {"test_file_path": "examples/system_interaction/io_test_output.txt"}},
        {"io.write_file": ["@var.test_file_path", "这是初始内容\n第二行内容\n"], "output": "write_result"},
        {"echo": ["文件写入结果: ", "@var.write_result", "\n"]},
        
        {"io.append_file": ["@var.test_file_path", "这是追加的内容\n第四行内容\n"], "output": "append_result"},
        {"echo": ["文件追加结果: ", "@var.append_result", "\n"]},
        
        {"io.read_file": ["@var.test_file_path"], "output": "file_content"},
        {"echo": ["文件内容:\n", "@var.file_content", "\n"]},
        
        {"comment": "3. 目录列表测试"},
        {"echo": ["目录列表测试:\n"]},
        {"io.list_dir": ["examples/system_interaction"], "output": "dir_content"},
        {"echo": ["目录内容: ", "@var.dir_content", "\n\n"]},
        
        {"comment": "4. JSON文件操作测试"},
        {"echo": ["JSON文件操作测试:\n"]},
        {"var": {"json_data": {
          "name": "测试用户",
          "age": 30,
          "interests": ["编程", "阅读", "旅行"],
          "address": {
            "city": "北京",
            "country": "中国"
          }
        }}},
        {"io.write_json": ["examples/system_interaction/test_data.json", "@var.json_data", true], "output": "json_write_result"},
        {"echo": ["JSON写入结果: ", "@var.json_write_result", "\n"]},
        
        {"io.read_json": ["examples/system_interaction/test_data.json"], "output": "json_read_result"},
        {"echo": ["读取的JSON数据: ", "@var.json_read_result", "\n"]},
        {"echo": ["用户名: ", "@var.json_read_result.name", "\n"]},
        {"echo": ["兴趣列表: ", "@var.json_read_result.interests", "\n"]},
        {"echo": ["城市: ", "@var.json_read_result.address.city", "\n\n"]},
        
        {"comment": "5. 输入函数测试 - 需要用户交互"},
        {"echo": ["输入函数测试 (需要用户交互):\n"]},
        {"echo": ["注意: 以下测试需要用户交互，如果是自动化测试环境请注释掉这部分\n"]},
        {"var": {"skip_interactive": false}},
        
        {"if": {
          "condition": {"op": "eq", "left": "@var.skip_interactive", "right": true},
          "then": [
            {"echo": ["跳过交互式测试部分\n"]}
          ],
          "else": [
            {"io.input_with_default": ["请输入您的名字", "匿名用户"], "output": "user_name"},
            {"echo": ["您好, ", "@var.user_name", "!\n"]},
            
            {"io.input_number": ["请输入一个1-100之间的数字: ", 1, 100], "output": "user_number"},
            {"echo": ["您输入的数字是: ", "@var.user_number", "\n"]},
            
            {"io.confirm": ["您喜欢这个IO模块的增强功能吗?"], "output": "user_feedback"},
            {"echo": ["用户反馈: ", "@var.user_feedback", "\n"]}
          ]
        }},
        
        {"comment": "6. 清理测试文件"},
        {"io.delete_file": ["@var.test_file_path"], "output": "delete_result"},
        {"echo": ["删除测试文件结果: ", "@var.delete_result", "\n"]},
        {"io.delete_file": ["examples/system_interaction/test_data.json"], "output": "delete_json_result"},
        {"echo": ["删除JSON测试文件结果: ", "@var.delete_json_result", "\n"]},
        
        {"echo": ["\n===== 测试完成 =====\n"]}
      ]
    }
  }
} 