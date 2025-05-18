{
  "include": ["fbnc"],
  "program": {
    "main": {
      "body": [
        {"echo": ["=== JiLang 外部Lua斐波那契模块测试 ===\n"]},
        
        {"comment": "测试斐波那契函数"},
        {"fbnc.fibonacci": [10], "output": "fib_result"},
        {"echo": ["斐波那契数列第10项: ", "@var.fib_result", "\n"]},
        
        {"comment": "测试递归斐波那契不同参数"},
        {"fbnc.fibonacci": [0], "output": "fib_0"},
        {"fbnc.fibonacci": [1], "output": "fib_1"},
        {"fbnc.fibonacci": [5], "output": "fib_5"},
        {"fbnc.fibonacci": [15], "output": "fib_15"},
        
        {"echo": ["斐波那契数列第0项: ", "@var.fib_0", "\n"]},
        {"echo": ["斐波那契数列第1项: ", "@var.fib_1", "\n"]},
        {"echo": ["斐波那契数列第5项: ", "@var.fib_5", "\n"]},
        {"echo": ["斐波那契数列第15项: ", "@var.fib_15", "\n"]},
        
        {"echo": ["\n=== 测试完成 ===\n"]}
      ]
    }
  }
} 