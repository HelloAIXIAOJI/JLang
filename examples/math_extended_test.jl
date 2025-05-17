{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== 数学模块扩展功能测试 =====", "\n"]},
        
        // 绝对值函数测试
        {"echo": ["1. 绝对值函数 (abs):", "\n"]},
        {"var": {"test_value": -42}},
        {"math.abs": "@var.test_value"},
        {"echo": ["  -42的绝对值: ", "@var.result", "\n"]},
        
        // 三角函数测试
        {"echo": ["2. 三角函数 (sin, cos, tan):", "\n"]},
        {"math.sin": 30},
        {"echo": ["  sin(30°) = ", "@var.result", "\n"]},
        {"math.cos": 60},
        {"echo": ["  cos(60°) = ", "@var.result", "\n"]},
        {"math.tan": 45},
        {"echo": ["  tan(45°) = ", "@var.result", "\n"]},
        
        // 对数函数测试
        {"echo": ["3. 对数函数 (log, ln):", "\n"]},
        {"math.log": 100},
        {"echo": ["  log10(100) = ", "@var.result", "\n"]},
        {"math.ln": 2.718281828459045},
        {"echo": ["  ln(e) ≈ ", "@var.result", "\n"]},
        
        // 最大最小值测试
        {"echo": ["4. 最大最小值 (max, min):", "\n"]},
        {"var": {"numbers": [3, 8, 1, 6, 2]}},
        {"math.max": "@var.numbers"},
        {"echo": ["  数组[3, 8, 1, 6, 2]的最大值: ", "@var.result", "\n"]},
        {"math.min": "@var.numbers"},
        {"echo": ["  数组[3, 8, 1, 6, 2]的最小值: ", "@var.result", "\n"]},
        {"math.max": [10, 5, 20, 15]},
        {"echo": ["  数组[10, 5, 20, 15]的最大值: ", "@var.result", "\n"]},
        
        // 向上向下取整测试
        {"echo": ["5. 取整函数 (floor, ceil):", "\n"]},
        {"var": {"float_value": 7.8}},
        {"math.floor": "@var.float_value"},
        {"echo": ["  floor(7.8) = ", "@var.result", "\n"]},
        {"math.ceil": "@var.float_value"},
        {"echo": ["  ceil(7.8) = ", "@var.result", "\n"]},
        {"var": {"float_value": -3.2}},
        {"math.floor": "@var.float_value"},
        {"echo": ["  floor(-3.2) = ", "@var.result", "\n"]},
        {"math.ceil": "@var.float_value"},
        {"echo": ["  ceil(-3.2) = ", "@var.result", "\n"]},
        
        // 随机数测试
        {"echo": ["6. 随机数生成 (random):", "\n"]},
        {"math.random": []},
        {"echo": ["  生成0到1之间的随机数: ", "@var.result", "\n"]},
        {"math.random": [100]},
        {"echo": ["  生成0到100之间的随机数: ", "@var.result", "\n"]},
        {"math.random": [50, 100]},
        {"echo": ["  生成50到100之间的随机数: ", "@var.result", "\n"]},
        
        // 组合使用数学函数
        {"echo": ["\n7. 复杂数学计算示例:", "\n"]},
        {"var": {"radius": 5}},
        {"echo": ["  计算圆面积:", "\n"]},
        {"math.pow": [5, 2]},
        {"var": {"radius_squared": "@var.result"}},
        {"math.multiply": ["@var.radius_squared", 3.14159]},
        {"echo": ["  半径为5的圆面积: ", "@var.result", "\n"]},
        
        {"echo": ["  计算三角形斜边:", "\n"]},
        {"var": {"side_a": 3, "side_b": 4}},
        {"math.pow": ["@var.side_a", 2]},
        {"var": {"a_squared": "@var.result"}},
        {"math.pow": ["@var.side_b", 2]},
        {"var": {"b_squared": "@var.result"}},
        {"math.add": ["@var.a_squared", "@var.b_squared"]},
        {"math.sqrt": "@var.result"},
        {"echo": ["  直角三角形(3,4)的斜边: ", "@var.result", "\n"]},
        
        {"echo": ["===== 测试完成 =====\n"]}
      ]
    }
  }
} 