{
  "program": {
    "main": {
      "params": {},
      "body": [
        {
          "comment": "测试函数返回值机制，直接使用返回值而不依赖result变量"
        },
        {
          "comment": "1. 数组操作返回值"
        },
        {
          "var": {
            "arr": {
              "array.create": [1, 2, 3, 4, 5]
            }
          }
        },
        {
          "echo": ["创建的数组: ", "$arr", "\n"]
        },
        {
          "var": {
            "length": {
              "array.length": ["$arr"]
            }
          }
        },
        {
          "echo": ["数组长度: ", "$length", "\n"]
        },
        {
          "var": {
            "updatedArr": {
              "array.push": ["$arr", 6]
            }
          }
        },
        {
          "echo": ["添加元素后的数组: ", "$updatedArr", "\n"]
        },
        {
          "var": {
            "sliced": {
              "array.slice": ["$updatedArr", 1, 4]
            }
          }
        },
        {
          "echo": ["切片后的数组 (index 1-4): ", "$sliced", "\n"]
        },
        {
          "var": {
            "item": {
              "array.get": ["$updatedArr", 2]
            }
          }
        },
        {
          "echo": ["数组第3个元素: ", "$item", "\n"]
        },
        {
          "comment": "2. 对象操作返回值"
        },
        {
          "var": {
            "person": {
              "object.create": {
                "name": "李四",
                "age": 28,
                "city": "上海"
              }
            }
          }
        },
        {
          "echo": ["创建的对象: ", "$person", "\n"]
        },
        {
          "var": {
            "updatedPerson": {
              "object.set": ["$person", "job", "工程师"]
            }
          }
        },
        {
          "echo": ["添加属性后的对象: ", "$updatedPerson", "\n"]
        },
        {
          "var": {
            "name": {
              "object.get": ["$updatedPerson", "name"]
            }
          }
        },
        {
          "echo": ["对象的name属性: ", "$name", "\n"]
        },
        {
          "var": {
            "hasJob": {
              "object.has": ["$updatedPerson", "job"]
            }
          }
        },
        {
          "echo": ["对象是否有job属性: ", "$hasJob", "\n"]
        },
        {
          "var": {
            "keys": {
              "object.keys": ["$updatedPerson"]
            }
          }
        },
        {
          "echo": ["对象的所有键: ", "$keys", "\n"]
        },
        {
          "comment": "3. 正则表达式操作返回值"
        },
        {
          "var": {
            "text": "联系电话: 13812345678，邮箱: example@test.com"
          }
        },
        {
          "var": {
            "matched": {
              "regex.match": ["$text", "\\d+"]
            }
          }
        },
        {
          "echo": ["匹配的电话号码: ", "$matched", "\n"]
        },
        {
          "var": {
            "hasEmail": {
              "regex.test": ["$text", "[a-z]+@[a-z]+\\.[a-z]+"]
            }
          }
        },
        {
          "echo": ["是否包含邮箱: ", "$hasEmail", "\n"]
        },
        {
          "var": {
            "masked": {
              "regex.replace": ["$text", "\\d{11}", "138****5678"]
            }
          }
        },
        {
          "echo": ["隐藏电话号码后的文本: ", "$masked", "\n"]
        },
        {
          "comment": "4. 数学计算返回值"
        },
        {
          "var": {
            "sum": {
              "math.add": [10, 20]
            }
          }
        },
        {
          "echo": ["10 + 20 = ", "$sum", "\n"]
        },
        {
          "var": {
            "product": {
              "math.multiply": ["$sum", 2]
            }
          }
        },
        {
          "echo": ["(10 + 20) * 2 = ", "$product", "\n"]
        },
        {
          "var": {
            "power": {
              "math.pow": [2, 8]
            }
          }
        },
        {
          "echo": ["2^8 = ", "$power", "\n"]
        },
        {
          "comment": "5. 嵌套表达式使用返回值"
        },
        {
          "var": {
            "personKeysLength": {
              "array.length": [
                {
                  "object.keys": ["$updatedPerson"]
                }
              ]
            }
          }
        },
        {
          "var": {
            "powerOf8": {
              "math.pow": [2, 3]
            }
          }
        },
        {
          "var": {
            "additionResult": {
              "math.add": ["$personKeysLength", "$powerOf8"]
            }
          }
        },
        {
          "echo": ["person对象的属性数量 + 2^3 = ", "$additionResult", "\n"]
        },
        {
          "var": {
            "complexResult": {
              "object.create": {
                "数组长度": {
                  "array.length": ["$updatedArr"]
                },
                "对象属性": {
                  "object.keys": ["$updatedPerson"]
                },
                "数学计算": {
                  "math.multiply": [
                    {
                      "array.length": ["$updatedArr"]
                    },
                    {
                      "array.length": [
                        {
                          "object.keys": ["$updatedPerson"]
                        }
                      ]
                    }
                  ]
                }
              }
            }
          }
        },
        {
          "echo": ["复杂嵌套结果: ", "$complexResult", "\n"]
        }
      ]
    }
  }
} 