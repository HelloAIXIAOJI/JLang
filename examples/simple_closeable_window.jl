{
    "include": ["windowwv"],
    "program": {
        "main": {
            "body": [
                {"comment": "创建一个可关闭测试窗口"},
                {"echo": ["启动窗口测试，请验证关闭按钮和JavaScript通信...\n"]},
                
                {"windowwv.create": ["可关闭WebView测试", 600, 400]},
                {"var": {"window": "@var.result"}},
                {"echo": ["创建的窗口ID: ", "@var.window.id", "\n"]},
                
                {"comment": "定义测试HTML，包含按钮和通信测试"},
                {"var": {"html_content": "<!DOCTYPE html><html><head><title>JiLang WebView关闭测试</title><style>body{font-family:Arial,sans-serif;margin:20px;background-color:#f0f8ff;text-align:center;}h1{color:#0066cc;}.button{display:inline-block;padding:10px 20px;margin:10px;background-color:#0066cc;color:white;border:none;border-radius:5px;cursor:pointer;}.close-btn{background-color:#cc0000;}.info{margin:20px;padding:15px;background-color:#e6f7ff;border:1px solid #b3e0ff;border-radius:5px;}</style></head><body><h1>关闭窗口测试</h1><div class='info'><p>请测试以下功能：</p><p>1. 点击\"调用JiLang函数\"按钮看是否有弹窗</p><p>2. 点击\"关闭此窗口\"按钮看是否能关闭窗口</p><p>3. 点击窗口右上角的X按钮看是否能关闭窗口</p></div><button id='test-jilang' class='button'>调用JiLang函数</button><button id='close-window' class='button close-btn'>关闭此窗口</button><div id='result-area' class='info'></div><script>function log(message){document.getElementById('result-area').innerHTML+='<p>'+message+'</p>';}document.getElementById('test-jilang').addEventListener('click',function(){log('尝试调用JiLang函数...');try{if(typeof sayHello==='function'){const result=sayHello('WebView窗口');log('函数调用结果: '+result);}else{log('错误: sayHello函数不可用，可能未正确绑定');}}catch(e){log('函数调用错误: '+e);}});document.getElementById('close-window').addEventListener('click',function(){log('尝试关闭窗口...');window.close();});log('页面已加载，可以开始测试');</script></body></html>"}},
                
                {"comment": "加载HTML内容"},
                {"windowwv.load_html": ["@var.window.id", "@var.html_content"]},
                {"echo": ["HTML内容已加载\n"]},
                
                {"comment": "绑定JavaScript函数到JiLang"},
                {"windowwv.bind": ["@var.window.id", "sayHello", "greet"]},
                {"echo": ["JavaScript函数已绑定\n"]},
                
                {"comment": "显示窗口"},
                {"windowwv.show": ["@var.window.id"]},
                {"echo": ["窗口已显示，开始测试...\n"]},
                
                {"comment": "运行事件循环，保持窗口打开直到关闭"},
                {"windowwv.run": ["@var.window.id"]},
                {"echo": ["窗口已关闭，测试完成\n"]}
            ]
        },
        
        "greet": {
            "params": ["name"],
            "body": [
                {"echo": ["收到WebView的调用: name=", "@param.name", "\n"]},
                {"return": {"value": "JiLang收到了调用，向你问好！"}}
            ]
        }
    }
} 