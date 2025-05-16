{
  "program": {
    "main": {
      "body": [
        {"echo": ["===== 测试 concat 语句 =====\n"]},
        
        {"echo": ["1. 测试数组格式 - 基本使用:"]},
        {"var": {"greeting": {"concat": ["Hello, ", "World!"]}}},
        {"echo": ["结果: ", "@var.greeting", "\n"]},
        
        {"echo": ["2. 测试对象格式 - 基本使用:"]},
        {"concat": {
          "target": "traditional",
          "parts": ["你好, ", "世界!"]
        }},
        {"echo": ["结果: ", "@var.traditional", "\n"]},
        
        {"echo": ["3. 连接不同数据类型:"]},
        {"var": {"mixed": {"concat": [
          "字符串: ", "text", 
          ", 数字: ", 42, 
          ", 布尔值: ", true, 
          ", 数组: ", [1, 2, 3],
          ", null值: ", null
        ]}}},
        {"echo": ["结果: ", "@var.mixed", "\n"]},
        
        {"echo": ["4. 嵌套concat调用:"]},
        {"var": {"first": "第一部分"}},
        {"var": {"second": "第二部分"}},
        {"var": {"nested": {"concat": [
          "开始 -> ", 
          {"concat": ["[", "@var.first", " + ", "@var.second", "]"]}, 
          " <- 结束"
        ]}}},
        {"echo": ["结果: ", "@var.nested", "\n"]},
        
        {"echo": ["5. 在表达式中使用concat:"]},
        {"var": {"username": "admin"}},
        {"if": {
          "condition": {"op": "eq", "left": {"concat": ["用户:", "@var.username"]}, "right": "用户:admin"},
          "then": [
            {"echo": ["条件匹配成功!"]}
          ],
          "else": [
            {"echo": ["条件匹配失败!"]}
          ]
        }},
        
        {"echo": ["\n===== 测试完成 ====="]}
      ]
    }
  }
} 