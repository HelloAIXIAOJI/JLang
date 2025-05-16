# 条件判断系统

本文档详细介绍了JiLang中的条件判断系统，包括条件表达式的语法、支持的操作符以及条件评估的工作原理。

## 条件表达式基本语法

JiLang中的条件表达式使用对象格式，基本语法如下：

```json
{
  "op": "操作符",
  "left": "左操作数",
  "right": "右操作数"
}
```

其中：
- `op`：表示比较操作符，如`eq`（等于）、`gt`（大于）等
- `left`：左侧操作数，可以是字面量、变量引用或嵌套表达式
- `right`：右侧操作数，可以是字面量、变量引用或嵌套表达式

## 支持的操作符

JiLang条件判断系统支持以下操作符：

| 操作符 | 描述 | 示例 |
|-------|------|------|
| eq | 等于 | `{"op": "eq", "left": "A", "right": "A"}` |
| neq | 不等于 | `{"op": "neq", "left": "A", "right": "B"}` |
| gt | 大于 | `{"op": "gt", "left": 10, "right": 5}` |
| lt | 小于 | `{"op": "lt", "left": 5, "right": 10}` |
| gte | 大于等于 | `{"op": "gte", "left": 10, "right": 10}` |
| lte | 小于等于 | `{"op": "lte", "left": 5, "right": 10}` |
| and | 逻辑与 | `{"op": "and", "left": true, "right": true}` |
| or | 逻辑或 | `{"op": "or", "left": true, "right": false}` |

## 条件表达式示例

### 简单比较

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {
          "condition": {"op": "eq", "left": "@var.age", "right": 25},
          "then": [
            {"echo": ["年龄是25岁"]}
          ],
          "else": [
            {"echo": ["年龄不是25岁"]}
          ]
        }}
      ]
    }
  }
}
```

### 数值比较

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {
          "condition": {"op": "gte", "left": "@var.score", "right": 60},
          "then": [
            {"echo": ["及格"]}
          ],
          "else": [
            {"echo": ["不及格"]}
          ]
        }}
      ]
    }
  }
}
```

### 字符串比较

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {
          "condition": {"op": "eq", "left": "@var.name", "right": "张三"},
          "then": [
            {"echo": ["你好，张三"]}
          ],
          "else": [
            {"echo": ["你好，访客"]}
          ]
        }}
      ]
    }
  }
}
```

### 复合条件（逻辑运算）

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {
          "condition": {
            "op": "and",
            "left": {"op": "gte", "left": "@var.age", "right": 18},
            "right": {"op": "lt", "left": "@var.age", "right": 65}
          },
          "then": [
            {"echo": ["成年人"]}
          ],
          "else": [
            {"echo": ["未成年或老年人"]}
          ]
        }}
      ]
    }
  }
}
```

## 不同数据类型的比较

JiLang的条件判断系统支持不同数据类型之间的比较，遵循以下规则：

### 数字与数字比较

直接比较数值大小。

```json
{"op": "gt", "left": 10, "right": 5}  // 结果为true
```

### 字符串与字符串比较

按字典序比较。

```json
{"op": "gt", "left": "B", "right": "A"}  // 结果为true
```

### 布尔值与布尔值比较

`true`大于`false`。

```json
{"op": "gt", "left": true, "right": false}  // 结果为true
```

### 数字与字符串比较

尝试将字符串转换为数字后比较；如果转换失败，则按字符串比较。

```json
{"op": "eq", "left": 123, "right": "123"}  // 结果为true
```

### 布尔值与数字比较

布尔值转换为数字（`true` -> 1, `false` -> 0）后比较。

```json
{"op": "eq", "left": true, "right": 1}  // 结果为true
```

### null值比较

`null`在数值比较中被视为0。

```json
{"op": "eq", "left": null, "right": 0}  // 结果为true
```

## 条件评估工作原理

JiLang的条件判断由`evaluate_condition`函数实现，其工作流程如下：

