// example sources
// -> https://github.com/iron/router
// -> https://github.com/brson/httptest

// curl -X POST -d '{"a":3, "b":1.3}' http://localhost:3000/set
// curl -L http://localhost:3000/
// curl -L http://localhost:3000/?key=value

// wireshark debugging of ETCD traffic
// (tcp.port == 2379 || udp.port == 2379) && http

#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]

extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate urlencoded;
extern crate torc_statesync;
extern crate hyper;

use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json::*;
use std::io::Read;
use std::collections::BTreeMap;
use std::collections::HashMap;
use urlencoded::UrlEncodedQuery;
use torc_statesync::TestClient;
use hyper::Client as hclient;

fn main() {
    let mut router = Router::new();

    router.get("/", move |r: &mut Request| do_request(r));
    router.post("/set", move |r: &mut Request| set_entry(r));
    router.get("/health", move |r: &mut Request| do_healthcheck());

    // get service health check
    fn do_healthcheck() -> IronResult<Response> {
        println!("Check health status of ETCD instance");
        let client = hclient::new();
        let mut res = client.get("http://10.16.0.31:2379/health").send().unwrap();
        //assert_eq!(res.status, hyper::Ok);
        let mut response = String::new();
        res.read_to_string(&mut response).unwrap();
        println!("{}", response);
        Ok(Response::with((status::Ok, response)))
    }

    // get service with attributes
    fn do_request(request: &mut Request) -> IronResult<Response> {
        println!("Load attributes out of the request uri");
        match request.get_ref::<UrlEncodedQuery>() {
            Ok(ref hashmap) => {
                println!("Parsed GET request query string:\n {:?}", hashmap);
                println!("store")
            },
            Err(ref e) => println!("{:?}", e)
        };

        // TODO: load key+value out of ETCD
        let result = lookup("key".to_string());

        // Prepare result set to be exposed as JSON
        let mut entry2 = BTreeMap::new();
        entry2.insert(result, "Test1");
        entry2.insert("b".to_string(), "Test2");

        let payload = serde_json::to_string(&entry2).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a POST message and store it persitently.
    fn set_entry(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        println!("received [{}] as payload", payload);
        let deserialized_map: HashMap<String, String> = serde_json::from_str(&payload).unwrap();
        println!("deserialized it.");
        //TODO store deserialized_map persitently
        store(deserialized_map);
        println!("Should store value at this point.");

        Ok(Response::with(status::Ok))
    }

    fn lookup(key: String) -> String {
        println!("Lookup value [{}] in key store.", key);
        let test_client = TestClient::new();

        let key = String::from("/att/a");
        let input = String::from("Foundry");

        // initial input to start communication - necessary for establishing further communication
        test_client.c.set(&key, &input, None).ok().unwrap();

        println!("client setup");
        let response = test_client.c.get(&key, false, false, false).ok().unwrap();

        println!("Value parsing");
        let value = response.node.value.unwrap();
        println!("Value parsing done {}", value);

        println!("client fetched result value as [{}]", value);

        // return lookup value
        value
    }

    fn store(map: HashMap<String, String>){
        println!("store");
        let test_client = TestClient::new();
        for (key, value) in &map {
            println!("key [{}] and value [{}]", key, value);
            test_client.c.set(&key, &value, None).ok().unwrap();
            //let response = testClient1.c.set(&key, &value, None).ok().unwrap();
            //let response = testClient1.c.get(&key, false, false, false).ok().unwrap();

        }
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
