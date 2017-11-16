let get_airports_node = require("./airports.js");
let get_airports_rust = require("./lib");

console.time("node");
get_airports_node("./airport-codes.csv", -22.90278, -43.2075, data => {
    console.log("node, result count", data.length);
    console.timeEnd("node");
});

console.time("rust");
get_airports_rust("./airport-codes.csv", -22.90278, -43.2075, data => {
    console.log("rust, result count", data.length);
    console.timeEnd("rust");
});