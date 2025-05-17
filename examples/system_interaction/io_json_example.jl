{
  "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== JSON操作示例 =====\n"]},
        
        {"var": {
          "config": {
            "app": {
              "name": "JiLang应用",
              "version": "1.0.0"
            },
            "settings": {
              "theme": "dark",
              "language": "zh-CN"
            },
            "users": [
              {"id": 1, "name": "用户1"},
              {"id": 2, "name": "用户2"}
            ]
          }
        }},
        
        {"echo": ["原始配置:\n"]},
        {"io.json_get": ["@var.config", "app.name"]},
        {"echo": ["应用名称: ", "@var.result", "\n"]},
        
        {"io.json_get": ["@var.config", "settings.theme"]},
        {"echo": ["当前主题: ", "@var.result", "\n"]},
        
        {"io.json_get": ["@var.config", "users.1.name"]},
        {"echo": ["第二个用户: ", "@var.result", "\n\n"]},
        
        {"io.json_set": ["@var.config", "app.version", "1.1.0"]},
        {"var": {"updated_config": "@var.result"}},
        {"echo": ["更新版本后的应用版本: "]},
        {"io.json_get": ["@var.updated_config", "app.version"]},
        {"echo": ["@var.result", "\n"]},
        
        {"io.json_set": ["@var.config", "settings.notifications", true]},
        {"var": {"updated_config": "@var.result"}},
        {"echo": ["添加通知设置后: "]},
        {"io.json_get": ["@var.updated_config", "settings.notifications"]},
        {"echo": ["@var.result", "\n"]},
        
        {"var": {"new_user": {"id": 3, "name": "用户3"}}},
        {"io.json_get": ["@var.config", "users"]},
        {"var": {"users": "@var.result"}},
        {"array.push": ["@var.users", "@var.new_user"]},
        {"io.json_set": ["@var.config", "users", "@var.users"]},
        {"var": {"final_config": "@var.result"}},
        {"echo": ["添加新用户后总用户数: "]},
        {"io.json_get": ["@var.final_config", "users"]},
        {"var": {"final_users": "@var.result"}},
        {"array.length": ["@var.final_users"]},
        {"echo": ["@var.result", "\n"]},
        
        {"echo": ["最后一个用户名称: "]}, 
        {"io.json_get": ["@var.final_config", "users.2.name"]},
        {"echo": ["@var.result", "\n"]},
        
        {"echo": ["\n===== 示例完成 =====\n"]}
      ]
    }
  }
} 