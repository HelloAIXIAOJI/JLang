# 嵌套属性访问

在JiLang中，您可以使用点号(`.`)和方括号(`[]`)语法访问嵌套对象的属性和数组元素。本文档介绍这些功能的使用方法和示例。

## 对象属性访问

使用点号(`.`)可以访问对象的属性。嵌套层级可以任意深度：

```json
{
  "var": {"user": {"profile": {"name": "张三"}}},
  "echo": ["用户名: ", "@var.user.profile.name", "\n"]
}
```

## 数组索引访问

使用方括号(`[]`)可以通过索引访问数组元素。JiLang中数组索引从0开始：

```json
{
  "var": {"numbers": [10, 20, 30, 40, 50]},
  "echo": ["第三个数字: ", "@var.numbers[2]", "\n"]
}
```

## 混合使用点号和方括号

可以混合使用点号和方括号来访问复杂的嵌套数据结构：

```json
{
  "var": {"users": [
    {"name": "张三", "skills": ["编程", "设计"]},
    {"name": "李四", "skills": ["写作", "绘画"]}
  ]},
  "echo": ["第一个用户的第二个技能: ", "@var.users[0].skills[1]", "\n"]
}
```

## 复杂示例

下面是一个综合示例，展示了各种嵌套属性访问方法：

```json
{
  "program": {
    "main": {
      "body": [
        {"comment": "创建复杂的嵌套结构"},
        {"var": {"company": {
          "name": "示例公司",
          "departments": [
            {
              "name": "研发部",
              "employees": [
                {"name": "张三", "position": "开发工程师"},
                {"name": "李四", "position": "设计师"}
              ]
            },
            {
              "name": "市场部",
              "employees": [
                {"name": "王五", "position": "市场经理"}
              ]
            }
          ]
        }}},
        
        {"comment": "访问基本属性"},
        {"echo": ["公司名称: ", "@var.company.name", "\n"]},
        
        {"comment": "访问数组元素"},
        {"echo": ["第一个部门: ", "@var.company.departments[0].name", "\n"]},
        
        {"comment": "访问嵌套数组元素"},
        {"echo": ["研发部第二位员工: ", "@var.company.departments[0].employees[1].name", "\n"]},
        {"echo": ["该员工职位: ", "@var.company.departments[0].employees[1].position", "\n"]},
        
        {"comment": "跨多层访问"},
        {"echo": ["市场部第一位员工: ", "@var.company.departments[1].employees[0].name", "\n"]}
      ]
    }
  }
}
```

## 注意事项

使用嵌套属性访问时，需要注意以下几点：

1. **路径有效性**：确保访问路径中的每个部分都存在，否则会导致错误
2. **数组索引范围**：确保数组索引在有效范围内（0到数组长度减1）
3. **类型匹配**：确保对象属性用点号访问，数组元素用方括号访问

## 错误处理

如果嵌套属性路径无效，通常会返回`null`或引发错误，具体取决于使用的上下文。您可以使用条件语句来检查路径的有效性：

```json
{
  "if": {
    "condition": {
      "op": "neq",
      "left": "@var.user.profile",
      "right": "null"
    },
    "then": [
      {"echo": ["用户资料存在: ", "@var.user.profile.name", "\n"]}
    ],
    "else": [
      {"echo": ["用户资料不存在\n"]}
    ]
  }
}
```

## 相关链接

- [变量引用基础](variable-references-basics.zh.md) - 了解JiLang中变量引用的基本概念
- [变量引用高级功能](advanced-variable-references.zh.md) - 学习变量引用的高级特性和错误处理 