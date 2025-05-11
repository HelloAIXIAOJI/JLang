# JiLang

*基于JSON语法的图灵完备编程语言*

[![许可证: MIT](https://img.shields.io/badge/许可证-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![语言: Rust](https://img.shields.io/badge/实现语言-Rust-orange.svg)
![状态: 实验性](https://img.shields.io/badge/状态-实验性-blue.svg)

## JiLang是什么？

JiLang（原名JsonLang，即"Ji's Language"）是一种实验性编程语言，使用JSON作为其语法基础。它允许开发者使用熟悉的JSON结构编写程序，同时提供变量、循环、条件、函数等完整（或者较为完整）的编程语言功能。

JiLang的核心理念是将结构化数据格式（JSON）转变为一种功能完整的编程语言，使编程更加直观和一致。

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

## 主要特性

- **基于JSON的语法**：使用熟悉的JSON格式编写程序
- **变量和常量**：使用`@var.`和`@const.`前缀引用变量和常量
- **控制结构**：支持if-else语句、while循环、for循环
- **函数**：定义和调用自定义函数，包括递归函数支持
- **模块系统**：导入和使用内置或自定义模块
- **弱类型系统**：灵活的类型转换，类似于PHP
- **数据结构操作**：轻松处理数组和对象
- **系统交互**：从JiLang代码中运行外部命令

## 入门指南

### 安装JiLang

#### 预编译二进制文件

从[发布页面](https://github.com/HelloAIXIAOJI/JiLang/releases)下载最新版本。

#### 从源代码构建

确保已安装Rust，然后执行：

```bash
git clone https://github.com/HelloAIXIAOJI/JiLang.git
cd JiLang
cargo build --release
```

编译后的可执行文件将在`target/release/JiLang`中可用。

### 第一个JiLang程序

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

## 文档导航

JiLang的文档分为以下几个部分：

- [基本语法](syntax/basic.md) - JiLang语法的基本概念和规则
- [引用符号](syntax/reference_symbols.md) - 变量引用符号(@、$、￥等)的使用方法
- [变量与类型](syntax/variables.md) - 变量声明、类型系统和作用域
- [控制结构](syntax/control_flow.md) - 条件语句和循环结构
- [函数与模块](syntax/functions.md) - 函数定义、调用和模块系统
- [内置模块](modules/index.md) - JiLang提供的标准模块
- [示例程序](examples.md) - 各种功能的示例程序

## 开发JiLang的应用场景

JiLang适用于以下场景：

- **教育目的**：学习编程语言实现和解释器设计
- **配置脚本**：将静态JSON配置升级为动态脚本
- **嵌入式脚本**：为应用程序提供基于JSON的脚本引擎
- **实验项目**：探索编程语言设计的新概念

## 命令行选项

JiLang支持以下命令行选项：

```bash
# 显示帮助信息
JiLang --help

# 显示关于信息
JiLang --about

# 以调试模式运行（显示详细执行信息）
JiLang --debug my_program.jl

# 以容错模式运行（报告非关键错误但不终止）
JiLang --ignore-non-critical-errors my_program.jl

# 检查错误但不执行程序
JiLang --check my_program.jl
```

## 项目状态

JiLang是一个实验性项目，不建议在生产环境中使用。虽然它在技术上是图灵完备的，但它主要是作为一个编程语言实验和学习工具开发的。欢迎探索、学习和贡献，但请为严肃的项目使用成熟的语言。

## 参与贡献

我们欢迎各种形式的贡献：

- 提交bug报告和功能请求
- 改进文档和示例
- 增强和扩展核心解释器
- 开发新的扩展模块

## 许可证

JiLang采用MIT许可证。详情见[LICENSE](../license)文件。 