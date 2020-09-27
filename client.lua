local luv = require"luv"

local function req(opts)
  local client = luv.new_tcp()
  client:connect("127.0.0.1", 7542, function(err)
    if err then
      client:shutdown()
    end

    client:read_start(function(err, chunk)
      if not err and chunk then
        opts.on_rcv(chunk)
      else
        client:close()
      end
    end)

    local method = opts.method or ""
    local params = opts.params or "{}"
    local request = '{"jsonrpc":"2.0","method":"'..method..'","params":'..params..',"id":"1"}\n'
    client:write(request)
    client:shutdown()
  end)

  luv.run()
end

req {
  method = 'setup',
  params = '{ "language": "rust" }',
  on_rcv = function(data) print(data) end
}

req {
  method = 'setup',
  params = '{ "language": "lua" }',
  on_rcv = function(data) print(data) end
}

req {
  method = "navigation/definition",
  params = '{ "language": "rust", "node_name": "Project" }',
  on_rcv = function(data) print(data) end
}


