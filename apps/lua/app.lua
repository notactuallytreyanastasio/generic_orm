#!/usr/bin/env lua
-- app.lua - Todo list web application with built-in HTTP server
-- Run with: lua app.lua
-- Server: http://localhost:5005

local socket = require("socket")

-- Resolve the script directory for reliable requires
local script_dir = debug.getinfo(1, "S").source:match("^@(.*/)") or "./"
package.path = script_dir .. "vendor/orm/?.lua;"
    .. script_dir .. "vendor/orm/?/init.lua;"
    .. script_dir .. "vendor/temper-core/?.lua;"
    .. script_dir .. "vendor/temper-core/?/init.lua;"
    .. script_dir .. "vendor/std/?.lua;"
    .. script_dir .. "vendor/std/?/init.lua;"
    .. script_dir .. "?.lua;"
    .. package.path

local db = require("db")
local templates = require("templates")

local PORT = tonumber(os.getenv("PORT")) or 5005
local STATIC_DIR = script_dir .. "static/"

-- ============================================================
-- HTTP Helpers
-- ============================================================

-- URL-decode a percent-encoded string
local function url_decode(s)
    s = s:gsub("+", " ")
    s = s:gsub("%%(%x%x)", function(h)
        return string.char(tonumber(h, 16))
    end)
    return s
end

-- Parse form-urlencoded body
local function parse_form(body)
    local params = {}
    if not body or body == "" then return params end
    for pair in body:gmatch("[^&]+") do
        local key, val = pair:match("^([^=]+)=?(.*)")
        if key then
            params[url_decode(key)] = url_decode(val or "")
        end
    end
    return params
end

-- Parse an HTTP request from a client socket
local function parse_request(client)
    -- Read request line
    local line, err = client:receive("*l")
    if not line then return nil, err end

    local method, path, version = line:match("^(%S+)%s+(%S+)%s+(%S+)")
    if not method then return nil, "malformed request line" end

    -- Read headers
    local headers = {}
    while true do
        local hline, herr = client:receive("*l")
        if not hline or hline == "" then break end
        local name, value = hline:match("^([^:]+):%s*(.*)")
        if name then
            headers[name:lower()] = value
        end
    end

    -- Read body if Content-Length present
    local body = ""
    local content_length = tonumber(headers["content-length"])
    if content_length and content_length > 0 then
        body = client:receive(content_length) or ""
    end

    return {
        method = method,
        path = path,
        version = version,
        headers = headers,
        body = body,
    }
end

