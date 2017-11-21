let express = require('express');
let get_airports_node = require("./airports.js");
let get_airports_rust = require("./lib");

let router = express();

router
    .use(express.static('public'))
    .get('/airports/node/:lat/:lon', function (req, res) {
        console.log("params", req.params);        
        let lat = parseFloat(req.params.lat);
        let lon = parseFloat(req.params.lon);
        console.time("node");
        get_airports_node("./airport-codes.csv", lat, lon, data => {
            console.log("node, result count", data.length);
            console.timeEnd("node");
            res.send(data);
        });
    })
    .get('/airports/rust/:lat/:lon', function (req, res) {
        console.log("params", req.params);
        let lat = parseFloat(req.params.lat);
        let lon = parseFloat(req.params.lon);
        console.time("rust");
        get_airports_rust("./airport-codes.csv", lat, lon, data => {
            console.log("rust, result count", data.length);
            console.timeEnd("rust");
            res.send(data);
        });
    })
    .listen(5000, () => console.log('Listening on http://localhost:5000/ '))

