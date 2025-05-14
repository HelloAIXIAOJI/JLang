{
  "program": {
    "main": {
      "params": {},
      "body": [
        {
          "comment": "演示在控制流语句中使用返回值"
        },
        {
          "comment": "1. If语句的返回值"
        },
        {
          "var": {
            "x": 10
          }
        },
        {
          "var": {
            "ifResult": {
              "if": {
                "condition": {
                  "math.gt": ["$x", 5]
                },
                "then": [
                  {
                    "object.create": {
                      "status": "success",
                      "message": "x大于5"
                    }
                  }
                ],
                "else": [
                  {
                    "object.create": {
                      "status": "failure",
                      "message": "x不大于5"
                    }
                  }
                ]
              }
            }
          }
        },
        {
          "echo": ["if语句返回值: ", "$ifResult", "\n"]
        },
        {
          "comment": "2. While循环的返回值"
        },
        {
          "var": {
            "counter": 0
          }
        },
        {
          "var": {
            "whileResult": {
              "while": {
                "condition": {
                  "math.lt": ["$counter", 5]
                },
                "body": [
                  {
                    "var": {
                      "counter": {
                        "math.add": ["$counter", 1]
                      }
                    }
                  },
                  {
                    "object.create": {
                      "iteration": "$counter",
                      "message": "循环执行中"
                    }
                  }
                ]
              }
            }
          }
        },
        {
          "echo": ["while循环最后一次迭代的返回值: ", "$whileResult", "\n"]
        },
        {
          "comment": "3. For循环的返回值"
        },
        {
          "var": {
            "forResult": {
              "for": {
                "init": {
                  "var": {
                    "i": 1
                  }
                },
                "condition": {
                  "math.lte": ["$i", 5]
                },
                "update": {
                  "var": {
                    "i": {
                      "math.add": ["$i", 1]
                    }
                  }
                },
                "body": [
                  {
                    "var": {
                      "square": {
                        "math.pow": ["$i", 2]
                      }
                    }
                  },
                  {
                    "object.create": {
                      "number": "$i",
                      "squared": "$square"
                    }
                  }
                ]
              }
            }
          }
        },
        {
          "echo": ["for循环最后一次迭代的返回值: ", "$forResult", "\n"]
        },
        {
          "comment": "4. Switch语句的返回值"
        },
        {
          "var": {
            "grade": "B"
          }
        },
        {
          "var": {
            "switchResult": {
              "switch": {
                "expression": "$grade",
                "cases": {
                  "A": [
                    {
                      "object.create": {
                        "score": 90,
                        "description": "优秀"
                      }
                    }
                  ],
                  "B": [
                    {
                      "object.create": {
                        "score": 80,
                        "description": "良好"
                      }
                    }
                  ],
                  "C": [
                    {
                      "object.create": {
                        "score": 70,
                        "description": "中等"
                      }
                    }
                  ]
                },
                "default": [
                  {
                    "object.create": {
                      "score": 60,
                      "description": "及格"
                    }
                  }
                ]
              }
            }
          }
        },
        {
          "echo": ["switch语句返回值: ", "$switchResult", "\n"]
        },
        {
          "comment": "5. Try-Catch语句的返回值"
        },
        {
          "var": {
            "willFail": false
          }
        },
        {
          "var": {
            "tryResult": {
              "try": {
                "body": [
                  {
                    "if": {
                      "condition": "$willFail",
                      "then": [
                        {
                          "throw": "发生错误"
                        }
                      ],
                      "else": [
                        {
                          "object.create": {
                            "status": "success",
                            "message": "执行成功"
                          }
                        }
                      ]
                    }
                  }
                ],
                "catch": [
                  {
                    "object.create": {
                      "status": "error",
                      "message": "捕获到异常"
                    }
                  }
                ],
                "finally": [
                  {
                    "echo": ["try-catch执行完成\n"]
                  }
                ]
              }
            }
          }
        },
        {
          "echo": ["try-catch语句返回值: ", "$tryResult", "\n"]
        },
        {
          "comment": "6. 复合控制流的返回值"
        },
        {
          "var": {
            "input": 15
          }
        },
        {
          "var": {
            "complexResult": {
              "if": {
                "condition": {
                  "math.gt": ["$input", 10]
                },
                "then": [
                  {
                    "var": {
                      "temp": {
                        "math.multiply": ["$input", 2]
                      }
                    }
                  },
                  {
                    "while": {
                      "condition": {
                        "math.gt": ["$temp", 20]
                      },
                      "body": [
                        {
                          "var": {
                            "temp": {
                              "math.subtract": ["$temp", 5]
                            }
                          }
                        },
                        {
                          "object.create": {
                            "currentValue": "$temp",
                            "operation": "减5"
                          }
                        }
                      ]
                    }
                  }
                ],
                "else": [
                  {
                    "var": {
                      "temp": {
                        "math.add": ["$input", 5]
                      }
                    }
                  },
                  {
                    "object.create": {
                      "result": "$temp",
                      "operation": "加5"
                    }
                  }
                ]
              }
            }
          }
        },
        {
          "echo": ["复合控制流返回值: ", "$complexResult", "\n"]
        }
      ]
    }
  }
} 