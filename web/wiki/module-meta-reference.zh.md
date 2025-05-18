# 模块元数据与访问

本文档详细介绍了JiLang中的模块元数据系统，包括如何在代码中访问模块元数据和使用命令行查看模块元数据。

## 模块元数据概述

在JiLang中，每个模块都可以包含元数据(metadata)，用于描述模块的基本信息，如版本、描述和作者等。这些元数据对于模块的文档和管理非常有用。

模块元数据可以通过两种方式访问：
1. 在JiLang程序中通过`@var.module_meta`变量
2. 在命令行中使用`--modulemeta`选项

## @var.module_meta变量

### 变量介绍

`@var.module_meta`是JiLang解释器自动创建的全局变量，包含所有已加载模块的元数据。该变量是一个对象，其中每个属性对应一个已加载模块的元数据。

### 访问模式

访问模块元数据的基本模式是：

```json
"@var.module_meta.模块名.属性名"
```

例如：
```json
{
  "echo": ["math模块版本：", "@var.module_meta.math.version"]
}
```

### 可用属性

对于标准模块和用户定义的模块，通常可以访问以下属性：

| 属性名 | 描述 |
|-------|------|
| version | 模块版本号 |
| description | 模块描述 |
| author | 模块作者 |

自定义模块可能包含额外的元数据属性。

### 使用示例

```json
{
  "include": ["math", "io", "utils"],
  "program": {
    "main": {
      "body": [
        {"echo": ["模块元数据信息：\n"]},
        {"echo": ["Math模块：\n"]},
        {"echo": ["  版本: ", "@var.module_meta.math.version", "\n"]},
        {"echo": ["  描述: ", "@var.module_meta.math.description", "\n"]},
        {"echo": ["  作者: ", "@var.module_meta.math.author", "\n"]},
        
        {"echo": ["\nIO模块：\n"]},
        {"echo": ["  版本: ", "@var.module_meta.io.version", "\n"]},
        {"echo": ["  描述: ", "@var.module_meta.io.description", "\n"]},
        
        {"echo": ["\n自定义Utils模块：\n"]},
        {"echo": ["  版本: ", "@var.module_meta.utils.version", "\n"]},
        {"echo": ["  描述: ", "@var.module_meta.utils.description", "\n"]},
        {"echo": ["  作者: ", "@var.module_meta.utils.author", "\n"]}
      ]
    }
  }
}
```

## --modulemeta命令行选项

### 选项介绍

`--modulemeta`是JiLang解释器的一个命令行选项，用于显示指定模块文件的元数据信息，无需执行整个程序。这对于检查模块信息和调试非常有用。

### 用法

```
jlang --modulemeta <模块文件路径>
```

例如：
```
jlang --modulemeta modules/utils.jl
```

### 输出内容

`--modulemeta`选项的输出包括：

1. **基本模块信息**：
   - 名称
   - 版本
   - 描述
   - 作者

2. **支持的函数列表**：
   - 函数名
   - 函数描述
   - 参数列表及类型
   - 返回值类型
   - 示例(如果有)

3. **自定义元数据**：
   - 如果模块定义了额外的元数据，也会显示出来

### 输出示例

```
正在加载模块: utils (路径: /path/to/modules/utils.jl)

=== 模块元数据 ===
名称: utils
版本: 1.0.0
描述: JiLang实用工具模块
作者: JiLang团队

=== 支持的函数 ===
* greet
  描述: 向用户打招呼
  参数:
    - name: 用户名 (string)
  返回值: Any

* count
  描述: 打印从start到end的数字
  参数:
    - start: 起始数字 (number)
    - end: 结束数字 (number)
  返回值: Any

=== 模块自定义元数据 ===
{
  "version": "1.0.0",
  "description": "JiLang实用工具模块",
  "author": "JiLang团队"
}
```

## 创建模块元数据

### 在JiLang (.jl) 模块中

在JiLang原生模块中，通过`module_meta`字段定义元数据：

```json
{
  "module_meta": {
    "version": "1.0.0",
    "description": "模块描述",
    "author": "作者名称"
  },
  "program": {
    // 函数定义...
  }
}
```

### 在Lua (.lua) 模块中

在Lua模块中，通过`module_meta`表定义元数据：

```lua
local module_meta = {
    version = "1.0.0",
    description = "模块描述",
    author = "作者名称"
}

-- 函数定义...

return {
    module_meta = module_meta,
    -- 导出函数...
}
```

## 技术实现细节

JiLang解释器在加载模块时，会自动收集所有模块的元数据，并将它们存储在一个全局上下文中。程序开始执行前，解释器会创建`module_meta`全局变量，并将收集到的元数据填充到该变量中。

对于命令行的`--modulemeta`选项，解释器会单独加载指定的模块文件，并提取其元数据和函数信息，然后以格式化的方式显示，而不会执行模块中的代码。

## 最佳实践

1. **始终为模块提供元数据**：增加可发现性和可维护性
2. **遵循版本号约定**：使用语义化版本号（例如`1.0.0`）
3. **提供详细描述**：清晰描述模块的功能和用途
4. **在使用前检查**：通过`--modulemeta`检查模块信息，确保与预期一致
5. **文档引用**：在模块文档中引用元数据，保持一致性

## 注意事项

1. 如果模块未定义元数据，`@var.module_meta`中仍会包含该模块，但可能只有最基本的信息
2. 模块元数据只对已加载（通过`include`指令）的模块可用
3. 在运行时修改`@var.module_meta`变量不会影响实际模块行为 