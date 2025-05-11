{
    "program": {
        "main": {
            "body": [
                {"comment": "Switch语句示例"},
                {"var": {"choice": 2}},
                {"echo": ["选择的值是: ", "@var.choice", "\n\n"]},
                
                {"comment": "基本switch语句示例"},
                {"switch": {
                    "expr": "@var.choice",
                    "cases": [
                        {
                            "value": 1,
                            "body": [
                                {"echo": ["匹配到选项1\n"]}
                            ]
                        },
                        {
                            "value": 2,
                            "body": [
                                {"echo": ["匹配到选项2\n"]}
                            ]
                        },
                        {
                            "value": 3,
                            "body": [
                                {"echo": ["匹配到选项3\n"]}
                            ]
                        },
                        {
                            "default": true,
                            "body": [
                                {"echo": ["未找到匹配的选项\n"]}
                            ]
                        }
                    ]
                }},
                
                {"echo": ["\n带有fallthrough的示例:\n"]},
                {"var": {"day": "周二"}},
                {"switch": {
                    "expr": "@var.day",
                    "cases": [
                        {
                            "value": "周一",
                            "body": [
                                {"echo": ["今天是周一\n"]}
                            ]
                        },
                        {
                            "value": "周二",
                            "fallthrough": true,
                            "body": [
                                {"echo": ["今天是周二\n"]}
                            ]
                        },
                        {
                            "value": "周三",
                            "fallthrough": true,
                            "body": [
                                {"echo": ["明天是周三\n"]}
                            ]
                        },
                        {
                            "value": "周四",
                            "body": [
                                {"echo": ["后天是周四\n"]}
                            ]
                        },
                        {
                            "default": true,
                            "body": [
                                {"echo": ["是其他日子\n"]}
                            ]
                        }
                    ]
                }},
                
                {"comment": "测试不同类型的匹配"},
                {"echo": ["\n测试不同类型的匹配:\n"]},
                {"var": {"value": null}},
                {"switch": {
                    "expr": "@var.value",
                    "cases": [
                        {
                            "value": 0,
                            "body": [
                                {"echo": ["值为数字0\n"]}
                            ]
                        },
                        {
                            "value": null,
                            "body": [
                                {"echo": ["值为null\n"]}
                            ]
                        },
                        {
                            "value": false,
                            "body": [
                                {"echo": ["值为false\n"]}
                            ]
                        },
                        {
                            "default": true,
                            "body": [
                                {"echo": ["其他值\n"]}
                            ]
                        }
                    ]
                }}
            ]
        }
    }
} 