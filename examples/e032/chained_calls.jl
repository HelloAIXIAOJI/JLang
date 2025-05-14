{
  "program": {
    "main": {
      "params": {},
      "body": [
        {
          "comment": "演示链式调用和函数组合"
        },
        {
          "comment": "1. 创建数组、获取长度、计算长度的2倍"
        },
        {
          "var": {
            "doubledLength": {
              "math.multiply": [
                {
                  "array.length": [
                    {
                      "array.create": [1, 2, 3, 4, 5]
                    }
                  ]
                },
                2
              ]
            }
          }
        },
        {
          "echo": ["数组长度的2倍是: ", "$doubledLength", "\n"]
        },
        {
          "comment": "2. 创建对象、设置属性、获取属性"
        },
        {
          "var": {
            "updatedInfo": {
              "object.set": [
                {
                  "object.create": {
                    "name": "张三",
                    "age": 30
                  }
                },
                "job",
                "工程师"
              ]
            }
          }
        },
        {
          "echo": ["更新后的对象: ", "$updatedInfo", "\n"]
        },
        {
          "var": {
            "personName": {
              "object.get": ["$updatedInfo", "name"]
            }
          }
        },
        {
          "echo": ["获取的名字: ", "$personName", "\n"]
        },
        {
          "comment": "3. 创建数组、添加元素、获取元素"
        },
        {
          "var": {
            "arr": {
              "array.push": [
                {
                  "array.create": ["a", "b", "c"]
                },
                "d"
              ]
            }
          }
        },
        {
          "var": {
            "secondItem": {
              "array.get": ["$arr", 1]
            }
          }
        },
        {
          "echo": ["第二个元素: ", "$secondItem", ", 完整数组: ", "$arr", "\n"]
        },
        {
          "comment": "4. 嵌套函数调用 - 切片数组并获取元素"
        },
        {
          "var": {
            "element": {
              "array.get": [
                {
                  "array.slice": [
                    {
                      "array.create": [10, 20, 30, 40, 50]
                    },
                    1,
                    4
                  ]
                },
                1
              ]
            }
          }
        },
        {
          "echo": ["切片后获取的元素: ", "$element", "\n"]
        },
        {
          "comment": "5. 字符串处理 - 分割文本并操作结果"
        },
        {
          "var": {
            "text": "这是一段测试文本，用于演示函数调用"
          }
        },
        {
          "var": {
            "firstWord": {
              "array.get": [
                {
                  "regex.split": ["$text", "，"]
                },
                0
              ]
            }
          }
        },
        {
          "var": {
            "wordCount": {
              "array.length": [
                {
                  "regex.split": ["$text", " "]
                }
              ]
            }
          }
        },
        {
          "echo": ["第一部分文本: ", "$firstWord", ", 单词数量: ", "$wordCount", "\n"]
        }
      ]
    }
  }
} 