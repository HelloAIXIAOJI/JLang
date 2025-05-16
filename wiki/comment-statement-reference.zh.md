# 注释系统（comment语句）

本文档详细介绍了JiLang的注释系统，包括注释语句的语法、用法和最佳实践。

## 基本语法

在JiLang中，注释使用`comment`语句实现，基本语法如下：

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "这是一条注释"}
      ]
    }
  }
}
```

其中：
- `comment`：关键字，表示这是一个注释语句
- 注释内容可以是任何字符串

## 基本用法

### 简单注释

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "这是一个简单的注释"},
        {"var": {"x": 10}},
        {"echo": ["x的值是：", "@var.x"]}
      ]
    }
  }
}
```

### 多行注释

尽管JSON不支持真正的多行字符串，但可以在注释中使用换行符：

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "这是第一行\n这是第二行\n这是第三行"},
        {"var": {"y": 20}},
        {"echo": ["y的值是：", "@var.y"]}
      ]
    }
  }
}
```

### 分段注释

可以使用多个注释语句来组织代码：

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "--- 变量初始化部分 ---"},
        {"var": {"name": "张三", "age": 25}},
        
        {"comment": "--- 业务逻辑处理部分 ---"},
        {"if": {
          "condition": {"op": "gte", "left": "@var.age", "right": 18},
          "then": [
            {"echo": ["@var.name", "是成年人"]}
          ],
          "else": [
            {"echo": ["@var.name", "是未成年人"]}
          ]
        }}
      ]
    }
  }
}
```

## 注释中使用变量引用

JiLang的注释可以包含变量引用，这些引用会被解析，但仅在调试模式下显示：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"count": 5}},
        {"comment": "当前计数：@var.count"},
        {"echo": ["计数值：", "@var.count"]}
      ]
    }
  }
}
```

在调试模式下运行时（使用`--debug`参数），注释中的变量引用会被解析并显示。

## 调试用途

`comment`语句在调试模式下会显示在控制台，这使其成为调试的有力工具：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"x": 10, "y": 20}},
        {"comment": "DEBUG: x=@var.x, y=@var.y"},
        {"var": {"sum": {"math.add": ["@var.x", "@var.y"]}}},
        {"comment": "DEBUG: 计算结果 sum=@var.sum"},
        {"echo": ["总和：", "@var.sum"]}
      ]
    }
  }
}
```

在正常模式下运行时，注释不会显示，但在调试模式下，可以看到变量的实时值。

## 特殊用途注释

### 代码分区注释

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "========================="},
        {"comment": "== 用户信息处理部分 =="},
        {"comment": "========================="},
        {"var": {"user": {"name": "张三", "age": 25}}},
        
        {"comment": "========================="},
        {"comment": "== 数据计算部分 =="},
        {"comment": "========================="},
        {"var": {"result": {"math.multiply": ["@var.user.age", 2]}}}
      ]
    }
  }
}
```

### TODO注释

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"items": ["苹果", "香蕉", "橙子"]}},
        {"comment": "TODO: 添加对数组的排序功能"},
        {"echo": ["当前物品：", "@var.items"]}
      ]
    }
  }
}
```

### 禁用代码注释

要暂时禁用某段代码，可以将其转换为注释：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"debug": true}},
        {"comment": "以下代码暂时禁用"},
        {"comment": "原代码：{\"echo\": [\"调试信息：\", \"@var.debug\"]}"},
        {"echo": ["正常运行"]}
      ]
    }
  }
}
```

## 注释与调试模式

JiLang的注释行为会根据解释器的运行模式而变化：

### 正常模式

在正常模式下，注释语句不会产生任何输出或副作用：

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "这条注释在正常模式下不可见"},
        {"echo": ["这行会显示"]}
      ]
    }
  }
}
```

执行结果（正常模式）：
```
这行会显示
```

### 调试模式

