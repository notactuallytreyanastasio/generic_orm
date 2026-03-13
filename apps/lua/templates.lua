-- templates.lua - HTML template functions
local templates = {}

-- HTML-escape a string
local function esc(s)
    if not s then return "" end
    s = tostring(s)
    s = s:gsub("&", "&amp;")
    s = s:gsub("<", "&lt;")
    s = s:gsub(">", "&gt;")
    s = s:gsub('"', "&quot;")
    s = s:gsub("'", "&#39;")
    return s
end

-- Base template: Mac menubar + Win95 window chrome
function templates.base(title, content, status_text)
    status_text = status_text or "Ready"
    return [[<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>]] .. esc(title) .. [[ - TodoApp</title>
    <link rel="stylesheet" href="/static/retro.css">
    <style>
        .list-card {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 6px 8px;
            border-bottom: 1px solid #e0e0e0;
            cursor: default;
        }
        .list-card:last-child { border-bottom: none; }
        .list-card:hover {
            background: var(--win95-selection);
            color: var(--win95-selection-text);
        }
        .list-card:hover .list-meta { color: var(--win95-selection-text); }
        .list-card:hover .win95-btn.danger { color: #ff9999; }
        .list-card a {
            text-decoration: none;
            color: inherit;
            flex: 1;
        }
        .list-meta {
            font-size: 13px;
            color: var(--win95-btn-shadow);
        }
        .empty-state {
            padding: 24px;
            text-align: center;
            color: var(--win95-btn-shadow);
            font-size: 15px;
        }
        .back-link {
            display: inline-block;
            margin-bottom: 8px;
            color: var(--win95-title);
            text-decoration: none;
            font-size: 14px;
        }
        .back-link:hover { text-decoration: underline; }
        .progress-bar {
            width: 100%;
            height: 16px;
            border: 2px solid;
            border-color: var(--win95-btn-shadow) var(--win95-btn-highlight) var(--win95-btn-highlight) var(--win95-btn-shadow);
            box-shadow: inset 1px 1px 0 var(--win95-btn-dark-shadow);
            background: var(--win95-field-bg);
        }
        .progress-fill {
            height: 100%;
            background: var(--win95-title);
            transition: width 0.3s;
        }
    </style>
</head>
<body>
    <div class="mac-menubar">
        <span class="apple-logo">&#63743;</span>
        <span class="menu-item app-name">TodoApp</span>
        <span class="menu-item">File</span>
        <span class="menu-item">Edit</span>
        <span class="menu-item">Help</span>
        <span style="flex:1"></span>
        <span class="menu-item" style="font-weight:normal">Lua 5.4</span>
    </div>
    <div class="desktop">
        <div class="win95-window">
            <div class="win95-titlebar">
                <span class="title-text">]] .. esc(title) .. [[</span>
                <div class="win95-titlebar-buttons">
                    <button class="win95-titlebar-btn">_</button>
                    <button class="win95-titlebar-btn">&#9633;</button>
                    <button class="win95-titlebar-btn">X</button>
                </div>
            </div>
            <div class="win95-body">
                ]] .. content .. [[
            </div>
            <div class="win95-statusbar">
                <div class="status-section">]] .. esc(status_text) .. [[</div>
                <div class="status-section" style="flex:0;white-space:nowrap">
                    <span class="lang-badge">Lua</span>
                </div>
            </div>
        </div>
    </div>
</body>
</html>]]
end

-- Lists page: show all lists
function templates.lists_page(lists)
    local parts = {}

    -- New list form
    table.insert(parts, [[
        <div class="win95-toolbar">
            <form method="POST" action="/lists" style="display:flex;gap:4px;width:100%">
                <input type="text" name="name" class="win95-input" placeholder="New list name..." required>
                <button type="submit" class="win95-btn primary">New List</button>
            </form>
        </div>
    ]])

    -- List of lists
    table.insert(parts, '<div class="win95-sunken" style="min-height:200px;margin-top:8px">')

    if #lists == 0 then
        table.insert(parts, '<div class="empty-state">No lists yet. Create one above!</div>')
    else
        for _, list in ipairs(lists) do
            local pct = 0
            if list.total > 0 then
                pct = math.floor((list.done / list.total) * 100)
            end
            table.insert(parts, string.format([[
                <div class="list-card">
                    <a href="/lists/%d">
                        <div style="font-size:15px;font-weight:bold">%s</div>
                        <div class="list-meta">%d/%d tasks done (%d%%)</div>
                    </a>
                    <form method="POST" action="/lists/%d/delete" style="margin:0"
                          onsubmit="return confirm('Delete list &quot;%s&quot; and all its todos?')">
                        <button type="submit" class="win95-btn danger" title="Delete list">X</button>
                    </form>
                </div>
            ]], list.id, esc(list.name), list.done, list.total, pct,
                list.id, esc(list.name)))
        end
    end

    table.insert(parts, '</div>')

    local total_lists = #lists
    local total_todos = 0
    for _, l in ipairs(lists) do total_todos = total_todos + l.total end

    local status = string.format("%d list(s), %d total todo(s)", total_lists, total_todos)
    return templates.base("My Todo Lists", table.concat(parts), status)
end

-- Single list page: show todos for a list
function templates.list_page(list, todos)
    local parts = {}

    -- Back link
    table.insert(parts, '<a href="/" class="back-link">&laquo; Back to Lists</a>')

    -- Progress bar
    local done = 0
    for _, t in ipairs(todos) do
        if t.completed == 1 then done = done + 1 end
    end
    local pct = 0
    if #todos > 0 then
        pct = math.floor((done / #todos) * 100)
    end

    table.insert(parts, string.format([[
        <div style="margin-bottom:8px">
            <div style="display:flex;justify-content:space-between;margin-bottom:2px">
                <span style="font-size:13px">Progress</span>
                <span style="font-size:13px">%d/%d (%d%%)</span>
            </div>
            <div class="progress-bar"><div class="progress-fill" style="width:%d%%"></div></div>
        </div>
    ]], done, #todos, pct, pct))

    -- Add todo form
    table.insert(parts, string.format([[
        <div class="win95-toolbar">
            <form method="POST" action="/lists/%d/todos" style="display:flex;gap:4px;width:100%%">
                <input type="text" name="title" class="win95-input" placeholder="Add a new todo..." required>
                <button type="submit" class="win95-btn primary">Add</button>
            </form>
        </div>
    ]], list.id))

    -- Todo list
    table.insert(parts, '<div class="win95-sunken" style="min-height:200px;margin-top:8px">')

    if #todos == 0 then
        table.insert(parts, '<div class="empty-state">No todos yet. Add one above!</div>')
    else
        for _, todo in ipairs(todos) do
            local completed_class = ""
            if todo.completed == 1 then
                completed_class = " completed"
            end
            local check_label = todo.completed == 1 and "Undo" or "Done"
            table.insert(parts, string.format([[
                <div class="todo-item%s">
                    <form method="POST" action="/todos/%d/toggle" style="margin:0;display:inline">
                        <input type="checkbox" class="win95-checkbox" %s
                               onchange="this.form.submit()" title="%s">
                    </form>
                    <span class="todo-text">%s</span>
                    <div class="todo-actions">
                        <form method="POST" action="/todos/%d/delete" style="margin:0"
                              onsubmit="return confirm('Delete this todo?')">
                            <button type="submit" class="win95-btn danger" title="Delete">X</button>
                        </form>
                    </div>
                </div>
            ]], completed_class, todo.id,
                todo.completed == 1 and 'checked="checked"' or '',
                check_label, esc(todo.title), todo.id))
        end
    end

    table.insert(parts, '</div>')

    local status = string.format("List: %s | %d todo(s), %d done", list.name, #todos, done)
    return templates.base(esc(list.name), table.concat(parts), status)
end

return templates
