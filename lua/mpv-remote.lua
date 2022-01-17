local socket = mp.get_property("input-ipc-server")

if socket == nil or socket == "" then
  socket = mp.get_property_native('options/vo-mmcss-profile', o) ~= o
    and "\\\\.\\pipe\\mpvsocket"
    or "/tmp/mpvsocket"
  mp.set_property("input-ipc-server", socket)
end

local server = mp.command_native_async({
  name = "subprocess",
  playback_only = false,
  capture_stdout = true,
  args = { "mpv-remote-server", socket },
})

mp.register_event('shutdown', function() mp.abort_async_command(server) end)
