package.path = package.path .. ";" .. lfs.writedir() .. "\\Mods\\tech\\Pelican\\scripts\\?.lua"
package.cpath = package.cpath .. ";" .. lfs.writedir() .. "\\Mods\\tech\\Pelican\\bin\\?.dll"

local P = require("pelican")

P.logger.init_config({
    level = "DEBUG",
    pattern = "{d} - {l} - {m}{n}",
    file = lfs.writedir() .. "\\Logs\\pelican.log",
})

P.logger.info("Pelican Running...")
P.logger.info("Pelican Name: " .. P.name)
P.logger.info("Pelican Version: " .. P.version)
P.logger.info(P.json.encode({ a = 1, b = 2 }))
P.logger.info(P.uuid.v4())

local server = P.web.serve({
    host = "127.0.0.1",
    port = 1234,
})

local router = P.web.router()

router:add_method("get_airbases", function()
    function generalPosObj(DCSpos)
        lat, lon, alt = coord.LOtoLL(DCSpos)
        return {
            ['DCS'] = DCSpos,
            ['World'] = {
                ['lat'] = lat,
                ['lon'] = lon,
                ['alt'] = alt
            }
        }
    end

    local reverse_category = {}

    for k, v in pairs(Airbase.Category) do
        reverse_category[v] = k
    end

    local airbases = {}

    -- TODO: Get this from a correct location
    local term_type = {
        [16] = "Runway",
        [40] = "HelicopterOnly",
        [68] = "HardenedAirShelter",
        [72] = "AirplaneOnly",
        [100] = "SmallAirplane",
        [104] = "OpenAirSpawn",
        [176] = "AirplaneOnlyAndOpenAirSpawn",
        [216] = "AllHelicopterUsable",
        [244] = "AllAirplaneUsable",
        [311] = "AllSmallAirplaneUsable",
    }

    for k, v in pairs(world.getAirbases()) do
        local airbase = {
            ID = v:getID(),
            WorldID = v:getWorldID(),
            callsign = v:getCallsign(),
            typeName = v:getTypeName()
        }

        for _k, _v in pairs(v:getDesc()) do
            airbase[_k] = _v
        end

        airbase["category_name"] = reverse_category[airbase.category]

        airbase["runways"] = {}

        for _k, _v in pairs(v:getRunways()) do
            _v["id"] = _k
            table.insert(airbase["runways"], _v)
        end

        airbase["parking"] = {}

        for _k, _v in pairs(v:getParking()) do
            _v["id"] = _k
            table.insert(airbase["parking"], _v)
        end

        airbase["theatre"] = env.mission.theatre
        airbase["pos"] = generalPosObj(v:getPoint())

        for _k, _v in pairs(airbase.parking) do
            _v.Term_Type_Name = term_type[_v.Term_Type]
            _v.pos = generalPosObj(_v.vTerminalPos)
        end

        table.insert(airbases, airbase)
    end

    return airbases
end)

timer.scheduleFunction(function(arg, time)
    local result, err = server:process_rpc(router)
    if not result then
        P.logger.error("RPC error: " .. tostring(err))
    end

    return timer.getTime() + .1
end, nil, timer.getTime() + .1)