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
| abs | 计算绝对值 |
| sin | 计算正弦值（角度制） |
| cos | 计算余弦值（角度制） |
| tan | 计算正切值（角度制） |
| log | 计算以10为底的对数 |
| ln | 计算自然对数（以e为底） |
| max | 计算最大值 |
| min | 计算最小值 |
| floor | 向下取整 |
| ceil | 向上取整 |
| random | 生成随机数 |

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
        {"math.round": [3.7], "output": "result1"},
        {"echo": ["3.7四舍五入 = ", "@var.result1"]},
        {"math.round": [3.2], "output": "result2"},
        {"echo": ["3.2四舍五入 = ", "@var.result2"]}
      ]
    }
  }
}
```

#### 特殊处理

- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### abs

#### 描述
计算一个数的绝对值。

#### 参数
- **value**: 要计算绝对值的数

#### 返回值
参数的绝对值。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.abs": [-42], "output": "result"},
        {"echo": ["-42的绝对值 = ", "@var.result"]}
      ]
    }
  }
}
```

#### 特殊处理

- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### sin

#### 描述
计算角度的正弦值（使用角度制，而非弧度制）。

#### 参数
- **degrees**: 角度值（以度为单位）

#### 返回值
指定角度的正弦值。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.sin": [30], "output": "result"},
        {"echo": ["30度的正弦值 = ", "@var.result"]}
      ]
    }
  }
}
```

#### 特殊处理

- 内部会将角度转换为弧度后计算
- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### cos

#### 描述
计算角度的余弦值（使用角度制，而非弧度制）。

#### 参数
- **degrees**: 角度值（以度为单位）

#### 返回值
指定角度的余弦值。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.cos": [60], "output": "result"},
        {"echo": ["60度的余弦值 = ", "@var.result"]}
      ]
    }
  }
}
```

#### 特殊处理

- 内部会将角度转换为弧度后计算
- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### tan

#### 描述
计算角度的正切值（使用角度制，而非弧度制）。

#### 参数
- **degrees**: 角度值（以度为单位）

#### 返回值
指定角度的正切值。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.tan": [45], "output": "result"},
        {"echo": ["45度的正切值 = ", "@var.result"]}
      ]
    }
  }
}
```

#### 特殊处理

- 内部会将角度转换为弧度后计算
- 如果参数不足，返回0
- 非数值参数会尝试转换为数值
- 90度（以及270度、450度等）会导致无穷大的结果

### log

#### 描述
计算以10为底的对数。

#### 参数
- **value**: 要计算对数的正数

#### 返回值
参数的以10为底的对数。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.log": [100], "output": "result"},
        {"echo": ["log10(100) = ", "@var.result"]}
      ]
    }
  }
}
```

#### 特殊处理

- 参数必须为正数，否则会产生错误
- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### ln

#### 描述
计算自然对数（以e为底的对数）。

#### 参数
- **value**: 要计算自然对数的正数

#### 返回值
参数的自然对数值。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"var": {"e": 2.718281828459045}},
        {"math.ln": "@var.e", "output": "result"},
        {"echo": ["ln(e) = ", "@var.result"]}
      ]
    }
  }
}
```

#### 特殊处理

- 参数必须为正数，否则会产生错误
- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### max

#### 描述
计算一组数值的最大值。

#### 参数
- 任意数量的数值参数，或者一个数值数组

#### 返回值
所有参数中的最大值。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.max": [10, 5, 20, 15], "output": "result1"},
        {"echo": ["最大值 = ", "@var.result1"]},
        
        {"var": {"numbers": [3, 8, 1, 6, 2]}},
        {"math.max": "@var.numbers", "output": "result2"},
        {"echo": ["数组最大值 = ", "@var.result2"]}
      ]
    }
  }
}
```

#### 特殊处理

- 如果第一个参数是数组，会计算数组中的最大值
- 如果参数是变量引用且指向数组，也会计算该数组的最大值
- 如果参数为空，返回0
- 非数值参数会尝试转换为数值

### min

#### 描述
计算一组数值的最小值。

#### 参数
- 任意数量的数值参数，或者一个数值数组

#### 返回值
所有参数中的最小值。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.min": [10, 5, 20, 15], "output": "result1"},
        {"echo": ["最小值 = ", "@var.result1"]},
        
        {"var": {"numbers": [3, 8, 1, 6, 2]}},
        {"math.min": "@var.numbers", "output": "result2"},
        {"echo": ["数组最小值 = ", "@var.result2"]}
      ]
    }
  }
}
```

#### 特殊处理

- 如果第一个参数是数组，会计算数组中的最小值
- 如果参数是变量引用且指向数组，也会计算该数组的最小值
- 如果参数为空，返回0
- 非数值参数会尝试转换为数值

### floor

#### 描述
向下取整，即取不大于给定数值的最大整数。

#### 参数
- **value**: 要向下取整的数值

#### 返回值
不大于参数的最大整数。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.floor": [7.8], "output": "result1"},
        {"echo": ["floor(7.8) = ", "@var.result1"]},
        
        {"math.floor": [-3.2], "output": "result2"},
        {"echo": ["floor(-3.2) = ", "@var.result2"]}
      ]
    }
  }
}
```

#### 特殊处理

- 对于负数，向下取整会得到更小的数（例如，floor(-3.2) = -4）
- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### ceil

