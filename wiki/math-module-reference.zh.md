# Math标准库模块

本文档详细介绍了JiLang中的Math标准库模块，该模块提供了基本的数学运算功能。

## 模块概述

Math模块是JiLang内置的标准库模块，提供了基础的数学运算函数。作为内置模块，在任何JiLang程序中都可以直接使用，无需额外安装。

## 导入模块

在JiLang程序中使用以下方式导入Math模块：

```json
{
  "include": ["math"],
  "program": {
    // 程序主体
  }
}
```

## 提供的函数

Math模块提供以下核心功能：

| 函数名 | 说明 |
|-------|------|
| add | 计算多个数值的和 |
| subtract | 从第一个参数中减去后续所有参数 |
| multiply | 计算多个数值的乘积 |
| divide | 第一个参数除以后续所有参数 |
| pow | 计算一个数的幂 |
| sqrt | 计算平方根 |
| round | 四舍五入到最接近的整数 |

### add

#### 描述
计算所有参数的和。

#### 参数
- 任意数量的数值参数

#### 返回值
所有参数的和。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.add": [5, 10, 15], "output": "sum"},
        {"echo": ["5 + 10 + 15 = ", "@var.sum"]}
      ]
    }
  }
}
```

#### 特殊处理

- 空参数列表返回0
- 非数值参数会尝试转换为数值
- 字符串中包含的数字会被提取（例如 "10px" 会被转换为10）

### subtract

#### 描述
从第一个参数中减去所有后续参数。

#### 参数
- 至少一个数值参数
- 任意数量的后续数值参数

#### 返回值
减法运算的结果。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.subtract": [20, 5, 3], "output": "difference"},
        {"echo": ["20 - 5 - 3 = ", "@var.difference"]}
      ]
    }
  }
}
```

#### 特殊处理

- 只有一个参数时，直接返回该参数
- 空参数列表返回0
- 非数值参数会尝试转换为数值

### multiply

#### 描述
计算所有参数的乘积。

#### 参数
- 任意数量的数值参数

#### 返回值
所有参数的乘积。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.multiply": [2, 3, 4], "output": "product"},
        {"echo": ["2 * 3 * 4 = ", "@var.product"]}
      ]
    }
  }
}
```

#### 特殊处理

- 空参数列表返回0
- 任何参数为0时，结果可能为0
- 非数值参数会尝试转换为数值

### divide

#### 描述
第一个参数除以所有后续参数。

#### 参数
- 至少一个数值参数（被除数）
- 任意数量的后续数值参数（除数，不能为0）

#### 返回值
除法运算的结果。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.divide": [100, 5, 2], "output": "quotient"},
        {"echo": ["100 / 5 / 2 = ", "@var.quotient"]}
      ]
    }
  }
}
```

#### 错误处理

- 如果任何除数为0，会产生除零错误
- 空参数列表返回0
- 非数值参数会尝试转换为数值

### pow

#### 描述
计算一个数的幂。

#### 参数
- **base**: 底数
- **exponent**: 指数

#### 返回值
底数的指数次幂（base^exponent）。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.pow": [2, 3], "output": "result"},
        {"echo": ["2的3次方 = ", "@var.result"]}
      ]
    }
  }
}
```

#### 特殊处理

- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### sqrt

#### 描述
计算一个数的平方根。

#### 参数
- **value**: 要计算平方根的值

#### 返回值
参数的平方根。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.sqrt": [16], "output": "result"},
        {"echo": ["16的平方根 = ", "@var.result"]}
      ]
    }
  }
}
```

#### 特殊处理

- 负数的平方根在JiLang中会返回NaN
- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### round

#### 描述
将数值四舍五入到最接近的整数。

#### 参数
- **value**: 要四舍五入的值

#### 返回值
四舍五入后的整数。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.round": [4.3], "output": "round_down"},
        {"echo": ["4.3四舍五入 = ", "@var.round_down"]},
        
        {"math.round": [4.7], "output": "round_up"},
        {"echo": ["4.7四舍五入 = ", "@var.round_up"]}
      ]
    }
  }
}
```

#### 特殊处理

- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

## 类型转换

Math模块中的所有函数都会尝试将输入参数转换为数值：

1. **数字**：直接使用
2. **字符串**：尝试解析为数字
   - 如果字符串可以直接解析为数字，使用解析结果
   - 如果字符串包含数字和非数字字符（如 "10px"），会尝试提取数字部分
3. **布尔值**：true转换为1，false转换为0
4. **空数组或空对象**：转换为0
5. **null**：转换为0
6. **变量引用**：先解析变量，再按上述规则转换

## 组合使用示例

### 计算圆的面积

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"var": {"PI": 3.14159}},
        {"var": {"radius": 5}},
        
        {"comment": "计算面积：PI * radius^2"},
        {"math.pow": ["@var.radius", 2], "output": "radius_squared"},
        {"math.multiply": ["@var.PI", "@var.radius_squared"], "output": "area"},
        
        {"echo": ["半径为", "@var.radius", "的圆的面积是: ", "@var.area"]}
      ]
    }
  }
}
```

### 综合计算示例

```json
{
  "include": ["math", "io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["数学计算示例\n"]},
        
        {"io.input": ["请输入一个数字: "], "output": "input_number"},
        
        {"comment": "计算各种数学运算结果"},
        {"math.sqrt": ["@var.input_number"], "output": "sqrt_result"},
        {"math.pow": ["@var.input_number", 2], "output": "square_result"},
        {"math.pow": ["@var.input_number", 3], "output": "cube_result"},
        
        {"echo": ["\n计算结果:"]},
        {"echo": ["原始数字: ", "@var.input_number"]},
        {"echo": ["平方根: ", "@var.sqrt_result"]},
        {"echo": ["平方: ", "@var.square_result"]},
        {"echo": ["立方: ", "@var.cube_result"]}
      ]
    }
  }
}
```

## 自定义数学函数

可以通过组合Math模块的基础函数创建更复杂的数学功能：

### 示例：平均值计算

```json
{
  "include": ["math"],
  "program": {
    "calculate_average": {
      "params": {
        "numbers": "array"
      },
      "body": [
        {"var": {"sum": 0, "count": 0}},
        {"for": {
          "var": "num",
          "in": "@params.numbers",
          "body": [
            {"var": {"sum": {"math.add": ["@var.sum", "@var.num"]}}},
            {"var": {"count": {"math.add": ["@var.count", 1]}}}
          ]
        }},
        {"var": {"result": {"math.divide": ["@var.sum", "@var.count"]}}}
      ]
    },
    "main": {
      "body": [
        {"var": {"data": [10, 20, 30, 40, 50]}},
        {"calculate_average": ["@var.data"], "output": "avg"},
        {"echo": ["平均值: ", "@var.avg"]}
      ]
    }
  }
}
```

## 最佳实践

1. **类型检查**：虽然Math模块会尝试自动转换类型，但应该尽量传入正确的数值类型
2. **错误处理**：特别注意处理除零错误，确保除法操作的除数不为零
3. **精度考虑**：JiLang使用浮点数进行计算，可能会有精度损失，特别是在处理非常大或非常小的数字时
4. **组合函数**：对于复杂的数学计算，可以自定义函数将基本操作组合起来

## 限制与注意事项

1. 所有计算都使用双精度浮点数，可能有精度限制
2. 对于非常大或非常小的数字，可能会出现精度问题或溢出
3. Math模块还可能添加更多的数学函数，查看最新文档以获取完整列表
4. 在处理用户输入时，应始终验证是否可以成功转换为数值 