在调试模式下（使用`--debug`参数），注释语句会显示在控制台，并带有特殊前缀：

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "这条注释在调试模式下可见"},
        {"echo": ["这行会显示"]}
      ]
    }
  }
}
```

执行结果（调试模式）：
```
[COMMENT] 这条注释在调试模式下可见
这行会显示
```

## 注释的评估行为

虽然注释主要是为了代码可读性，但在JiLang中，注释内容会被解释器评估，特别是其中的变量引用：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"status": "运行中"}},
        {"comment": "当前状态: @var.status"},
        {"var": {"status": "已完成"}},
        {"comment": "更新后状态: @var.status"}
      ]
    }
  }
}
```

在调试模式下，这会显示：
```
[COMMENT] 当前状态: 运行中
[COMMENT] 更新后状态: 已完成
```

## 使用场景

### 代码文档

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "此程序计算斐波那契数列的前10个数"},
        {"var": {"fib": [0, 1], "i": 2}},
        {"while": {
          "condition": {"op": "lt", "left": "@var.i", "right": 10},
          "body": [
            {"comment": "计算下一个斐波那契数"},
            {"var": {"next": {"op": "add", "left": "@var.fib[@var.i-1]", "right": "@var.fib[@var.i-2]"}}},
            {"var": {"fib": {"array.append": ["@var.fib", "@var.next"]}}},
            {"var": {"i": {"op": "add", "left": "@var.i", "right": 1}}}
          ]
        }},
        {"echo": ["斐波那契数列：", "@var.fib"]}
      ]
    }
  }
}
```

### 调试信息

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"items": [10, 20, 30, 40, 50]}},
        {"var": {"total": 0, "i": 0}},
        {"while": {
          "condition": {"op": "lt", "left": "@var.i", "right": 5},
          "body": [
            {"comment": "迭代 @var.i: 当前值=@var.items[@var.i], 累计=@var.total"},
            {"var": {"total": {"op": "add", "left": "@var.total", "right": "@var.items[@var.i]"}}},
            {"var": {"i": {"op": "add", "left": "@var.i", "right": 1}}},
            {"comment": "更新后: 累计=@var.total, 索引=@var.i"}
          ]
        }},
        {"echo": ["总和：", "@var.total"]}
      ]
    }
  }
}
```

## 最佳实践

### 注释命名约定

为提高代码可读性，可以使用一致的注释前缀：

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "TODO: 添加数据验证"},
        {"comment": "FIXME: 修复数组越界问题"},
        {"comment": "NOTE: 这里使用了特殊处理"},
        {"comment": "WARNING: 这个函数在大量数据时性能较低"},
        {"comment": "DEBUG: x=10, y=20, z=30"}
      ]
    }
  }
}
```

### 注释与格式化

为提高可读性，使用适当的分隔符：

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "===================================="},
        {"comment": "=== 初始化部分 ==="},
        {"comment": "===================================="},
        {"var": {"config": {"debug": true, "maxItems": 100}}},
        
        {"comment": "------------------------------------"},
        {"comment": "--- 主程序逻辑 ---"},
        {"comment": "------------------------------------"},
        {"echo": ["配置已加载"]}
      ]
    }
  }
}
```

### 调试开关

结合变量和条件语句，可以创建一个简单的调试开关：

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"DEBUG": true}},
        {"var": {"x": 10, "y": 20}},
        
        {"if": {
          "condition": {"op": "eq", "left": "@var.DEBUG", "right": true},
          "then": [
            {"comment": "调试信息: x=@var.x, y=@var.y"}
          ]
        }},
        
        {"var": {"result": {"op": "add", "left": "@var.x", "right": "@var.y"}}},
        
        {"if": {
          "condition": {"op": "eq", "left": "@var.DEBUG", "right": true},
          "then": [
            {"comment": "计算结果: @var.result"}
          ]
        }},
        
        {"echo": ["最终结果：", "@var.result"]}
      ]
    }
  }
}
```

## 技术实现

`comment`语句在JiLang解释器中的实现相对简单：

1. 解析注释内容字符串
2. 处理其中的变量引用（如果有）
3. 在调试模式下将结果输出到控制台
4. 在正常模式下，不执行任何操作

注释语句不会改变程序的执行流程或修改任何变量，它仅用于提供代码说明和调试信息。 