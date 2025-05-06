{
    "program": {
        "main": {
            "body": [
                {"echo": ["JsonLang 系统命令执行示例\n\n"]},
                
                {"comment": "显示当前目录"},
                {"exec": {
                    "cmd": "dir",
                    "output": "dir_output"
                }},
                {"echo": ["当前目录内容：\n", "@var.dir_output.stdout", "\n"]},
                
                {"comment": "创建一个临时文件"},
                {"var": {"content": "这是通过JsonLang创建的文件\n来自系统命令执行功能\n"}},
                {"exec": {
                    "cmd": "echo",
                    "args": ["@var.content", ">", "temp_file.txt"],
                    "output": "create_result"
                }},
                
                {"comment": "读取创建的文件"},
                {"exec": {
                    "cmd": "type",
                    "args": ["temp_file.txt"],
                    "output": "file_content"
                }},
                {"echo": ["文件内容：\n", "@var.file_content.stdout", "\n"]},
                
                {"comment": "检查命令执行状态"},
                {"echo": ["命令执行状态码: ", "@var.file_content.status", "\n"]},
                
                {"comment": "演示跨平台兼容"},
                {"exec": {
                    "cmd": "echo",
                    "args": ["Hello from JsonLang!"],
                    "output": "echo_result"
                }},
                {"echo": ["Echo输出: ", "@var.echo_result.stdout", "\n"]},
                
                {"comment": "删除临时文件"},
                {"exec": {
                    "cmd": "del",
                    "args": ["temp_file.txt"],
                    "output": "cleanup_result"
                }},
                {"echo": ["清理完成，执行状态: ", "@var.cleanup_result.status", "\n"]}
            ]
        }
    }
} 