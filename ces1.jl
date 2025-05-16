{
    "include": ["io"],
  "program": {
    "main": {
      "body": [
        {"echo": ["这是要写入文件的内容"], "output": "content"},
        {"io.write_file": ["output.txt", "@var.content"]}
      ]
    }
  }
}