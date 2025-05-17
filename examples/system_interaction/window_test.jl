{
  "include": ["window"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== Window模块测试 =====\n"]},
        
        {"comment": "检查当前环境是否支持窗口功能"},
        {"window.is_supported": []},
        {"var": {"supported": "@var.result"}},
        {"echo": ["当前环境支持窗口功能: ", "@var.supported", "\n"]},
        
        {"if": {
          "condition": {"op": "eq", "left": "@var.supported", "right": false},
          "then": [
            {"echo": ["当前环境不支持窗口功能，跳过窗口测试\n"]}
          ],
          "else": [
            {"comment": "创建窗口"},
            {"window.create": ["JiLang测试窗口", 400, 300]},
            {"echo": ["窗口创建结果: ", "@var.result", "\n"]},
            
            {"comment": "设置窗口属性"},
            {"window.set_title": ["JiLang - 改变后的标题"]},
            {"echo": ["设置标题结果: ", "@var.result", "\n"]},
            
            {"window.set_size": [500, 400]},
            {"echo": ["设置大小结果: ", "@var.result", "\n"]},
            
            {"window.set_position": [200, 200]},
            {"echo": ["设置位置结果: ", "@var.result", "\n"]},
            
            {"comment": "添加控件"},
            {"window.add_label": ["这是一个标签", 20, 20, 200, 30]},
            {"echo": ["添加标签结果: ", "@var.result", "\n"]},
            {"var": {"label_id": "@var.result"}},
            
            {"window.add_button": ["点击我", 20, 60, 100, 30]},
            {"echo": ["添加按钮结果: ", "@var.result", "\n"]},
            {"var": {"button_id": "@var.result"}},
            
            {"window.add_textbox": [20, 100, 200, 30, "默认文本"]},
            {"echo": ["添加文本框结果: ", "@var.result", "\n"]},
            {"var": {"textbox_id": "@var.result"}},
            
            {"window.add_checkbox": ["选项", 20, 140, false]},
            {"echo": ["添加复选框结果: ", "@var.result", "\n"]},
            {"var": {"checkbox_id": "@var.result"}},
            
            {"comment": "注册事件"},
            {"window.on_click": ["@var.button_id", "button_clicked"]},
            {"echo": ["注册点击事件结果: ", "@var.result", "\n"]},
            
            {"window.on_key": ["Enter", "key_pressed"]},
            {"echo": ["注册按键事件结果: ", "@var.result", "\n"]},
            
            {"window.on_close": ["window_closed"]},
            {"echo": ["注册窗口关闭事件结果: ", "@var.result", "\n"]},
            
            {"comment": "对话框测试"},
            {"window.message_box": ["提示", "这是一个消息框", "info"]},
            {"echo": ["消息框显示结果: ", "@var.result", "\n"]},
            
            {"window.input_dialog": ["输入", "请输入内容:", "默认值"]},
            {"echo": ["输入对话框结果: ", "@var.result", "\n"]},
            
            {"window.file_dialog": ["open", "."]},
            {"echo": ["文件对话框结果: ", "@var.result", "\n"]},
            
            {"comment": "关闭窗口"},
            {"window.close": []},
            {"echo": ["窗口关闭结果: ", "@var.result", "\n"]}
          ]
        }},
        
        {"echo": ["===== Window模块测试完成 =====\n"]}
      ]
    },
    
    "button_clicked": {
      "body": [
        {"echo": ["按钮被点击了\n"]}
      ]
    },
    
    "key_pressed": {
      "body": [
        {"echo": ["按键被按下了\n"]}
      ]
    },
    
    "window_closed": {
      "body": [
        {"echo": ["窗口被关闭了\n"]}
      ]
    }
  }
} 