{
  "include": ["math"],
  "program": {
    "main": {
      "params": {},
      "body": [
        {
          "comment": "不同变量引用方式与函数返回值结合"
        },
        {
          "comment": "1. 测试不同的变量引用方式"
        },
        {
          "var": {
            "username": "张三",
            "age": 30,
            "city": "北京"
          }
        },
        {
          "var": {
            "data": {
              "object.create": {
                "name": "$username",
                "age": "$age",
                "location": "$city"
              }
            }
          }
        },
        {
          "echo": ["引用变量创建对象: ", "$data", "\n"]
        },
        {
            "echo": ["age: ", {
              "object.get": ["$data", "age"]
            }, "\n"]
          },
        {
          "comment": "2. 使用$语法创建和访问变量"
        },
        {
          "var": {
            "items": {
              "array.create": ["$username", 100, true, "$city"]
            }
          }
        },
        {
          "var": {
            "firstItem": {
              "array.get": ["$items", 0]
            }
          }
        },
        {
          "echo": ["数组第一项: ", "$firstItem", "\n"]
        },
        {
          "comment": "3. 使用中文环境变量名"
        },
        {
          "var": {
            "价格": 99.8
          }
        },
        {
          "var": {
            "商品信息": {
              "object.create": {
                "名称": "测试产品",
                "单价": "$价格",
                "数量": 2
              }
            }
          }
        },
        {
          "var": {
            "总价": {
              "math.multiply": ["$价格", {
                "object.get": ["$商品信息", "数量"]
              }]
            }
          }
        },
        {
          "echo": ["计算总价: ", "$总价", "\n"]
        },
        {
          "comment": "4. 使用常量引用"
        },
        {
          "var": {
            "constants": {
              "object.create": {
                "PI": 3.14159,
                "MAX_SIZE": 100
              }
            }
          }
        },
        {
          "var": {
            "半径": 5
          }
        },
        {
          "var": {
            "面积": {
              "math.multiply": [
                {
                  "object.get": ["$constants", "PI"]
                },
                {
                  "math.multiply": ["$半径", "$半径"]
                }
              ]
            }
          }
        },
        {
          "echo": ["计算圆面积: ", "$面积", "\n"]
        },
        {
          "comment": "5. 环境变量信息"
        },
        {
          "var": {
            "系统信息": {
              "object.create": {
                "系统类型": "Windows",
                "用户名": "User",
                "临时目录": "Temp"
              }
            }
          }
        },
        {
          "echo": ["环境变量信息: ", "$系统信息", "\n"]
        },
        {
          "comment": "6. 嵌套属性访问与数组索引"
        },
        {
          "var": {
            "user": {
              "object.create": {
                "name": "李四",
                "contacts": {
                  "email": "lisi@example.com",
                  "phone": "12345678901"
                },
                "skills": ["Java", "Python", "Rust"]
              }
            }
          }
        },
        {
          "var": {
            "邮箱": {
              "object.get": [
                {
                  "object.get": ["$user", "contacts"]
                },
                "email"
              ]
            },
            "编程语言": {
              "array.get": [
                {
                  "object.get": ["$user", "skills"]
                },
                1
              ]
            }
          }
        },
        {
          "echo": ["嵌套访问邮箱: ", "$邮箱", ", 第二个编程语言: ", "$编程语言", "\n"]
        },
        {
          "comment": "7. 组合多种引用方式与函数返回值"
        },
        {
          "var": {
            "结果": {
              "object.create": {
                "用户": "$username",
                "常量": {
                  "object.get": ["$constants", "MAX_SIZE"]
                },
                "计算结果": {
                  "math.add": [
                    {
                      "array.length": [
                        {
                          "object.get": ["$user", "skills"]
                        }
                      ]
                    },
                    "$age"
                  ]
                }
              }
            }
          }
        },
        {
          "echo": ["最终复杂组合结果: ", "$结果", "\n"]
        }
      ]
    }
  }
} 