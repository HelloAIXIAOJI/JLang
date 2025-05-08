{
    "program": {
        "main": {
            "body": [
                {"echo": ["JiLang 系统命令执行示例 (Unix版)\n\n"]},
                
                {"comment": "显示当前目录"},
                {"exec": {
                    "cmd": "ls",
                    "args": ["-la"],
                    "output": "dir_output"
                }},
                {"echo": ["当前目录内容：\n", "@var.dir_output.stdout", "\n"]},
                
                {"comment": "创建一个临时文件"},
                {"var": {"content": "这是通过JiLang创建的文件\n来自系统命令执行功能\n"}},
                {"exec": {
                    "cmd": "echo",
                    "args": ["@var.content", ">", "temp_file.txt"],
                    "output": "create_result"
                }},
                
                {"comment": "读取创建的文件"},
                {"exec": {
                    "cmd": "cat",
                    "args": ["temp_file.txt"],
                    "output": "file_content"
                }},
                {"echo": ["文件内容：\n", "@var.file_content.stdout", "\n"]},
                
                {"comment": "获取系统信息"},
                {"exec": {
                    "cmd": "uname",
                    "args": ["-a"],
                    "output": "system_info"
                }},
                {"echo": ["系统信息: ", "@var.system_info.stdout", "\n"]},
                
                {"comment": "显示进程信息"},
                {"exec": {
                    "cmd": "ps",
                    "args": ["aux", "|", "grep", "cargo"],
                    "output": "process_info"
                }},
                {"echo": ["进程信息:\n", "@var.process_info.stdout", "\n"]},
                
                {"comment": "清理临时文件"},
                {"exec": {
                    "cmd": "rm",
                    "args": ["temp_file.txt"],
                    "output": "cleanup_result"
                }},
                {"echo": ["清理完成，执行状态: ", "@var.cleanup_result.status", "\n"]}
            ]
        }
    }
} 