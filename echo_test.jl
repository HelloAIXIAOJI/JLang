{
  "program": {
    "main": {
      "body": [
        {"var": {"test_value": 42}},
        {"echo": ["测试值: @var.test_value", "\n"]},
        
        {"var": {"name": "张三"}},
        {"echo": ["你好，", "@var.name", "！", "\n"]}
      ]
    }
  }
} 