-- math_lua.lua
local function fibonacci(n)
    -- 添加类型检查和转换
    if type(n) == "table" then
        n = tonumber(n[1]) or 0
    elseif type(n) ~= "number" then
        n = tonumber(n) or 0
    end
    
    if n <= 1 then
      return n
    else
      return fibonacci(n - 1) + fibonacci(n - 2)
    end
  end
  
  return {
    fibonacci = fibonacci
  }