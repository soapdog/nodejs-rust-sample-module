let get_airports = require("./airports.js");

console.time("node");
get_airports("./airport-codes.csv", -22.90278, -43.2075, data => {
    console.log("node", data.length);
    console.timeEnd("node");
});

console.time("rust");
require("./lib")("./airport-codes.csv", -22.90278, -43.2075, data => {
    console.log("rust", data.length);
    console.timeEnd("rust");
});