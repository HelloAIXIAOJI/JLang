{
    "include": ["http"],
    "program": {
        "main": {
            "body": [
                {"echo": ["=== HTTP GET请求测试 ===\n"]},
                
                {"comment": "GET请求测试"},
                {"var": {"get_url": "https://httpbin.org/get"}},
                {"http.get": ["@var.get_url"], "output": "get_result"},
                
                {"comment": "先提取状态码到单独变量"},
                {"var": {"status_code": "@var.get_result.status"}},
                {"echo": ["GET状态码: ", "@var.status_code", "\n"]},
                
                {"comment": "URL编码/解码测试"},
                {"var": {"text": "JiLang测试"}},
                {"http.url_encode": ["@var.text"], "output": "encoded"},
                {"echo": ["已编码: ", "@var.encoded", "\n"]},
                {"http.url_decode": ["@var.encoded"], "output": "decoded"},
                {"echo": ["已解码: ", "@var.decoded", "\n"]},
                
                {"echo": ["\n=== 测试完成 ===\n"]}
            ]
        }
    }
} 