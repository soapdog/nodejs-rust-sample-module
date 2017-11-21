#[macro_use]
extern crate neon;

extern crate csv;
extern crate serde;
extern crate geo;

#[macro_use]
extern crate serde_derive;

use std::string::String;
use std::str::FromStr;

use geo::Point;
use geo::algorithm::haversine_distance::HaversineDistance;

use neon::vm::{Call, JsResult};
use neon::js::{JsNull, JsString, JsUndefined, JsFunction, JsArray, Object, JsObject, JsNumber};
use neon::mem::Handle;
use neon::js::error::JsError;
use neon::js::error::Kind::TypeError;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Airport {
    ident: String,
    kind: String,
    name: String,
    coordinates: String,
    elevation_ft: String,
    continent: String,
    iso_country: String,
    iso_region: String,
    municipality: String,
    gps_code: String,
    iata_code: String,
    local_code: String
}

fn airport_distance(call: Call) -> JsResult<JsUndefined> {
    let fn_handle = call.arguments.get(call.scope, 3).unwrap();

    let scope = call.scope; 
    let file: String = call.arguments.require(scope, 0)?.check::<JsString>()?.value();
	let lat2: f64 = call.arguments.require(scope, 1)?.check::<JsNumber>()?.value();
    let lon2: f64 = call.arguments.require(scope, 2)?.check::<JsNumber>()?.value();
    
    let mut rdr = csv::Reader::from_path(file).expect("csv filename");
    let mut r: Vec<Airport> = vec![];

    for result in rdr.deserialize() {
        let airport: Airport = match result {
            Ok(f) => f,
            Err(_e) => return Ok(JsUndefined::new())
        };

        let v: Vec<&str> = airport.coordinates.split(", ").collect();
        let lon1: f64 = f64::from_str(v[0]).or_else(|_e| JsError::throw(TypeError, "longitude from CSV is wrong"))?;
        let lat1: f64 = f64::from_str(v[1]).or_else(|_e| JsError::throw(TypeError, "latitude from CSV is wrong"))?;
        let p = Point::new(lat1, lon1);
        let dist = p.haversine_distance(&Point::new(lat2, lon2));

        if dist < 30_000.0 {
            r.push(airport.clone());
            
        }
    }

    let arr = JsArray::new(scope, r.len() as u32);
    let mut i = 0;

    for a in r.into_iter() {
        let obj = JsObject::new(scope);

        obj.set("ident", JsString::new(scope, &a.ident).expect("ident from results array is wrong"))?;
        obj.set("kind", JsString::new(scope, &a.kind).expect("kind from results array is wrong"))?;
        obj.set("name", JsString::new(scope, &a.name).expect("name from results array is wrong"))?;
        obj.set("coordinates", JsString::new(scope, &a.coordinates).expect("coordinates from results array is wrong"))?;
        obj.set("elevation_ft", JsString::new(scope, &a.elevation_ft).expect("elevation_ft from results array is wrong"))?;
        obj.set("continent", JsString::new(scope, &a.continent).expect("continent from results array is wrong"))?;
        obj.set("iso_country", JsString::new(scope, &a.iso_country).expect("iso_country from results array is wrong"))?;
        obj.set("iso_region", JsString::new(scope, &a.iso_region).expect("iso_region from results array is wrong"))?;
        obj.set("municipality", JsString::new(scope, &a.municipality).expect("municipality from results array is wrong"))?;
        obj.set("gps_code", JsString::new(scope, &a.gps_code).expect("gps_code from results array is wrong"))?;
        obj.set("iata_code", JsString::new(scope, &a.iata_code).expect("iata_code from results array is wrong"))?;
        obj.set("local_code", JsString::new(scope, &a.local_code).expect("local_code from results array is wrong"))?;

        arr.set(i, obj)?;
        i = i + 1;
    }

    

    if let Some(function) = fn_handle.downcast::<JsFunction>() {
        let args: Vec<Handle<JsArray>> = vec![arr];
    	let _ = function.call(scope, JsNull::new(), args);
    }

	Ok(JsUndefined::new())
}

register_module!(m, {
    m.export("airport_distance", airport_distance)
});
