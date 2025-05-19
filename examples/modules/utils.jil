{
    "module_meta": {
        "version": "1.0.0",
        "description": "JiLang实用工具模块",
        "author": "JiLang团队"
    },
    "program": {
        "greet": {
            "params": {
                "name": "string"
            },
            "body": [
                {"echo": ["你好，", "@params.name", "！这是来自utils模块的问候。\n"]}
            ]
        },
        "count": {
            "params": {
                "start": "number",
                "end": "number"
            },
            "body": [
                {"echo": ["开始计数：\n"]},
                {"var": {"step": 1}},
                {"if": {
                    "condition": {
                        "op": "gt",
                        "left": "@params.start",
                        "right": "@params.end"
                    },
                    "then": [
                        {"var": {"step": -1}}
                    ]
                }},
                {"for": {
                    "var": "i",
                    "range": ["@params.start", "@params.end"],
                    "step": "@var.step",
                    "body": [
                        {"echo": ["数字：", "@var.i", "\n"]}
                    ]
                }}
            ]
        }
    }
} 