{
  "program": {
    "main": {
      "body": [
        {"echo": ["===== 测试return语句 =====", "\n"]},
        
        {"var": {"result1": {"test_return_value": 42}}},
        {"echo": ["直接返回值测试: @var.result1", "\n"]},
        
        {"var": {"x": 10, "y": 20}},
        {"var": {"result2": {"test_return_variable": "@var.x"}}},
        {"echo": ["返回变量测试: @var.result2", "\n"]},
        
        {"var": {"result3": {"test_return_expression": {"math.add": [5, 8]}}}},
        {"echo": ["返回表达式测试: @var.result3", "\n"]},
        
        {"var": {"result4": {"test_return_object": {"user": "admin", "id": 1001}}}},
        {"echo": ["返回对象测试: @var.result4", "\n"]},
        
        {"var": {"result5": {"test_return_array": [1, 2, 3]}}},
        {"echo": ["返回数组测试: @var.result5", "\n"]},
        
        {"var": {"result6": {"test_return_null": null}}},
        {"echo": ["返回null测试: @var.result6", "\n"]},
        
        {"var": {"result7": {"test_nested_return": {"nested": {"value": 999}}}}},
        {"echo": ["嵌套对象返回测试: @var.result7", "\n"]},
        
        {"var": {"result8": {"test_early_return": 100}}},
        {"echo": ["提前返回测试: @var.result8", "\n"]},
        
        {"echo": ["测试完成", "\n"]}
      ]
    },
    
    "test_return_value": {
      "params": {},
      "body": [
        {"return": 42}
      ]
    },
    
    "test_return_variable": {
      "params": {"var_ref": null},
      "body": [
        {"return": "@param.var_ref"}
      ]
    },
    
    "test_return_expression": {
      "params": {},
      "body": [
        {"return": {"math.add": [5, 8]}}
      ]
    },
    
    "test_return_object": {
      "params": {},
      "body": [
        {"return": {"user": "admin", "id": 1001}}
      ]
    },
    
    "test_return_array": {
      "params": {},
      "body": [
        {"return": [1, 2, 3]}
      ]
    },
    
    "test_return_null": {
      "params": {},
      "body": [
        {"return": null}
      ]
    },
    
    "test_nested_return": {
      "params": {},
      "body": [
        {"return": {"nested": {"value": 999}}}
      ]
    },
    
    "test_early_return": {
      "params": {},
      "body": [
        {"echo": ["这条消息不会被执行后面的代码"]},
        {"return": 100},
        {"echo": ["这条消息永远不会被显示"]}
      ]
    }
  },
  "include": ["math"]
} 