-- Send an HTTP response
local function send_response(client, status_code, status_text, headers, body)
    local parts = {}
    table.insert(parts, string.format("HTTP/1.1 %d %s\r\n", status_code, status_text))
    for k, v in pairs(headers) do
        table.insert(parts, string.format("%s: %s\r\n", k, v))
    end
    if not headers["Content-Length"] then
        table.insert(parts, string.format("Content-Length: %d\r\n", #body))
    end
    table.insert(parts, "Connection: close\r\n")
    table.insert(parts, "\r\n")
    table.insert(parts, body)
    client:send(table.concat(parts))
end

-- Send HTML response
local function html_response(client, html, code, status)
    code = code or 200
    status = status or "OK"
    send_response(client, code, status, {
        ["Content-Type"] = "text/html; charset=utf-8",
    }, html)
end

-- Send redirect response
local function redirect(client, location)
    send_response(client, 303, "See Other", {
        ["Location"] = location,
        ["Content-Type"] = "text/html",
    }, '<html><body>Redirecting to <a href="' .. location .. '">' .. location .. '</a></body></html>')
end

-- Guess MIME type from file extension
local function mime_type(path)
    local ext = path:match("%.(%w+)$")
    local types = {
        css  = "text/css",
        js   = "application/javascript",
        html = "text/html",
        png  = "image/png",
        jpg  = "image/jpeg",
        gif  = "image/gif",
        ico  = "image/x-icon",
        svg  = "image/svg+xml",
        txt  = "text/plain",
    }
    return types[ext] or "application/octet-stream"
end

-- Serve a static file
local function serve_static(client, filepath)
    -- Sanitize path to prevent directory traversal
    if filepath:find("%.%.", 1, true) then
        send_response(client, 403, "Forbidden", { ["Content-Type"] = "text/plain" }, "Forbidden")
        return
    end

    local full_path = STATIC_DIR .. filepath
    local f = io.open(full_path, "rb")
    if not f then
        send_response(client, 404, "Not Found", { ["Content-Type"] = "text/plain" }, "File not found")
        return
    end
    local content = f:read("*a")
    f:close()
    send_response(client, 200, "OK", {
        ["Content-Type"] = mime_type(filepath),
        ["Cache-Control"] = "public, max-age=3600",
    }, content)
end

-- ============================================================
-- Route Handlers
-- ============================================================

-- GET / - show all lists
local function handle_lists_index(client)
    local lists = db.lists_get_all()
    html_response(client, templates.lists_page(lists))
end

-- POST /lists - create a new list
local function handle_lists_create(client, req)
    local params = parse_form(req.body)
    local name = params.name
    if name and name ~= "" then
        db.lists_create(name)
    end
    redirect(client, "/")
end

-- POST /lists/{id}/delete - delete a list
local function handle_lists_delete(client, _, list_id)
    db.lists_delete(list_id)
    redirect(client, "/")
end

-- GET /lists/{id} - show todos for a list
local function handle_list_show(client, _, list_id)
    local list = db.lists_get_by_id(list_id)
    if not list then
        html_response(client, templates.base("Not Found",
            '<div class="empty-state">List not found.</div><a href="/" class="back-link">&laquo; Back</a>'),
            404, "Not Found")
        return
    end
    local todos = db.todos_get_by_list(list_id)
    html_response(client, templates.list_page(list, todos))
end

-- POST /lists/{id}/todos - add a todo to a list
local function handle_todos_create(client, req, list_id)
    local params = parse_form(req.body)
    local title = params.title
    if title and title ~= "" then
        db.todos_create(title, list_id)
    end
    redirect(client, "/lists/" .. list_id)
end

-- POST /todos/{id}/toggle - toggle a todo
local function handle_todos_toggle(client, _, todo_id)
    local list_id = db.todos_get_list_id(todo_id)
    db.todos_toggle(todo_id)
    if list_id then
        redirect(client, "/lists/" .. list_id)
    else
        redirect(client, "/")
    end
end

-- POST /todos/{id}/delete - delete a todo
local function handle_todos_delete(client, _, todo_id)
    local list_id = db.todos_get_list_id(todo_id)
    db.todos_delete(todo_id)
    if list_id then
        redirect(client, "/lists/" .. list_id)
    else
        redirect(client, "/")
    end
end

-- ============================================================
-- Router
-- ============================================================

local function route(client, req)
    local method = req.method
    local path = req.path

    -- Static files
    local static_path = path:match("^/static/(.+)$")
    if static_path then
        serve_static(client, static_path)
        return
    end

    -- GET / - lists index
    if method == "GET" and path == "/" then
        handle_lists_index(client)
        return
    end

    -- POST /lists - create list
    if method == "POST" and path == "/lists" then
        handle_lists_create(client, req)
        return
    end

    -- POST /lists/{id}/delete
    local list_id_del = path:match("^/lists/(%d+)/delete$")
    if method == "POST" and list_id_del then
        handle_lists_delete(client, req, tonumber(list_id_del))
        return
    end

    -- GET /lists/{id}
    local list_id_show = path:match("^/lists/(%d+)$")
    if method == "GET" and list_id_show then
        handle_list_show(client, req, tonumber(list_id_show))
        return
    end

    -- POST /lists/{id}/todos
    local list_id_add = path:match("^/lists/(%d+)/todos$")
    if method == "POST" and list_id_add then
        handle_todos_create(client, req, tonumber(list_id_add))
        return
    end

    -- POST /todos/{id}/toggle
    local todo_id_toggle = path:match("^/todos/(%d+)/toggle$")
    if method == "POST" and todo_id_toggle then
        handle_todos_toggle(client, req, tonumber(todo_id_toggle))
        return
    end

    -- POST /todos/{id}/delete
    local todo_id_del = path:match("^/todos/(%d+)/delete$")
    if method == "POST" and todo_id_del then
        handle_todos_delete(client, req, tonumber(todo_id_del))
        return
    end

    -- Favicon
    if path == "/favicon.ico" then
        send_response(client, 204, "No Content", {}, "")
        return
    end

    -- 404
    html_response(client, templates.base("404 Not Found",
        '<div class="empty-state">Page not found.<br><a href="/" class="back-link">&laquo; Home</a></div>'),
        404, "Not Found")
end

-- ============================================================
-- Server
-- ============================================================

local function main()
    -- Initialize database
    db.open()
    db.migrate()
    local seeded = db.seed()

    -- Create TCP server
    local server, err = socket.bind("0.0.0.0", PORT)
    if not server then
        io.stderr:write("Error: Could not bind to port " .. PORT .. ": " .. (err or "unknown error") .. "\n")
        io.stderr:write("Is another process using port " .. PORT .. "?\n")
        os.exit(1)
    end

    server:settimeout(1) -- 1 second timeout for accept (allows Ctrl+C)

    print("============================================")
    print("  Lua Todo App (generic ORM)")
    print("  http://localhost:" .. PORT)
    print("============================================")
    if seeded then
        print("  Seeded 2 sample lists with todos")
    end
    print("  Press Ctrl+C to stop")
    print("")

    -- Main accept loop
    while true do
        local client, accept_err = server:accept()
        if client then
            client:settimeout(5)
            local ok, pcall_err = pcall(function()
                local req, parse_err = parse_request(client)
                if req then
                    local timestamp = os.date("%Y-%m-%d %H:%M:%S")
                    print(string.format("[%s] %s %s", timestamp, req.method, req.path))
                    route(client, req)
                else
                    if parse_err ~= "closed" and parse_err ~= "timeout" then
                        send_response(client, 400, "Bad Request",
                            { ["Content-Type"] = "text/plain" }, "Bad request: " .. (parse_err or ""))
                    end
                end
            end)
            if not ok then
                io.stderr:write("Error handling request: " .. tostring(pcall_err) .. "\n")
                pcall(function()
                    send_response(client, 500, "Internal Server Error",
                        { ["Content-Type"] = "text/plain" }, "Internal server error")
                end)
            end
            client:close()
        end
    end
end

-- Run
local ok, err = pcall(main)
if not ok then
    io.stderr:write("Fatal: " .. tostring(err) .. "\n")
    db.close()
    os.exit(1)
end
