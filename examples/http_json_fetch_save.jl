{
    "include": ["http", "io"],
    "program": {
        "main": {
            "body": [
                {"echo": ["=== JiLang HTTP和IO模块集成示例 ===\n"]},
                {"echo": ["此示例从JSONPlaceholder API获取数据并保存到文件\n\n"]},
                
                {"comment": "定义API地址"},
                {"var": {"api_url": "https://jsonplaceholder.typicode.com"}},
                {"var": {"output_file": "api_data.json"}},
                
                {"comment": "创建请求头"},
                {"var": {"headers": {
                    "Accept": "application/json",
                    "User-Agent": "JiLang-HTTP-Demo/0.1"
                }}},
                
                {"comment": "获取用户数据"},
                {"echo": ["获取用户列表...\n"]},
                {"http.get": ["@var.api_url/users", "@var.headers"], "output": "users_response"},
                
                {"comment": "检查响应状态"},
                {"var": {"has_error": {"op": "has_key", "object": "@var.users_response", "key": "error"}}},
                {"if": {
                    "condition": "@var.has_error",
                    "then": [
                        {"var": {"error_msg": "@var.users_response.error"}},
                        {"echo": ["错误: ", "@var.error_msg", "\n"]},
                        {"return": {"value": false}}
                    ]
                }},
                
                {"var": {"users_count": "@var.users_response.body.length"}},
                {"echo": ["成功获取", "@var.users_count", "位用户\n"]},
                
                {"comment": "处理数据 - 创建一个精简的用户列表"},
                {"var": {"processed_data": []}},
                {"var": {"count": 0}},
                {"var": {"length": "@var.users_response.body.length"}},
                
                {"while": {
                    "condition": {
                        "op": "lt",
                        "left": "@var.count",
                        "right": "@var.length"
                    },
                    "body": [
                        {"var": {"user_id": "@var.users_response.body.@var.count.id"}},
                        {"var": {"user_name": "@var.users_response.body.@var.count.name"}},
                        {"var": {"user_email": "@var.users_response.body.@var.count.email"}},
                        {"var": {"user_company": "@var.users_response.body.@var.count.company.name"}},
                        
                        {"var": {"user": {
                            "id": "@var.user_id",
                            "name": "@var.user_name",
                            "email": "@var.user_email",
                            "company": "@var.user_company"
                        }}},
                        
                        {"@var.processed_data.push": "@var.user"},
                        {"var": {"count": {"op": "add", "left": "@var.count", "right": 1}}}
                    ]
                }},
                
                {"comment": "获取特定用户的帖子"},
                {"echo": ["获取用户ID 1的帖子...\n"]},
                {"http.get": ["@var.api_url/users/1/posts"], "output": "posts_response"},
                
                {"comment": "将处理后的数据和帖子数据合并到一个对象"},
                {"var": {"final_data": {
                    "users": "@var.processed_data",
                    "user1_posts": "@var.posts_response.body",
                    "metadata": {
                        "timestamp": {"@date.now": []},
                        "source": "JSONPlaceholder API",
                        "generated_by": "JiLang HTTP Module Demo"
                    }
                }}},
                
                {"comment": "将结果保存到文件"},
                {"echo": ["保存数据到", "@var.output_file", "...\n"]},
                {"io.write_json": ["@var.output_file", "@var.final_data", true], "output": "write_result"},
                
                {"comment": "验证文件是否已保存"},
                {"io.file_exists": ["@var.output_file"], "output": "file_exists"},
                
                {"if": {
                    "condition": "@var.file_exists",
                    "then": [
                        {"echo": ["文件保存成功!\n"]},
                        
                        {"comment": "读取文件验证内容"},
                        {"io.read_json": ["@var.output_file"], "output": "read_data"},
                        {"var": {"users_count": "@var.read_data.users.length"}},
                        {"var": {"posts_count": "@var.read_data.user1_posts.length"}},
                        {"echo": ["文件中包含", "@var.users_count", "个用户记录和", 
                                  "@var.posts_count", "个帖子\n"]}
                    ],
                    "else": [
                        {"echo": ["文件保存失败!\n"]}
                    ]
                }},
                
                {"echo": ["\n=== 示例完成 ===\n"]}
            ]
        }
    }
} 