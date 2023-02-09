local fzy = require("fzy_impl")

local fzy_filter = {}

local function dump(o)
    if type(o) == "table" then
        local s = "{ "
        for k, v in pairs(o) do
            if type(k) ~= "number" then
                k = '"' .. k .. '"'
            end
            s = s .. "[" .. k .. "] = " .. dump(v) .. ","
        end
        return s .. "} "
    else
        return tostring(o)
    end
end

--
-- Find the last instance of a pattern in a string.
-- https://github.com/premake/premake-4.x/blob/7a6c9b6e1e357250671886c781ba351a4b804207/src/base/string.lua#L31
function string.findlast(s, pattern, plain)
  local curr = 0
  repeat
    local next = s:find(pattern, curr + 1, plain)
    if (next) then curr = next end
  until (not next)
  if (curr > 0) then
    return curr
  end
end

--
-- Retrieve the filename portion of a path.
--
-- https://github.com/premake/premake-4.x/blob/master/src/base/path.lua#L72
local function get_filename(p)
  local i = p:findlast("[/\\]")
  if (i) then
    return { filename = p:sub(i + 1), offset = i}
  else
    return { filename = p, offset = 0 }
  end
end

local function match_text_for(item, match_scope)
  if match_scope == 'FileNameOnly' then
    filename_info = get_filename(item)
    return { match_text = filename_info['filename'], offset = filename_info['offset'] }
  else
    return { match_text = item, offset = 0 }
  end
end

local function apply_fzy(query, candidates, enable_icon, match_scope)
    if string.match(query, '%u') then
        case_sensitive = true
    else
        case_sensitive = false
    end
    matches = {}

    for _, c in pairs(candidates) do
        raw_c = c

        -- Skip two chars, icon + one Space
        if enable_icon then
            c = string.sub(c, 5)
            match_info = match_text_for(c, match_scope)
            c = match_info['match_text']
            offset = match_info['offset'] + 3
        else
            match_info = match_text_for(c, match_scope)
            c = match_info['match_text']
            offset = match_info['offset'] - 1
        end

        -- Enable case_sensitive
        if fzy.has_match(query, c, case_sensitive) then
            positions, score = fzy.positions(query, c)
