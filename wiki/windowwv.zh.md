# WindowWV 模块

WindowWV 模块为 JiLang 提供了创建和操作 WebView 窗口的能力，让你可以在应用程序中显示 HTML 内容并与之交互。

## 简介

WebView 是一种轻量级的浏览器控件，可以嵌入到应用程序中来显示网页内容。通过 WindowWV 模块，你可以：

- 创建独立的窗口来显示 HTML 内容
- 加载本地 HTML 字符串或远程 URL
- 在 JavaScript 和 JiLang 之间建立通信桥梁
- 控制窗口的显示、大小和标题

这使得在 JiLang 应用中创建图形用户界面变得简单可行。

## 函数列表

### 基本窗口操作

| 函数 | 描述 | 参数 | 返回值 |
| ---- | ---- | ---- | ---- |
| `windowwv.create` | 创建一个新的 WebView 窗口 | `title`, `width`, `height`, [`options`] | 窗口对象，包含 `id` 和窗口信息 |
| `windowwv.show` | 显示指定的窗口 | `window_id` | 成功状态 |
| `windowwv.close` | 关闭指定的窗口 | `window_id` | 成功状态 |
| `windowwv.set_title` | 设置窗口标题 | `window_id`, `title` | 成功状态 |
| `windowwv.set_size` | 设置窗口大小 | `window_id`, `width`, `height` | 成功状态 |

### 内容操作

| 函数 | 描述 | 参数 | 返回值 |
| ---- | ---- | ---- | ---- |
| `windowwv.load_html` | 加载 HTML 内容到窗口 | `window_id`, `html_content`, [`base_url`] | 成功状态 |
| `windowwv.load_url` | 加载指定 URL 到窗口 | `window_id`, `url` | 成功状态 |
| `windowwv.eval` | 在窗口中执行 JavaScript 代码 | `window_id`, `js_code` | 执行结果 |

### JavaScript 与 JiLang 交互

| 函数 | 描述 | 参数 | 返回值 |
| ---- | ---- | ---- | ---- |
| `windowwv.bind` | 将 JavaScript 函数绑定到 JiLang 函数 | `window_id`, `js_function_name`, `jilang_function_name` | 成功状态 |
| `windowwv.invoke` | 从 JiLang 调用 JavaScript 函数 | `window_id`, `js_function_name`, [`arguments`] | 调用结果 |
| `windowwv.on` | 设置事件处理程序 | `event_config` 对象 | 成功状态 |

### 事件循环控制

| 函数 | 描述 | 参数 | 返回值 |
| ---- | ---- | ---- | ---- |
| `windowwv.run` | 运行窗口的事件循环，阻塞直到窗口关闭 | `window_id` | 成功状态 |
| `windowwv.run_all` | 运行所有窗口的事件循环 | 无 | 成功状态 |

## 使用示例

### 创建基本窗口

```json
{
  "include": ["windowwv"],
  "program": {
    "main": {
      "body": [
        {"windowwv.create": ["我的窗口", 800, 600]},
        {"var": {"window": "@var.result"}},
        {"var": {"html": "<html><body><h1>Hello JiLang!</h1></body></html>"}},
        {"windowwv.load_html": ["@var.window.id", "@var.html"]},
        {"windowwv.show": ["@var.window.id"]},
        {"windowwv.run": ["@var.window.id"]}
      ]
    }
  }
}
```

### JavaScript 与 JiLang 交互

```json
{
  "include": ["windowwv"],
  "program": {
    "main": {
      "body": [
        {"windowwv.create": ["交互窗口", 800, 600]},
        {"var": {"window": "@var.result"}},
        {"var": {"html": "<html><body><h1>交互测试</h1><button onclick='sayHello(\"WebView\")'>调用 JiLang</button></body></html>"}},
        {"windowwv.load_html": ["@var.window.id", "@var.html"]},
        {"windowwv.bind": ["@var.window.id", "sayHello", "greet"]},
        {"windowwv.show": ["@var.window.id"]},
        {"windowwv.run": ["@var.window.id"]}
      ]
    },
    "greet": {
      "params": ["name"],
      "body": [
        {"echo": ["收到来自 WebView 的问候: ", "@param.name", "\n"]},
        {"return": {"value": "JiLang 向你问好!"}}
      ]
    }
  }
}
```

## 高级用法

### 窗口选项

创建窗口时可以指定选项对象来控制窗口行为：

```json
{"var": {"window_options": {
  "resizable": true,
  "decorations": true
}}},
{"windowwv.create": ["可调整大小的窗口", 800, 600, "@var.window_options"]}
```

可用选项：
- `resizable`：窗口是否可调整大小（布尔值）
- `decorations`：窗口是否显示边框和标题栏（布尔值）

### 事件处理

可以使用 `windowwv.on` 函数设置事件处理程序：

```json
{"windowwv.on": {
  "window": "@var.window.id",
  "event": "click",
  "body": [
    {"echo": ["检测到点击事件\n"]}
  ]
}}
```

## 限制和注意事项

1. 窗口必须通过 `windowwv.run` 或 `windowwv.run_all` 启动事件循环才能保持打开状态
2. 事件循环会阻塞当前线程，直到窗口关闭
3. JavaScript 与 JiLang 之间的通信是同步的，复杂操作可能导致界面暂时冻结
4. 窗口创建成功后会返回一个包含 `id` 属性的对象，必须保存此 ID 以便后续操作

## 示例项目

JiLang 提供了多个 WebView 示例项目：

1. `examples/simple_windowwv.jl`：基本的 WebView 创建和显示示例
2. `examples/test_windowwv.jl`：展示各种 WebView 功能的综合测试
3. `examples/windowwv_calculator.jl`：一个功能完整的计算器应用示例

通过研究这些示例，你可以更好地了解如何在实际项目中使用 WindowWV 模块。 