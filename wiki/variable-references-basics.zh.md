# 变量引用基础

JiLang中的变量引用系统允许您在程序中访问和操作已定义的变量、参数、常量和环境变量。本文档介绍JiLang变量引用的基本概念和使用方法。

## 引用类型

JiLang支持以下几种引用类型：

1. **变量引用** (`@var.`): 访问用户定义的变量
2. **参数引用** (`@params.`): 访问函数参数
3. **常量引用** (`@const.`): 访问常量
4. **环境变量引用** (`@env.`): 访问系统环境变量

## 基本语法

变量引用的基本语法是在引用前缀后跟变量名称：

```json
{
  "echo": ["变量值: ", "@var.变量名"]
}
```

## 变量引用前缀

除了标准的`@var.`前缀外，JiLang还支持以下简化的变量引用前缀：

1. **美元符号前缀** (`$`): 等同于`@var.`
2. **人民币符号前缀** (`￥`): 等同于`@var.`

这些简化前缀可以减少代码的复杂性，提高可读性。

### 示例

```json
{
  "program": {
    "main": {
      "body": [
        {"var": {"name": "张三"}},
        {"var": {"age": 25}},
        {"var": {"city": "北京"}},
        
        {"comment": "使用标准前缀"},
        {"echo": ["标准前缀: ", "@var.name", " 今年 ", "@var.age", " 岁\n"]},
        
        {"comment": "使用美元符号前缀"},
        {"echo": ["美元前缀: ", "$name", " 今年 ", "$age", " 岁\n"]},
        
        {"comment": "使用人民币符号前缀"},
        {"echo": ["人民币前缀: ", "￥name", " 今年 ", "￥age", " 岁\n"]}
      ]
    }
  }
}
```

## 使用常量

常量使用`@const.`前缀访问，常量通常在程序顶层的`const`部分定义：

```json
{
  "const": {
    "VERSION": "1.0",
    "APP_NAME": "测试应用"
  },
  "program": {
    "main": {
      "body": [
        {"echo": ["应用: ", "@const.APP_NAME", " 版本: ", "@const.VERSION", "\n"]}
      ]
    }
  }
}
```

## 函数参数引用

在函数内部，可以使用`@params.`前缀访问传递给函数的参数：

```json
{
  "program": {
    "main": {
      "body": [
        {"greet": ["世界"]}
      ]
    },
    "greet": {
      "params": ["name"],
      "body": [
        {"echo": ["你好, ", "@params.name", "!\n"]}
      ]
    }
  }
}
```

## 环境变量引用

使用`@env.`前缀可以访问系统环境变量：

```json
{
  "program": {
    "main": {
      "body": [
        {"echo": ["用户名: ", "@env.USERNAME", "\n"]},
        {"echo": ["系统路径: ", "@env.PATH", "\n"]}
      ]
    }
  }
}
```

## 调用结果引用

当调用模块函数时，结果会自动存储在`@var.result`变量中：

```json
{
  "include": ["math"],
  "program": {
    "main": {
      "body": [
        {"math.add": [5, 3]},
        {"echo": ["5 + 3 = ", "@var.result", "\n"]}
      ]
    }
  }
}
```

## 相关链接

- [嵌套属性访问](nested-property-access.zh.md) - 学习如何访问复杂对象中的嵌套属性
- [变量引用高级功能](advanced-variable-references.zh.md) - 了解变量引用的高级使用技巧和错误处理