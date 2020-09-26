local luv = require('luv')
local inspect = require('inspect')

local function req()
  local client = luv.new_tcp()
  client:connect("127.0.0.1", 7542, function()
    client:read_start(function (_, chunk)
      if chunk then
        print(chunk)
      else
        print('closing...')
      end
    end)

    print(inspect(client))
    client:write('{"jsonrpc":"2.0","method":"say_hello","id":"1"}')
    client:shutdown()

  end)

  luv.run()
end

req()
