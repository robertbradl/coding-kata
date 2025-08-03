local function closebraces(brace)
    print("Called brace parser: ", brace)
    if brace == '[' then
        return ']'
    elseif brace == '(' then
        return ')'
    elseif brace == '{' then
        return '}'
    end
end

local function test(braces)
    local buffer = {}
    for i = 1, #braces do
        local c = braces:sub(i, i)
        print("Testing: ", c, "Buffer length:", #buffer)
        if buffer.length == 0 then
            table.insert(buffer, c)
        elseif c == closebraces(buffer[#buffer]) then
            table.remove(buffer, #buffer)
        else
            table.insert(buffer, c)
        end
    end
    print("Buffer: ", table.concat(buffer))
    if #buffer == 0 then
        return true
    else
        return false
    end
end

function Main()
    local string = "(){}[]"
    print('Testing with String: ', string)
    if test(string) == true then
        print("No formatting errors!")
    else
        print("Malformed brackets!")
    end
end

Main()
