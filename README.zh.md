# JLang

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![Status: Experimental](https://img.shields.io/badge/Status-Experimental-blue.svg)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HelloAIXIAOJI/JsonLang)

*基于JSON语法的编程语言 - 现已图灵完备！（理论）*

[English Documentation](README.md)

## JLang是什么？

JLang是一种实验性编程语言，使用JSON作为其语法。它允许您使用熟悉的JSON结构编写程序，同时提供变量、循环、条件、函数等编程功能。

```json
{
    "include": ["math"],
    "program": {
        "main": {
            "body": [
                {"echo": ["你好，JLang！", "\n"]},
                {"var": {"count": 0}},
                {"while": {
                    "condition": {
                        "op": "lt",
                        "left": "@var.count",
                        "right": 3
                    },
                    "body": [
                        {"echo": ["计数：", "@var.count", "\n"]},
                        {"math.add": ["@var.count", 1]},
                        {"var": {"count": "@var.result"}}
                    ]
                }}
            ]
        }
    }
}
```

## 特性

- **基于JSON的语法**：使用熟悉的JSON格式编写程序
- **变量和常量**：存储和操作数据
- **控制结构**：if-else语句、while循环、for循环
- **函数**：定义和调用自定义函数，包括递归函数
- **模块**：导入和使用内置或自定义模块
- **弱类型系统**：灵活的类型转换，类似于PHP
- **嵌套数据结构**：访问多层对象属性
- **系统命令执行**：从JLang代码中运行外部命令

## 安装

### 预编译二进制文件

从[发布页面](https://github.com/HelloAIXIAOJI/JsonLang/releases)下载最新版本。

### 从源代码构建

确保已安装Rust，然后执行：

```bash
git clone https://github.com/HelloAIXIAOJI/JsonLang.git
cd JsonLang
cargo build --release
```

可执行文件将在`target/release/jlang`中可用。

## 快速入门

1. 创建一个名为`hello.jl`的文件，内容如下：

```json
{
    "program": {
        "main": {
            "body": [
                {"echo": ["你好，世界！\n"]}
            ]
        }
    }
}
```

2. 运行它：

```bash
jlang hello.jl
```

## 文档

有关JLang语法和功能的全面指南，请参阅：

- [语法文档](docs/syntax.md)：详细的语法参考
- [示例](docs/examples_zh.md)：演示各种功能的示例程序

## 关键概念

- 程序被结构化为具有`program`属性的JSON对象
- `main`函数是执行的入口点
- 变量使用`@var.`前缀引用
- 模块函数使用`module_name.function_name`语法调用
- 语句表示为具有单个属性的JSON对象

## 版本历史

- **0.3.0**：添加对象操作和正则表达式支持，增强switch语句，改进错误处理，新增命令行选项
- **0.2.3**：增强弱类型系统，改进类型转换和比较
- **0.2.2**：添加系统命令执行功能
- **0.2.1**：添加递归函数支持和局部变量作用域
- **0.2.0**：添加注释系统、增强模块系统和多层数据访问
- **0.1.0**：具有基本功能的初始版本

## 命令行选项

JLang支持以下命令行选项：

```bash
# 显示帮助信息
jlang --help

# 显示关于信息
jlang --about

# 显示创建者信息
jlang --creator

# 以调试模式运行（显示详细执行信息）
jlang --debug my_program.jl

# 以容错模式运行（报告非关键错误但不终止）
jlang --ignore-non-critical-errors my_program.jl

# 检查错误但不执行程序
jlang --check my_program.jl
```

## 状态

JLang是实验性的，不适用于生产环境。虽然在技术上是图灵完备的，但它是作为一个编程实验创建的。欢迎探索、学习和贡献，但请为严肃的项目使用成熟的语言。

## 许可证

JLang采用MIT许可证。详情见[LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交问题或拉取请求。

## 致谢

JLang由AIXIAOJI在2025年劳动节假期作为一个有趣的编程实验创建。 
