{
    "program": {
        "main": {
            "body": [
                {"comment": "数组遍历和索引访问示例"},
                {"echo": ["=== 数组遍历和索引访问示例 ===\n\n"]},
                
                {"comment": "创建测试数组"},
                {"var": {"fruits": ["苹果", "香蕉", "橙子", "草莓", "葡萄"]}},
                {"echo": ["水果数组: ", "@var.fruits", "\n\n"]},
                
                {"comment": "1. 直接访问数组索引"},
                {"echo": ["1. 直接访问数组索引:\n"]},
                {"echo": ["   - 第一个水果: ", "@var.fruits[0]", "\n"]},
                {"echo": ["   - 第三个水果: ", "@var.fruits[2]", "\n"]},
                {"echo": ["   - 最后一个水果: ", "@var.fruits[4]", "\n\n"]},
                
                {"comment": "2. 使用for循环遍历数组"},
                {"echo": ["2. 使用for...in遍历水果数组:\n"]},
                {"for": {
                    "var": "fruit",
                    "in": "@var.fruits",
                    "body": [
                        {"echo": ["   - 当前水果: ", "@var.fruit", "\n"]}
                    ]
                }},
                {"echo": ["\n"]},
                
                {"comment": "3. 带索引的遍历"},
                {"echo": ["3. 带索引的遍历:\n"]},
                {"array.length": ["@var.fruits"]},
                {"var": {"length": "@var.result"}},
                {"for": {
                    "var": "i",
                    "range": [0, "@var.length - 1"],
                    "body": [
                        {"echo": ["   - 索引 ", "@var.i", ": ", "@var.fruits[@var.i]", "\n"]}
                    ]
                }},
                {"echo": ["\n"]},
                
                {"comment": "4. 嵌套数组访问"},
                {"echo": ["4. 嵌套数组访问:\n"]},
                {"var": {"matrix": [[1, 2, 3], [4, 5, 6], [7, 8, 9]]}},
                {"echo": ["   矩阵: ", "@var.matrix", "\n"]},
                {"echo": ["   - 第一行第二列: ", "@var.matrix[0][1]", "\n"]},
                {"echo": ["   - 第三行第一列: ", "@var.matrix[2][0]", "\n"]},
                {"echo": ["   - 第二行: ", "@var.matrix[1]", "\n\n"]},
                
                {"comment": "5. 遍历嵌套数组"},
                {"echo": ["5. 遍历嵌套数组:\n"]},
                {"for": {
                    "var": "row",
                    "in": "@var.matrix",
                    "body": [
                        {"array.length": ["@var.row"]},
                        {"var": {"rowLength": "@var.result"}},
                        {"for": {
                            "var": "j",
                            "range": [0, "@var.rowLength - 1"],
                            "body": [
                                {"echo": ["   - 元素 (", "@var.j", "): ", "@var.row[@var.j]", "\n"]}
                            ]
                        }},
                        {"echo": ["   ---\n"]}
                    ]
                }},
                {"echo": ["\n"]},
                
                {"comment": "6. 数组操作与for...in组合"},
                {"echo": ["6. 数组操作与for...in组合:\n"]},
                {"var": {"numbers": [10, 20, 30, 40, 50]}},
                {"var": {"sum": 0}},
                {"for": {
                    "var": "num",
                    "in": "@var.numbers",
                    "body": [
                        {"var": {"sum": "@var.sum + @var.num"}}
                    ]
                }},
                {"echo": ["   - 数组: ", "@var.numbers", "\n"]},
                {"echo": ["   - 元素和: ", "@var.sum", "\n\n"]},
                
                {"comment": "7. 作为函数返回值的数组"},
                {"echo": ["7. 作为函数返回值的数组:\n"]},
                {"regex.match": ["(\\d+)-(\\d+)-(\\d+)", "2023-06-15"]},
                {"echo": ["   - 完整匹配组: ", "@var.result", "\n"]},
                {"echo": ["   - 年份: ", "@var.result[1]", "\n"]},
                {"echo": ["   - 月份: ", "@var.result[2]", "\n"]},
                {"echo": ["   - 日期: ", "@var.result[3]", "\n\n"]},
                
                {"comment": "总结"},
                {"echo": ["=== 总结 ===\n"]},
                {"echo": ["JsonLang现在支持直接的数组索引访问和for...in循环语法，使得处理数组数据更加简便和直观\n"]}
            ]
        }
    }
} 