1. 检查条件是否为对象，并包含`op`、`left`和`right`属性
2. 解析左右操作数，处理变量引用
3. 根据类型执行适当的比较：
   - 如果两边都可以转换为数字，进行数值比较
   - 对于特殊类型（布尔值、null），应用转换规则
   - 否则进行字符串比较
4. 返回条件评估结果（true或false）

### 执行流程示例

以下是条件评估的简化流程，以`{"op": "eq", "left": "@var.age", "right": 25}`为例：

1. 提取左操作数`@var.age`，假设其值为25
2. 提取右操作数`25`
3. 因为两个操作数都是数字，进行数值比较
4. 检查`25 == 25`，结果为`true`

## 类型自动转换

条件判断系统会尝试进行以下类型转换：

1. **字符串到数字**：如果字符串可以解析为数字，则转换
   ```json
   {"op": "eq", "left": "123", "right": 123}  // true
   ```

2. **布尔值到数字**：`true`转换为1，`false`转换为0
   ```json
   {"op": "eq", "left": true, "right": 1}  // true
   ```

3. **null到数字**：`null`转换为0
   ```json
   {"op": "eq", "left": null, "right": 0}  // true
   ```

4. **空数组/对象到数字**：空数组或空对象转换为0
   ```json
   {"op": "eq", "left": [], "right": 0}  // true
   ```

## 在控制结构中使用条件

条件表达式主要在以下语句中使用：

### if语句

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {
          "condition": {"op": "eq", "left": "@var.day", "right": "周末"},
          "then": [
            {"echo": ["休息日"]}
          ],
          "else": [
            {"echo": ["工作日"]}
          ]
        }}
      ]
    }
  }
}
```

### while循环

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"count": 1}},
        {"while": {
          "condition": {"op": "lte", "left": "@var.count", "right": 5},
          "body": [
            {"echo": ["计数: ", "@var.count"]},
            {"var": {"count": {"op": "add", "left": "@var.count", "right": 1}}}
          ]
        }}
      ]
    }
  }
}
```

### for循环条件

```json
{
  "program": {
    "main": {
      "body": [
        {"for": {
          "init": {"var": {"i": 0}},
          "condition": {"op": "lt", "left": "@var.i", "right": 5},
          "update": {"var": {"i": {"op": "add", "left": "@var.i", "right": 1}}},
          "body": [
            {"echo": ["i = ", "@var.i"]}
          ]
        }}
      ]
    }
  }
}
```

## 常见问题与最佳实践

### 类型不匹配问题

不同类型的比较可能会导致意外结果，建议确保比较的值类型一致。

```json
{
  "program": {
    "main": {
      "body": [
        // 潜在问题：字符串与数字比较
        {"if": {"condition": {"op": "eq", "left": "@var.value", "right": 100}}},
        
        // 更好的做法：确保类型一致
        {"var": {"numValue": {"math.convert_to_number": ["@var.value"]}}},
        {"if": {"condition": {"op": "eq", "left": "@var.numValue", "right": 100}}}
      ]
    }
  }
}
```

### 字符串比较陷阱

字符串比较是区分大小写的，需要注意。

```json
{
  "program": {
    "main": {
      "body": [
        // "A" 不等于 "a"
        {"if": {"condition": {"op": "eq", "left": "A", "right": "a"}}}  // false
      ]
    }
  }
}
```

### null和空字符串

`null`和空字符串（`""`）是不同的值。

```json
{
  "program": {
    "main": {
      "body": [
        {"if": {"condition": {"op": "eq", "left": null, "right": ""}}}  // false
      ]
    }
  }
}
```

### 布尔值表示

在判断真假值时，以下值会被视为假：
- 空字符串 `""`
- 字符串 `"0"`
- 字符串 `"false"`
- 数字 `0`
- 布尔值 `false`

其他值被视为真。

## 技术实现

条件评估功能在JiLang解释器中由`evaluate_condition`函数实现，支持：

1. 变量引用的解析
2. 不同类型间的智能比较
3. 数值和字符串比较
4. 布尔值和null等特殊值的处理
5. 逻辑运算（and, or） 