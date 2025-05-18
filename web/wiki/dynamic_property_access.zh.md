# 动态属性访问 (get_property)

本文档介绍JiLang中的`get_property`语句，该语句用于灵活地访问对象和数组的属性与元素，特别适用于需要动态决定访问路径的场景。

## 基本用法

`get_property`语句的基本语法如下：

```json
{
  "get_property": {
    "object": "要访问的对象或数组",
    "path": "属性路径或属性路径数组",
    "output": "存储结果的变量名（可选）"
  }
}
```

### 参数说明

- `object`: 要访问的对象或数组，可以是直接值或变量引用
- `path`: 属性访问路径，可以是以下两种格式之一：
  - 字符串路径：用点号分隔的属性路径，如`"details.name"`
  - 数组路径：属性/索引列表，如`["details", "skills", 1]`
- `output`: 可选参数，指定存储结果的变量名。如果不提供，结果将存储在`result`变量中

## 示例

### 1. 基本属性访问

```json
// 假设有一个user对象
{"var": {
  "user": {
    "name": "张三",
    "age": 30
  }
}}

// 直接访问name属性
{"get_property": {
  "object": "@var.user",
  "path": "name",
  "output": "user_name"
}}

// 结果：user_name = "张三"
```

### 2. 嵌套属性访问

```json
// 假设有复杂的嵌套对象
{"var": {
  "company": {
    "name": "示例公司",
    "departments": [
      {
        "name": "研发部",
        "staff": 50
      }
    ]
  }
}}

// 使用字符串路径访问
{"get_property": {
  "object": "@var.company",
  "path": "departments.0.name",
  "output": "dept_name"
}}

// 使用数组路径访问（等效）
{"get_property": {
  "object": "@var.company",
  "path": ["departments", "0", "name"],
  "output": "dept_name"
}}

// 结果：dept_name = "研发部"
```

### 3. 使用变量动态访问属性

这是`get_property`最强大的用例：使用变量决定要访问的属性或索引。

```json
// 假设我们有一个包含多种属性的用户
{"var": {
  "user": {
    "name": "张三",
    "age": 30,
    "email": "zhangsan@example.com"
  }
}}

// 定义要访问的属性
{"var": {"property": "email"}}

// 动态访问
{"get_property": {
  "object": "@var.user",
  "path": ["@var.property"],
  "output": "user_property"
}}

// 结果：user_property = "zhangsan@example.com"
```

### 4. 访问数组

`get_property`也可以用于数组访问，包括获取数组长度：

```json
// 定义一个数组
{"var": {"numbers": [10, 20, 30, 40, 50]}}

// 获取特定索引的元素
{"get_property": {
  "object": "@var.numbers",
  "path": "2",
  "output": "third_element"
}}

// 结果：third_element = 30

// 获取数组长度
{"get_property": {
  "object": "@var.numbers",
  "path": "length",
  "output": "array_length"
}}

// 结果：array_length = 5
```

### 5. 使用变量作为数组索引

```json
{"var": {"skills": ["编程", "设计", "写作"]}}
{"var": {"index": 1}}

{"get_property": {
  "object": "@var.skills",
  "path": ["@var.index"],
  "output": "selected_skill"
}}

// 结果：selected_skill = "设计"
```

## 错误处理

`get_property`会在以下情况返回错误：

1. 提供的路径访问了不存在的属性
2. 尝试通过索引访问非数组值
3. 数组索引超出范围
4. 尝试访问非对象的属性

## 与直接变量引用的比较

虽然JiLang支持在变量引用中使用方括号直接访问数组元素（如`@var.array[0]`），但是这种方式不支持使用变量作为索引。`get_property`语句提供了更强大的动态访问能力，尤其适用于：

1. 需要使用变量决定访问哪个属性
2. 需要使用变量作为数组索引
3. 需要在运行时构建完整的访问路径

## 应用场景

- 动态表单处理：根据用户输入访问不同对象属性
- 数据转换：在不同结构的数据对象之间映射字段
- 配置驱动的应用：根据配置访问不同的数据结构
- API响应处理：从复杂的嵌套JSON响应中提取特定字段 