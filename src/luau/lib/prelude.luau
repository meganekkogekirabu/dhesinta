--[[
    Copyright (C) 2026  Madeleine Choi

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
]]

dh = dh or {}

function dh.setup(): ()
    -- Significant portions of this function are taken from MediaWiki's Scribunto engine, licensed
    -- under GPLv3. The original files do not include a license notice, and can be found here:
    -- https://gerrit.wikimedia.org/r/admin/repos/mediawiki/extensions/Scribunto,general

    -- This should only be called once.
    dh.setup = nil

    -- Remove unsafe functions.
    os = {
        clock = os.clock,
        date = os.date,
        difftime = os.difftime,
        time = os.time,
    }
    debug = {
        traceback = debug.traceback,
    }
    package = {
        loaded = package.loaded,
        preload = package.preload,
        loaders = package.preloaders,
        seeall = package.seeall,
    }
    collectgarbage = nil
    module = nil
    coroutine = nil
    dofile = nil
    loadfile = nil
    io = nil
    file = nil
    load = nil
    loadstring = nil
    string.dump = nil

    -- Reduce precision for os.clock.
    do
        local _os_clock_old = os.clock
        function os.clock(): number
            local v = _os_clock_old
            return math.floor(v * 50000 + 0.5) / 5000
        end
    end

    -- Only allow getmetatable for tables.
    do
        local _getmetatable_old = getmetatable
        function getmetatable(obj: any): table?
            if type(obj) == "table" then
                return _getmetatable_old(obj)
            else
                return
            end
        end
    end
end
