const fs = require('fs');
const parse = require('csv-parse');
const geolib = require('geolib');

module.exports = function (file, lat, lon, cb) {

    fs.readFile(file, (err, content) => {
        if (err) {
            throw err;
        }

        parse(content, { columns: true }, (err, data) => {
            if (err) {
                throw err;
            }

            let found = data
                .filter(e => {
                    let lat1, lon1, lat2, lon2;
                    [lon1, lat1] = e.coordinates.split(",");
                    [lat2, lon2] = [lat, lon];

                    let dist = geolib.getDistance(
                        { latitude: lat1, longitude: lon1 },
                        { latitude: lat2, longitude: lon2 }
                    );

                    return dist <= 30000;
                })

            cb(found)

        });

    })
}