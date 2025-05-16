# JiLang

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![Status: Experimental](https://img.shields.io/badge/Status-Experimental-blue.svg)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HelloAIXIAOJI/JiLang)

*基于JSON语法的编程语言 - 现已图灵完备！（理论）*

[English Documentation](README.md)

## JiLang是什么？

JiLang是一种实验性编程语言，使用JSON作为其语法。它允许您使用熟悉的JSON结构编写程序，同时提供变量、循环、条件、函数等编程功能。

```json
{
    "include": ["math"],
    "program": {
        "main": {
            "body": [
                {"echo": ["你好，JiLang！", "\n"]},
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
- **系统命令执行**：从JiLang代码中运行外部命令

## 安装

### 预编译二进制文件

从[发布页面](https://github.com/HelloAIXIAOJI/JiLang/releases)下载最新版本。

### 从源代码构建

确保已安装Rust，然后执行：

```bash
git clone https://github.com/HelloAIXIAOJI/JiLang.git
cd JiLang
cargo build --release
```

可执行文件将在`target/release/JiLang`中可用。

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
JiLang hello.jl
```

## 文档

有关JiLang语法和功能的全面指南，请参阅：

- [语法文档](docs/syntax.md)：详细的语法参考
- [示例](docs/examples_zh.md)：演示各种功能的示例程序

## 关键概念

- 程序被结构化为具有`program`属性的JSON对象
- `main`函数是执行的入口点
- 变量使用`@var.`前缀引用
- 模块函数使用`module_name.function_name`语法调用
- 语句表示为具有单个属性的JSON对象

## 命令行选项

JiLang支持以下命令行选项：

```bash
# 显示帮助信息
JiLang --help

# 显示关于信息
JiLang --about

# 显示创建者信息
JiLang --creator

# 以调试模式运行（显示详细执行信息）
JiLang --debug my_program.jl

# 以容错模式运行（报告非关键错误但不终止）
JiLang --ignore-non-critical-errors my_program.jl

# 检查错误但不执行程序
JiLang --check my_program.jl
```

## 状态

JiLang目前处于活跃开发阶段。随着0.4.0版本的重大改进，语言功能日趋完善，但仍不建议用于关键生产环境。它既是一个编程实验，也是一个不断成熟的工具。我们欢迎各类探索、学习和贡献，但对于企业级关键应用，请考虑使用更广泛验证的语言。

## 许可证

JiLang采用MIT许可证。详情见[LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交问题或拉取请求。

## 致谢

<<<<<<< HEAD
JiLang由AIXIAOJI在2025年劳动节假期作为一个有趣的编程实验创建。 
=======
JLang由AIXIAOJI在2025年劳动节假期作为一个有趣的编程实验创建。 
>>>>>>> 9a79a65a60a03386c95695a5c54b72161681d5ab
