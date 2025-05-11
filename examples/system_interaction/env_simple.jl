{
    "program": {
        "main": {
            "body": [
                {"echo": ["环境变量测试:\n"]},
                {"echo": ["JLANG_TEST_VAR = ", "@env.JLANG_TEST_VAR", "\n"]},
                {"echo": ["JLANG_TEST_SPECIAL = ", "@env.JLANG_TEST_SPECIAL", "\n"]},
                {"echo": ["JLANG_TEST_PATH = ", "@env.JLANG_TEST_PATH", "\n"]},
                {"echo": ["JLANG_TEST_WINDOWS_PATH = ", "@env.JLANG_TEST_WINDOWS_PATH", "\n"]},
                {"echo": ["JLANG_TEST_JSON = ", "@env.JLANG_TEST_JSON", "\n"]},
                {"echo": ["JLANG_TEST_ARRAY = ", "@env.JLANG_TEST_ARRAY", "\n"]},
                {"echo": ["JLANG_APP_DEBUG = ", "@env.JLANG_APP_DEBUG", "\n"]},
                {"echo": ["JLANG_APP_PORT = ", "@env.JLANG_APP_PORT", "\n"]},
                {"echo": ["JLANG_APP_HOST = ", "@env.JLANG_APP_HOST", "\n"]},
                {"echo": ["JLANG_APP_ENV = ", "@env.JLANG_APP_ENV", "\n"]}
            ]
        }
    }
} 