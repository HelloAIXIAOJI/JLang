{
  "program": {
    "main": {
      "params": {},
      "body": [
        {
          "comment": "演示自定义输出变量与函数返回值结合使用"
        },
        {
          "comment": "1. 使用自定义输出变量"
        },
        {
          "array.create": {
            "size": 5,
            "init": 0,
            "output": "myArray"
          }
        },
        {
          "echo": ["创建的数组: ", "$myArray", "\n"]
        },
        {
          "comment": "2. 自定义输出创建对象"
        },
        {
          "object.create": {
            "name": "张三",
            "id": 12345,
            "output": "userData"
          }
        },
        {
          "echo": ["创建的用户数据: ", "$userData", "\n"]
        },
        {
          "comment": "3. 同时使用自定义输出和返回值"
        },
        {
          "var": {
            "numbers": {
              "array.create": [10, 20, 30, 40, 50]
            }
          }
        },
        {
          "var": {
            "sliced": {
              "array.slice": ["$numbers", 1, 4]
            }
          }
        },
        {
          "echo": ["自定义输出切片: ", "$sliced", "\n"]
        },
        {
          "var": {
            "directSlice": {
              "array.slice": ["$numbers", 0, 2]
            }
          }
        },
        {
          "echo": ["直接返回切片: ", "$directSlice", "\n"]
        },
        {
          "comment": "4. 模拟命令执行使用自定义输出"
        },
        {
          "var": {
            "cmdResult": "模拟命令执行结果"
          }
        },
        {
          "echo": ["命令执行结果: ", "$cmdResult", "\n"]
        },
        {
          "comment": "5. 正则表达式匹配使用自定义输出"
        },
        {
          "var": {
            "info": "用户ID: 12345, 年龄: 30"
          }
        },
        {
          "var": {
            "matchedId": {
              "regex.match": ["$info", "\\d+"]
            }
          }
        },
        {
          "echo": ["匹配到的ID: ", "$matchedId", "\n"]
        },
        {
          "comment": "6. 嵌套使用自定义输出和返回值"
        },
        {
          "var": {
            "innerArray": {
              "array.create": [1, 2, 3]
            }
          }
        },
        {
          "var": {
            "innerLength": {
              "array.length": ["$innerArray"]
            }
          }
        },
        {
          "echo": ["内部数组长度: ", "$innerLength", "\n"]
        }
      ]
    }
  }
} 