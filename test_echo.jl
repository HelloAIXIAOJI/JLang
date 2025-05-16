{
  "program": {
    "main": {
      "body": [
        {"echo": ["===== 测试 echo 语句 =====\n"]},
        
        {"echo": ["1. 基本输出和自动结果存储:"]},
        {"echo": ["Hello, World!"]},
        {"echo": ["上次输出结果: ", "@var.result", "\n"]},
        
        {"echo": ["2. 指定output参数:"]},
        {"echo": {
          "0": "这是存储到自定义变量的消息",
          "output": "custom_message"
        }},
        {"echo": ["结果存储在result变量: ", "@var.result"]},
        {"echo": ["结果同时存储在自定义变量: ", "@var.custom_message", "\n"]},
        
        {"echo": ["3. 数据类型输出:"]},
        {"var": {"number": 42}},
        {"var": {"bool_value": true}},
        {"var": {"array": [1, 2, 3]}},
        {"var": {"obj": {"name": "测试", "value": 100}}},
        {"echo": ["数字: ", "@var.number"]},
        {"echo": ["布尔值: ", "@var.bool_value"]},
        {"echo": ["数组: ", "@var.array"]},
        {"echo": ["对象: ", "@var.obj", "\n"]},
        
        {"echo": ["4. 在表达式中使用echo结果:"]},
        {"echo": ["测试表达式"]},
        {"if": {
          "condition": {"op": "eq", "left": "@var.result", "right": "测试表达式"},
          "then": [
            {"echo": ["表达式匹配成功!"]}
          ],
          "else": [
            {"echo": ["表达式匹配失败!"]}
          ]
        }},
        
        {"echo": ["\n===== 测试完成 ====="]}
      ]
    }
  }
} 