{
    "include": ["http"],
    "program": {
        "main": {
            "body": [
                {"echo": ["=== HTTP模块核心功能测试 ===\n"]},
                
                {"comment": "GET请求测试"},
                {"var": {"get_url": "https://httpbin.org/get"}},
                {"http.get": ["@var.get_url"], "output": "get_result"},
                {"var": {"get_status": "@var.get_result.status"}},
                {"echo": ["GET状态码: ", "@var.get_status", "\n"]},
                
                {"comment": "POST请求测试"},
                {"var": {"post_url": "https://httpbin.org/post"}},
                {"var": {"post_data": {"name": "JiLang", "version": "0.4.0"}}},
                {"http.post": ["@var.post_url", "@var.post_data"], "output": "post_result"},
                {"var": {"post_status": "@var.post_result.status"}},
                {"echo": ["POST状态码: ", "@var.post_status", "\n"]},
                
                {"comment": "PUT请求测试"},
                {"var": {"put_url": "https://httpbin.org/put"}},
                {"var": {"put_data": {"updated": true}}},
                {"http.put": ["@var.put_url", "@var.put_data"], "output": "put_result"},
                {"var": {"put_status": "@var.put_result.status"}},
                {"echo": ["PUT状态码: ", "@var.put_status", "\n"]},
                
                {"comment": "DELETE请求测试"},
                {"var": {"delete_url": "https://httpbin.org/delete"}},
                {"http.delete": ["@var.delete_url"], "output": "delete_result"},
                {"var": {"delete_status": "@var.delete_result.status"}},
                {"echo": ["DELETE状态码: ", "@var.delete_status", "\n"]},
                
                {"comment": "URL编码/解码测试"},
                {"var": {"text": "JiLang 测试文本 !@#$%^&*()"}},
                {"http.url_encode": ["@var.text"], "output": "encoded"},
                {"echo": ["已编码: ", "@var.encoded", "\n"]},
                {"http.url_decode": ["@var.encoded"], "output": "decoded"},
                {"echo": ["已解码: ", "@var.decoded", "\n"]},
                
                {"echo": ["\n=== 测试完成 ===\n"]}
            ]
        }
    }
} 