{
  "program": {
    "simple_return": {
      "params": {
        "value": null
      },
      "body": [
        {
          "echo": "函数开始执行..."
        },
        {
          "if": {
            "condition": {"op": "eq", "left": "@var.value", "right": true},
            "then": [
              {
                "echo": "条件为真，准备返回"
              },
              {
                "return": {
                  "value": "提前返回的值"
                }
              }
            ]
          }
        },
        {
          "echo": "这部分代码不应该被执行"
        }
      ]
    },
    
    "conditional_return": {
      "params": {
        "num": null
      },
      "body": [
        {
          "if": {
            "condition": {"op": "lt", "left": "@var.num", "right": 0},
            "then": [
              {
                "return": {
                  "value": "负数"
                }
              }
            ],
            "else": [
              {
                "if": {
                  "condition": {"op": "eq", "left": "@var.num", "right": 0},
                  "then": [
                    {
                      "return": {
                        "value": "零"
                      }
                    }
                  ],
                  "else": [
                    {
                      "return": {
                        "value": "正数"
                      }
                    }
                  ]
                }
              }
            ]
          }
        }
      ]
    },
    
    "factorial": {
      "params": {
        "n": null
      },
      "body": [
        {
          "if": {
            "condition": {"op": "lte", "left": "@var.n", "right": 1},
            "then": [
              {
                "return": {
                  "value": 1
                }
              }
            ]
          }
        },
        {
          "var": {
            "recursiveResult": {
              "factorial": [{"op": "sub", "left": "@var.n", "right": 1}]
            }
          }
        },
        {
          "return": {
            "value": {"op": "mul", "left": "@var.n", "right": "@var.recursiveResult"}
          }
        }
      ]
    },
    
    "main": {
      "params": {},
      "body": [
        {
          "echo": "测试简单的提前返回："
        },
        {
          "var": {
            "result1": {
              "simple_return": [true]
            }
          }
        },
        {
          "echo": "@var.result1"
        },
        {
          "echo": "测试条件返回："
        },
        {
          "var": {
            "result2a": {
              "conditional_return": [-5]
            }
          }
        },
        {
          "echo": "@var.result2a"
        },
        {
          "var": {
            "result2b": {
              "conditional_return": [0]
            }
          }
        },
        {
          "echo": "@var.result2b"
        },
        {
          "var": {
            "result2c": {
              "conditional_return": [10]
            }
          }
        },
        {
          "echo": "@var.result2c"
        },
        {
          "echo": "测试递归阶乘："
        },
        {
          "var": {
            "result3": {
              "factorial": [5]
            }
          }
        },
        {
          "echo": "@var.result3"
        }
      ]
    }
  },
  
  "entry": "main"
} 