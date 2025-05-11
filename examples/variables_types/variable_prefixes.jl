{
    "program": {
        "main": {
            "body": [
                {"echo": ["=== 测试不同的变量引用前缀 ===\n"]},
                
                {"comment": "定义一些变量"},
                {"var": {"name": "张三"}},
                {"var": {"age": 25}},
                {"var": {"city": "北京"}},
                
                {"comment": "使用传统的@var.前缀"},
                {"echo": ["传统引用方式: ", "@var.name", " 今年 ", "@var.age", " 岁，住在 ", "@var.city", "\n"]},
                
                {"comment": "使用$前缀"},
                {"echo": ["美元符号引用: ", "$name", " 今年 ", "$age", " 岁，住在 ", "$city", "\n"]},
                
                {"comment": "使用￥前缀"},
                {"echo": ["人民币符号引用: ", "￥name", " 今年 ", "￥age", " 岁，住在 ", "￥city", "\n"]},
                
                {"comment": "混合使用不同前缀"},
                {"echo": ["混合引用方式: ", "@var.name", " 今年 ", "$age", " 岁，住在 ", "￥city", "\n"]},
                
                {"echo": ["\n=== 测试完成 ===\n"]}
            ]
        }
    }
} 