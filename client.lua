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

local function join_table(t)
  local str = "{"
  for k, v in pairs(t) do
    str = string.format('%s"%s":"%s",', str, k, v)
  end

  return str:sub(0, -2).."}"
end

req {
  method = 'setup',
  params = join_table { language = "rust" },
  on_rcv = function(data) print(data) end
}

req {
  method = 'setup',
  params = join_table { language = "lua" },
  on_rcv = function(data) print(data) end
}

req {
  method = "navigation/definition",
  params = join_table {
    file = "/home/kiyan/dev/other/treesitter-lsp/src/rpc/mod.rs",
    row = "8",
    column = "15",
  },
  on_rcv = function(data) print(data) end
}
