# Find Airports Close to You Webapp
This is a sample webapp to demonstrate the usage of [Rust 🦀](https://rust-lang.org)-based modules with [NodeJS](https://nodejs.org) webapps.

# How the algorithm works
There is a [CSV](https://en.wikipedia.org/wiki/Comma-separated_values) file containing about 46.000 airports. For each record we have a lot of data, including its Latitude and Longitude.

Our algorithm is quite naive and there is a lot of room for improvements. Basically, we parse the CSV file into a large array of objects and then iterate over this array doing some math to calculate the [Harversine distance](https://en.wikipedia.org/wiki/Haversine_formula) between your location and the airport, if it is below 30km then that airport is included in the results.

There is no parallelism involved even though it would speedup the code a lot. 

> REMARK: Of course there are tons of ways of solving this, going from having it all in an Oracle database with some functions in PL/SQL to calculate everthing with a simple SELECT to having a ton of microservices or serverless lambda functions figuring this out using some third-party API you have no control of. **The main objective of this sample, is to show how easy it is to build Rust-based NodeJS modules by presenting an useful and rich sample.**

# How to setup

You need both NodeJS and Rust installed for this to work. You also need [Neon Bindings](https://guides.neon-bindings.com/) to be able to compile the Rust code. 

Use:

```
$ npm install
```

To install all the required NodeJS modules. To build the Rust-based module using Neon Bindings:

```
$ neon build
```

Run the application with:

```
$ node index.js
```


