{
    "include": ["http"],
    "program": {
        "main": {
            "body": [
                {"comment": "定义URL和请求体"},
                {"var": {"url": "https://httpbin.org/get"}},
                {"var": {"post_data": {"name": "JiLang", "version": 0.4}}},
                
                {"comment": "执行GET请求"},
                {"http.get": ["@var.url"], "output": "get_result"},
                
                {"comment": "打印结果"},
                {"echo": ["GET请求状态码: ", "@var.get_result.status", "\n"]},
                {"echo": ["GET响应头: ", "@var.get_result.headers", "\n"]},
                {"echo": ["GET响应体: ", "@var.get_result.body", "\n"]}
            ]
        }
    }
} 