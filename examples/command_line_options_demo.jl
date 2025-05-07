{
    "program": {
        "description": "命令行选项演示",
        "version": "1.0.0",
        "main": {
            "body": [
                {"echo": ["这是JLang命令行选项演示程序\n"]},
                {"echo": ["----------------------------------------\n"]},
                {"echo": ["这个程序用于展示JLang的命令行选项功能\n"]},
                {"echo": ["您可以通过以下命令尝试不同的运行模式：\n"]},
                {"echo": ["\n"]},
                {"echo": ["1. 正常模式：\n"]},
                {"echo": ["   jlang examples/command_line_options_demo.jl\n"]},
                {"echo": ["\n"]},
                {"echo": ["2. 调试模式：\n"]},
                {"echo": ["   jlang --debug examples/command_line_options_demo.jl\n"]},
                {"echo": ["   // 这将显示更详细的执行信息和调试数据\n"]},
                {"echo": ["\n"]},
                {"echo": ["3. 容错模式：\n"]},
                {"echo": ["   jlang --ignore-non-critical-errors examples/command_line_options_demo.jl\n"]},
                {"echo": ["   // 程序会继续执行，即使遇到非关键性错误\n"]},
                {"echo": ["\n"]},
                {"echo": ["4. 检查模式：\n"]},
                {"echo": ["   jlang --check examples/command_line_options_demo.jl\n"]},
                {"echo": ["   // 只检查错误，不执行程序代码\n"]},
                {"echo": ["\n"]},
                {"echo": ["5. 查看帮助信息：\n"]},
                {"echo": ["   jlang --help\n"]},
                {"echo": ["\n"]},
                {"echo": ["6. 查看关于信息：\n"]},
                {"echo": ["   jlang --about\n"]},
                {"echo": ["\n"]},
                {"echo": ["7. 查看创建者信息：\n"]},
                {"echo": ["   jlang --creator\n"]},
                {"echo": ["\n"]},
                {"echo": ["----------------------------------------\n"]},
                {"echo": ["演示程序执行完毕！\n"]}
            ]
        }
    }
} 