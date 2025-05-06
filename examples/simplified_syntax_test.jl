{
    "include": ["io", "math"],
    "const": {
        "PI": 3.14159
    },
    "program": {
        "main": {
            "body": [
                {"echo": ["请输入一个数字："]},
                {"io.input": [""]},
                {"var": {"number": "@var.result"}},
                {"echo": ["您输入的数字是：", "@var.number", "\n"]},
                {"math.sqrt": ["@var.number"]},
                {"var": {"sqrt_result": "@var.result"}},
                {"echo": ["平方根是：", "@var.sqrt_result", "\n"]},
                {"math.pow": ["@var.number", 2]},
                {"var": {"square_result": "@var.result"}},
                {"echo": ["平方是：", "@var.square_result", "\n"]},
                {"var": {"filename": "test_output.txt"}},
                {"concat": {
                    "target": "file_content",
                    "parts": [
                        "计算结果：\n",
                        "平方根：", "@var.sqrt_result", "\n",
                        "平方：", "@var.square_result", "\n"
                    ]
                }},
                {"io.write_file": ["@var.filename", "@var.file_content"]},
                {"echo": ["结果已保存到文件：", "@var.filename", "\n"]},
                {"io.read_file": ["@var.filename"]},
                {"echo": ["文件内容：\n", "@var.result"]}
            ]
        }
    }
} 