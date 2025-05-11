{
    "include": ["math"],
    "program": {
        "main": {
            "body": [
                {"var": {"users": [
                    {"name": "张三", "skills": ["Java", "Python"]},
                    {"name": "李四", "skills": ["C++", "JavaScript"]}
                ]}},
                {"echo": ["第二个用户的第一个技能: ", "@var.users[1].skills[0]", "\n"]}
            ]
        }
    }
} 