#### 描述
向上取整，即取不小于给定数值的最小整数。

#### 参数
- **value**: 要向上取整的数值

#### 返回值
不小于参数的最小整数。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.ceil": [7.8], "output": "result1"},
        {"echo": ["ceil(7.8) = ", "@var.result1"]},
        
        {"math.ceil": [-3.2], "output": "result2"},
        {"echo": ["ceil(-3.2) = ", "@var.result2"]}
      ]
    }
  }
}
```

#### 特殊处理

- 对于负数，向上取整会得到更大的数（例如，ceil(-3.2) = -3）
- 如果参数不足，返回0
- 非数值参数会尝试转换为数值

### random

#### 描述
生成随机数。

#### 参数
- 无参数：生成0到1之间的随机数
- 一个参数 **max**：生成0到max之间的随机数
- 两个参数 **min**, **max**：生成min到max之间的随机数

#### 返回值
指定范围内的随机浮点数。

#### 示例

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.random": [], "output": "result1"},
        {"echo": ["0到1之间的随机数: ", "@var.result1"]},
        
        {"math.random": [100], "output": "result2"},
        {"echo": ["0到100之间的随机数: ", "@var.result2"]},
        
        {"math.random": [50, 100], "output": "result3"},
        {"echo": ["50到100之间的随机数: ", "@var.result3"]}
      ]
    }
  }
}
```

#### 特殊处理

- 如果指定了min和max，但min大于等于max，会产生错误
- 生成的随机数是浮点数，包括下限但不包括上限
- 非数值参数会尝试转换为数值

## 类型转换

Math模块中的函数会尝试将输入参数转换为数值，遵循以下规则：

1. **数值类型**：直接使用
2. **字符串类型**：
   - 尝试将整个字符串解析为数字
   - 如果失败，尝试从字符串中提取数字部分
   - 如果字符串是变量引用，先解析引用再转换
3. **布尔类型**：true转换为1.0，false转换为0.0
4. **null类型**：转换为0.0
5. **空数组/空对象**：转换为0.0
6. **非空数组/非空对象**：通常不能直接转换，但某些函数(如max/min)有特殊处理

## 函数组合

Math模块的函数可以组合使用来完成复杂计算。例如，计算圆的面积：

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"var": {"radius": 5}},
        {"math.pow": ["@var.radius", 2], "output": "radius_squared"},
        {"math.multiply": ["@var.radius_squared", 3.14159], "output": "area"},
        {"echo": ["半径为5的圆面积 = ", "@var.area"]}
      ]
    }
  }
}
```

## 最佳实践

使用Math模块时，可以参考以下最佳实践：

1. **类型检查**：虽然Math模块会尝试转换输入数据类型，但最好传入正确的数值类型
2. **错误处理**：注意特殊情况如除以零、负数的平方根等
3. **变量引用**：使用变量引用可以更方便地进行多步骤计算
4. **精度考虑**：浮点数计算可能有小的精度误差，特别是处理小数位多的情况
5. **组合函数**：组合使用多个Math函数完成复杂计算

## 示例程序

这里有一个使用Math模块的示例程序，展示了各种数学函数的用法：

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"echo": ["===== Math模块功能演示 =====", "\n"]},
        
        {"var": {"a": 10, "b": 5}},
        {"echo": ["基础运算:", "\n"]},
        {"math.add": ["@var.a", "@var.b"], "output": "sum"},
        {"echo": ["  ", "@var.a", " + ", "@var.b", " = ", "@var.sum", "\n"]},
        
        {"math.subtract": ["@var.a", "@var.b"], "output": "diff"},
        {"echo": ["  ", "@var.a", " - ", "@var.b", " = ", "@var.diff", "\n"]},
        
        {"math.multiply": ["@var.a", "@var.b"], "output": "product"},
        {"echo": ["  ", "@var.a", " * ", "@var.b", " = ", "@var.product", "\n"]},
        
        {"math.divide": ["@var.a", "@var.b"], "output": "quotient"},
        {"echo": ["  ", "@var.a", " / ", "@var.b", " = ", "@var.quotient", "\n"]},
        
        {"echo": ["\n高级函数:", "\n"]},
        {"math.pow": ["@var.a", 2], "output": "power"},
        {"echo": ["  ", "@var.a", "的平方 = ", "@var.power", "\n"]},
        
        {"math.sqrt": ["@var.power"], "output": "sqrt_result"},
        {"echo": ["  ", "@var.power", "的平方根 = ", "@var.sqrt_result", "\n"]},
        
        {"var": {"numbers": [3, 8, 1, 6, 2]}},
        {"math.max": "@var.numbers", "output": "max_value"},
        {"math.min": "@var.numbers", "output": "min_value"},
        {"echo": ["  数组最大值: ", "@var.max_value", ", 最小值: ", "@var.min_value", "\n"]},
        
        {"var": {"angle": 45}},
        {"math.sin": "@var.angle", "output": "sin_value"},
        {"echo": ["  ", "@var.angle", "度的正弦值 = ", "@var.sin_value", "\n"]},
        
        {"math.random": [1, 100], "output": "random_value"},
        {"echo": ["  生成1到100的随机数: ", "@var.random_value", "\n"]},
        
        {"echo": ["===== 演示结束 =====", "\n"]}
      ]
    }
  }
}
``` 