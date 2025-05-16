{
    "include": ["utils"],
    "program": {
        "main": {
            "body": [
                {"echo": ["=== JiLang 模块元数据测试 ===\n"]},
                
                {"comment": "访问模块元数据"},
                {"echo": ["模块元数据: \n"]},
                {"echo": ["  名称: utils\n"]},
                {"echo": ["  版本: ", "@var.module_meta.utils.version", "\n"]},
                {"echo": ["  描述: ", "@var.module_meta.utils.description", "\n"]},
                {"echo": ["  作者: ", "@var.module_meta.utils.author", "\n\n"]},
                
                {"comment": "调用模块函数"},
                {"utils.greet": ["测试用户"]},
                
                {"echo": ["\n=== 测试完成 ===\n"]}
            ]
        }
    }
} 