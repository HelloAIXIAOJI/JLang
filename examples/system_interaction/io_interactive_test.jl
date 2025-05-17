{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== IO模块交互功能测试 =====\n"]},
        
        {"io.input_with_default": ["请输入您的名字", "匿名用户"], "output": "user_name"},
        {"echo": ["您好, ", "@var.user_name", "!\n"]},
        
        {"io.input_number": ["请输入一个1-10之间的数字: ", 1, 10], "output": "user_number"},
        {"echo": ["您输入的数字是: ", "@var.user_number", "\n"]},
        
        {"io.confirm": ["您喜欢这个测试吗?"], "output": "user_feedback"},
        {"echo": ["用户反馈: ", "@var.user_feedback", "\n"]},
        
        {"echo": ["\n===== 测试完成 =====\n"]}
      ]
    }
  }
} 