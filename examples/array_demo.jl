{
    "include": ["math"],
    "program": {
        "main": {
            "body": [
                {"comment": "JLang 数组操作示例"},
                
                {"comment": "创建数组 - 空数组"},
                {"array.create": []},
                {"var": {"empty_array": "@var.result"}},
                {"echo": ["空数组: ", "@var.empty_array", "\n\n"]},
                
                {"comment": "创建数组 - 带初始元素"},
                {"array.create": [1, 2, 3, "四", true, null]},
                {"var": {"numbers": "@var.result"}},
                {"echo": ["初始数组: ", "@var.numbers", "\n\n"]},
                
                {"comment": "创建指定大小的数组"},
                {"array.create": {"size": 5, "initial": 0}},
                {"var": {"zeros": "@var.result"}},
                {"echo": ["5个零的数组: ", "@var.zeros", "\n\n"]},
                
                {"comment": "获取数组长度"},
                {"array.length": ["@var.numbers"]},
                {"echo": ["数组长度: ", "@var.result", "\n\n"]},
                
                {"comment": "获取数组元素"},
                {"array.get": ["@var.numbers", 2]},
                {"echo": ["索引2的元素: ", "@var.result", "\n"]},
                
                {"comment": "设置数组元素"},
                {"array.set": ["@var.numbers", 2, "修改后的值"]},
                {"echo": ["修改后的数组: ", "@var.numbers", "\n\n"]},
                
                {"comment": "向数组添加元素"},
                {"array.push": ["@var.numbers", "新元素1", "新元素2"]},
                {"echo": ["添加元素后: ", "@var.numbers", "\n\n"]},
                
                {"comment": "从数组末尾移除元素"},
                {"array.pop": ["@var.numbers"]},
                {"var": {"popped": "@var.result"}},
                {"echo": ["移除的元素: ", "@var.popped", "\n"]},
                {"echo": ["移除后的数组: ", "@var.numbers", "\n\n"]},
                
                {"comment": "获取数组切片"},
                {"array.slice": ["@var.numbers", 1, 4]},
                {"var": {"slice": "@var.result"}},
                {"echo": ["切片(1-4): ", "@var.slice", "\n\n"]},
                
                {"comment": "动态创建并操作数组"},
                {"array.create": []},
                {"var": {"dynamic_array": "@var.result"}},
                
                {"comment": "使用循环填充数组"},
                {"for": {
                    "var": "i",
                    "range": [0, 5],
                    "body": [
                        {"math.multiply": ["@var.i", 10]},
                        {"array.push": ["@var.dynamic_array", "@var.result"]}
                    ]
                }},
                {"echo": ["动态生成的数组: ", "@var.dynamic_array", "\n\n"]},
                
                {"comment": "数组作为函数参数使用"},
                {"array.create": [5, 10, 15, 20, 25]},
                {"var": {"sum_array": "@var.result"}},
                {"sum_values": ["@var.sum_array"]},
                {"echo": ["数组所有元素的和: ", "@var.result", "\n"]}
            ]
        },
        
        "sum_values": {
            "params": {
                "arr": "array"
            },
            "body": [
                {"var": {"sum": 0}},
                {"array.length": ["@params.arr"]},
                {"var": {"len": "@var.result"}},
                
                {"for": {
                    "var": "i",
                    "range": [0, "@var.len"],
                    "body": [
                        {"var": {"idx": "@var.i"}},
                        {"array.get": ["@params.arr", "@var.idx"]},
                        {"var": {"current": "@var.result"}},
                        {"math.add": ["@var.sum", "@var.current"]},
                        {"var": {"sum": "@var.result"}}
                    ]
                }},
                
                {"var": {"result": "@var.sum"}}
            ]
        }
    }
} 