{
    "const": {
        "VERSION": "1.0.0",
        "AUTHOR": "Yang Ber",
        "GREETING": "你好"
    },
    "program": {
        "main": {
            "body": [
                {"var": {"message": "自豪的使用JLang开发!"}},
                {"var": {"name": "小kang"}},
                {"echo": ["当前版本：", "@const.VERSION", "\\n"]},
                {"echo": ["作者：", "@const.AUTHOR", "\\n"]},
                {"concat": {
                    "target": "fullMessage",
                    "parts": ["@const.GREETING", ", ", "@var.name", "! ", "@var.message"]
                }},
                {"echo": ["完整消息：", "@var.fullMessage", "\\n"]},
                {"if": {
                    "condition": {
                        "op": "eq",
                        "left": "@var.name",
                        "right": "小明"
                    },
                    "then": [
                        {"echo": ["欢迎回来，小明！\\n"]}
                    ],
                    "else": [
                        {"echo": ["你好，陌生人！\\n"]}
                    ]
                }},
                {"itiswow": ["@var.fullMessage"]}
            ]
        },
        "itiswow": {
            "body": [
                {"echo": ["收到的消息是：\\n"]},
                {"echo": ["@params.text", "\\n"]}
            ],
            "params": {
                "text": "string"
            }
        }
    }
} 