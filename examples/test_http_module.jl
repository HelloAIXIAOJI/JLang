{
    "include": ["http", "io"],
    "program": {
        "main": {
            "body": [
                {"echo": ["=== JiLang HTTP模块测试 ===\n"]},
                
                {"comment": "GET请求测试"},
                {"echo": ["执行GET请求到httpbin.org...\n"]},
                {"http.get": ["https://httpbin.org/get"], "output": "get_result"},
                {"echo": ["GET请求状态码: ", "@var.get_result.status", "\n"]},
                {"echo": ["GET响应数据: ", "@var.get_result.body", "\n\n"]},
                
                {"comment": "带参数的GET请求"},
                {"var": {"query_url": "https://httpbin.org/get?name=JiLang&version=0.1"}},
                {"http.get": ["@var.query_url"], "output": "get_params_result"},
                {"echo": ["带参数GET请求状态码: ", "@var.get_params_result.status", "\n"]},
                {"echo": ["带参数GET响应数据: ", "@var.get_params_result.body", "\n\n"]},
                
                {"comment": "POST请求测试"},
                {"var": {"post_data": {
                    "language": "JiLang",
                    "version": 0.4,
                    "features": ["http", "json", "lua"]
                }}},
                {"http.post": ["https://httpbin.org/post", "@var.post_data"], "output": "post_result"},
                {"echo": ["POST请求状态码: ", "@var.post_result.status", "\n"]},
                {"echo": ["POST响应数据: ", "@var.post_result.body", "\n\n"]},
                
                {"comment": "带自定义请求头的GET请求"},
                {"var": {"custom_headers": {
                    "User-Agent": "JiLang-TestClient/0.1",
                    "Accept": "application/json",
                    "X-Custom-Header": "JiLang-Test"
                }}},
                {"http.get": ["https://httpbin.org/headers", "@var.custom_headers"], "output": "headers_result"},
                {"echo": ["自定义请求头响应: ", "@var.headers_result.body", "\n\n"]},
                
                {"comment": "URL编码/解码测试"},
                {"var": {"raw_text": "JiLang 是一种基于JSON的程序设计语言！"}},
                {"http.url_encode": ["@var.raw_text"], "output": "encoded_text"},
                {"echo": ["URL编码: ", "@var.encoded_text", "\n"]},
                {"http.url_decode": ["@var.encoded_text"], "output": "decoded_text"},
                {"echo": ["URL解码: ", "@var.decoded_text", "\n\n"]},
                
                {"comment": "错误处理测试"},
                {"http.get": ["https://non-existent-domain-123456789.com"], "output": "error_result"},
                {"echo": ["错误请求结果: ", "@var.error_result", "\n\n"]},
                
                {"echo": ["\n=== HTTP模块测试完成 ===\n"]}
            ]
        }
    }
} 