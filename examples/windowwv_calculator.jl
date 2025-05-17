{
    "include": ["windowwv", "io"],
    "program": {
        "main": {
            "body": [
                {"echo": ["启动JiLang WebView计算器应用...\n"]},
                
                {"comment": "创建窗口"},
                {"windowwv.create": ["JiLang计算器", 350, 450]},
                {"var": {"window": "@var.result"}},
                
                {"comment": "计算器的HTML/CSS/JS"},
                {"var": {"calculator_html": "<!DOCTYPE html>\n<html>\n<head>\n    <title>JiLang计算器</title>\n    <style>\n        body { font-family: Arial, sans-serif; background-color: #f0f0f0; margin: 0; padding: 20px; }\n        .calculator { background-color: #fff; border-radius: 10px; box-shadow: 0 5px 15px rgba(0,0,0,0.1); max-width: 300px; margin: 0 auto; padding: 15px; }\n        .display { background-color: #f8f8f8; border: 1px solid #ddd; border-radius: 5px; padding: 10px; margin-bottom: 15px; text-align: right; font-size: 24px; font-weight: bold; height: 40px; overflow: hidden; }\n        .buttons { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; }\n        button { border: none; border-radius: 5px; padding: 15px; font-size: 18px; cursor: pointer; transition: background-color 0.2s; }\n        .number { background-color: #e0e0e0; }\n        .number:hover { background-color: #d0d0d0; }\n        .operator { background-color: #f8a93b; color: white; }\n        .operator:hover { background-color: #e5950e; }\n        .equals { background-color: #4caf50; color: white; }\n        .equals:hover { background-color: #3d8b40; }\n        .clear { background-color: #f44336; color: white; }\n        .clear:hover { background-color: #d32f2f; }\n        .title { text-align: center; color: #333; margin-bottom: 15px; }\n        .footer { text-align: center; margin-top: 15px; font-size: 12px; color: #666; }\n    </style>\n</head>\n<body>\n    <div class=\"calculator\">\n        <h2 class=\"title\">JiLang计算器</h2>\n        <div class=\"display\" id=\"display\">0</div>\n        <div class=\"buttons\">\n            <button class=\"clear\" onclick=\"clearDisplay()\">C</button>\n            <button class=\"operator\" onclick=\"appendToDisplay('/')\">÷</button>\n            <button class=\"operator\" onclick=\"appendToDisplay('*')\">×</button>\n            <button class=\"operator\" onclick=\"deleteLastChar()\">⌫</button>\n            \n            <button class=\"number\" onclick=\"appendToDisplay('7')\">7</button>\n            <button class=\"number\" onclick=\"appendToDisplay('8')\">8</button>\n            <button class=\"number\" onclick=\"appendToDisplay('9')\">9</button>\n            <button class=\"operator\" onclick=\"appendToDisplay('-')\">-</button>\n            \n            <button class=\"number\" onclick=\"appendToDisplay('4')\">4</button>\n            <button class=\"number\" onclick=\"appendToDisplay('5')\">5</button>\n            <button class=\"number\" onclick=\"appendToDisplay('6')\">6</button>\n            <button class=\"operator\" onclick=\"appendToDisplay('+')\">+</button>\n            \n            <button class=\"number\" onclick=\"appendToDisplay('1')\">1</button>\n            <button class=\"number\" onclick=\"appendToDisplay('2')\">2</button>\n            <button class=\"number\" onclick=\"appendToDisplay('3')\">3</button>\n            <button class=\"equals\" onclick=\"calculate()\">=</button>\n            \n            <button class=\"number\" onclick=\"appendToDisplay('0')\" style=\"grid-column: span 2;\">0</button>\n            <button class=\"number\" onclick=\"appendToDisplay('.')\">.</button>\n            <button class=\"operator\" onclick=\"sendToJiLang()\">↑</button>\n        </div>\n        <div class=\"footer\">\n            <p>由JiLang WebView模块提供支持</p>\n            <p id=\"result-from-jilang\"></p>\n        </div>\n    </div>\n\n    <script>\n        let displayValue = '0';\n        const display = document.getElementById('display');\n        \n        function updateDisplay() {\n            display.textContent = displayValue;\n        }\n        \n        function appendToDisplay(value) {\n            if (displayValue === '0' && value !== '.') {\n                displayValue = value;\n            } else {\n                displayValue += value;\n            }\n            updateDisplay();\n        }\n        \n        function clearDisplay() {\n            displayValue = '0';\n            updateDisplay();\n        }\n        \n        function deleteLastChar() {\n            if (displayValue.length > 1) {\n                displayValue = displayValue.slice(0, -1);\n            } else {\n                displayValue = '0';\n            }\n            updateDisplay();\n        }\n        \n        function calculate() {\n            try {\n                // 替换显示符号为实际运算符\n                let expression = displayValue.replace('×', '*').replace('÷', '/');\n                \n                // 安全地计算表达式\n                const result = new Function('return ' + expression)();\n                \n                // 处理小数位数，最多显示8位小数\n                displayValue = Number.isInteger(result) ? result.toString() : result.toFixed(8).replace(/\\.?0+$/, '');\n                updateDisplay();\n                \n                // 将计算结果发送给JiLang处理（可选）\n                calculateInJiLang(expression, result);\n            } catch (error) {\n                displayValue = '错误';\n                updateDisplay();\n                setTimeout(clearDisplay, 1000);\n            }\n        }\n        \n        function calculateInJiLang(expression, result) {\n            // 这个函数将由JiLang绑定\n            console.log('请求JiLang计算: ' + expression);\n        }\n        \n        function sendToJiLang() {\n            // 发送当前表达式到JiLang\n            if (displayValue !== '0' && displayValue !== '错误') {\n                calculateInJiLang(displayValue, null);\n            }\n        }\n        \n        // 接收来自JiLang的结果\n        function setResultFromJiLang(result) {\n            document.getElementById('result-from-jilang').textContent = '来自JiLang: ' + result;\n            return true;\n        }\n    </script>\n</body>\n</html>"}},
                
                {"comment": "加载计算器HTML"},
                {"windowwv.load_html": ["@var.window.id", "@var.calculator_html"]},
                
                {"comment": "绑定JavaScript和JiLang之间的函数"},
                {"windowwv.bind": ["@var.window.id", "calculateInJiLang", "calculate_expression"]},
                {"windowwv.bind": ["@var.window.id", "setResultFromJiLang", "set_result"]},
                
                {"comment": "显示窗口"},
                {"windowwv.show": ["@var.window.id"]},
                
                {"echo": ["计算器已启动，窗口ID: ", "@var.window.id", "\n"]},
                
                {"comment": "运行事件循环"},
                {"windowwv.run": ["@var.window.id"]}
            ]
        },
        
        "calculate_expression": {
            "params": ["expression", "client_result"],
            "body": [
                {"comment": "记录表达式"},
                {"echo": ["收到计算请求: ", "@param.expression", "\n"]},
                
                {"comment": "在JiLang中评估表达式"},
                {"var": {"result": {"op": "eval", "expression": "@param.expression"}}},
                
                {"comment": "将结果发送回WebView"},
                {"windowwv.invoke": ["@var.window.id", "setResultFromJiLang", ["@var.result"]]},
                
                {"comment": "将计算历史记录保存到文件"},
                {"var": {"history_entry": {"expression": "@param.expression", "result": "@var.result", "timestamp": {"@date.now": []}}}},
                {"io.append_file": ["calculator_history.json", "@var.history_entry\n"]},
                
                {"return": {"value": "@var.result"}}
            ]
        },
        
        "set_result": {
            "params": ["result"],
            "body": [
                {"echo": ["设置结果: ", "@param.result", "\n"]},
                {"return": {"value": true}}
            ]
        }
    }
} 