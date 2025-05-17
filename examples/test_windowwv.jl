{
    "include": ["windowwv"],
    "program": {
        "main": {
            "body": [
                {"comment": "创建一个WebView窗口"},
                {"var": {"window_options": {
                    "resizable": true,
                    "decorations": true,
                    "visible": false
                }}},
                {"windowwv.create": ["JiLang WebView Window (WindowWV)测试", 800, 600, "@var.window_options"]},
                {"var": {"window": "@var.result"}},
                
                {"comment": "定义一个简单的HTML内容"},
                {"var": {"html_content": "<!DOCTYPE html><html><head><title>JiLang WebView</title><style>body{font-family:Arial,sans-serif;margin:20px;color:#333;} h1{color:#0066cc;} button{padding:8px 15px;background:#0066cc;color:#fff;border:none;border-radius:4px;cursor:pointer;} button:hover{background:#004c99;}</style></head><body><h1>JiLang WebView 测试</h1><p>这是一个使用JiLang的WebView模块创建的窗口。</p><button id='testBtn'>点击我</button><div id='output'></div><script>document.getElementById('testBtn').addEventListener('click',function(){document.getElementById('output').innerHTML += '<p>按钮被点击了!</p>';});</script></body></html>"}},
                
                {"comment": "加载HTML内容"},
                {"windowwv.load_html": ["@var.window.id", "@var.html_content"]},
                
                {"comment": "显示窗口"},
                {"windowwv.show": ["@var.window.id"]},
                
                {"comment": "添加JavaScript代码"},
                {"windowwv.eval": ["@var.window.id", "console.log('JiLang WebView 已启动');"]},
                
                {"comment": "绑定JavaScript函数"},
                {"windowwv.bind": ["@var.window.id", "sayHello", "greet"]},
                
                {"comment": "通过eval注入一个调用按钮"},
                {"windowwv.eval": ["@var.window.id", "const btn = document.createElement('button'); btn.innerText = '调用JiLang函数'; btn.style.marginTop = '10px'; btn.onclick = function() { sayHello('WebView'); }; document.body.appendChild(btn);"]},
                
                {"comment": "设置窗口事件处理"},
                {"windowwv.on": {
                    "window": "@var.window.id",
                    "event": "click",
                    "body": [
                        {"echo": ["检测到点击事件\n"]}
                    ]
                }},
                
                {"comment": "运行WebView事件循环"},
                {"windowwv.run": ["@var.window.id"]}
            ]
        },
        
        "greet": {
            "params": ["name"],
            "body": [
                {"echo": ["收到来自WebView的问候: ", "@param.name", "\n"]},
                {"return": {"value": "JiLang向你问好!"}}
            ]
        }
    }
} 