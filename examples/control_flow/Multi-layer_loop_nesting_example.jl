{
    "include": ["math"],
    "program": {
        "process_level": {
            "params": {
                "level": "number",
                "max_depth": "number"
            },
            "body": [
                {"comment": ["处理层级 ", "@params.level", "/", "@params.max_depth"]},
                {"echo": ["层级 ", "@params.level", " 开始处理\n"]},
                
                {"if": {
                    "condition": {
                        "op": "lt",
                        "left": "@params.level",
                        "right": "@params.max_depth"
                    },
                    "then": [
                        {"comment": "还有更深的层级，递归处理"},
                        {"math.add": ["@params.level", 1]},
                        {"var": {"next_level": "@var.result"}},
                        {"process_level": ["@var.next_level", "@params.max_depth"]},
                        {"echo": ["层级 ", "@params.level", " 处理完成\n"]}
                    ],
                    "else": [
                        {"comment": "已到达最深层级，执行套娃操作"},
                        {"echo": ["达到最大深度 ", "@params.max_depth", "，开始执行多层嵌套操作\n"]},
                        
                        {"var": {"nested_data": {
                            "level1": {
                                "level2": {
                                    "level3": {
                                        "level4": {
                                            "value": "已达到数据嵌套第4层"
                                        }
                                    }
                                }
                            }
                        }}},
                        
                        {"echo": ["嵌套数据访问测试: ", "@var.nested_data.level1.level2.level3.level4.value", "\n"]},
                        
                        {"comment": "多层循环嵌套示例"},
                        {"for": {
                            "var": "i",
                            "range": [1, 3],
                            "body": [
                                {"echo": ["外层循环 i=", "@var.i", "\n"]},
                                {"for": {
                                    "var": "j",
                                    "range": [1, 2],
                                    "body": [
                                        {"echo": ["  中层循环 j=", "@var.j", "\n"]},
                                        {"for": {
                                            "var": "k",
                                            "range": [1, 2],
                                            "body": [
                                                {"echo": ["    内层循环 k=", "@var.k", "\n"]},
                                                {"if": {
                                                    "condition": {
                                                        "op": "eq",
                                                        "left": "@var.k",
                                                        "right": 1
                                                    },
                                                    "then": [
                                                        {"echo": ["      条件成立: k=1，嵌套在三层循环内的条件语句\n"]}
                                                    ],
                                                    "else": [
                                                        {"echo": ["      条件不成立: k≠1，嵌套在三层循环内的条件语句\n"]}
                                                    ]
                                                }}
                                            ]
                                        }}
                                    ]
                                }}
                            ]
                        }},
                        {"echo": ["层级 ", "@params.level", " 处理完成\n"]}
                    ]
                }}
            ]
        },
        "main": {
            "body": [
                {"echo": ["JiLang 多层嵌套（套娃）测试\n\n"]},
                {"var": {"max_depth": 5}},
                {"echo": ["将测试 ", "@var.max_depth", " 层的函数调用嵌套\n"]},
                {"process_level": [1, "@var.max_depth"]},
                {"echo": ["\n所有嵌套测试完成!\n"]}
            ]
        }
    }
}
