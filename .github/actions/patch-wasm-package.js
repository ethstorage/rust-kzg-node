const fs = require("fs");
const path = require("path");

const pkgPath = path.join("npm", "wasm32-wasi", "package.json");

const json = JSON.parse(fs.readFileSync(pkgPath, "utf8"));

delete json.cpu;
delete json.os;
delete json.napi;

fs.writeFileSync(pkgPath, JSON.stringify(json, null, 2));
console.log("✓ Patched WASM package.json successfully");
