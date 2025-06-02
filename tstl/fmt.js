const { formatText } = require("lua-fmt");
const fs = require("node:fs");
const path = require("node:path");

const fmt = (filePath) => {
    const data = fs.readFileSync(filePath, "utf8");
    fs.writeFileSync(filePath, formatText(data), "utf8");
};

const traverseAndFormatLuaFiles = (dir) => {
    const entries = fs.readdirSync(dir, { withFileTypes: true });

    entries.forEach((entry) => {
        const fullPath = path.join(dir, entry.name);

        if (entry.isDirectory()) {
            traverseAndFormatLuaFiles(fullPath);
        } else if (entry.isFile() && fullPath.endsWith(".lua")) {
            console.log(fullPath);
            fmt(fullPath);
        }
    });
};

const baseDirs = ["tests"];

baseDirs.forEach((baseDir) => traverseAndFormatLuaFiles(baseDir));
