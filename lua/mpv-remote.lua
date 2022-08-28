local msg = require("mp.msg")
local utils = require("mp.utils")
local socket = mp.get_property("input-ipc-server")

if socket == nil or socket == "" then
  socket = mp.get_property_native('options/vo-mmcss-profile', o) ~= o
    and "\\\\.\\pipe\\mpvsocket"
    or "/tmp/mpvsocket"
  mp.set_property("input-ipc-server", socket)
end

local address = "0.0.0.0"
local port = "8000"

local server = mp.command_native_async({
  name = "subprocess",
  playback_only = false,
  capture_stdout = true,
  args = { "mpv-remote-server", socket, address, port },
})

mp.register_event('shutdown', function() mp.abort_async_command(server) end)
