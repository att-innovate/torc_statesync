// The MIT License (MIT)
//
// Copyright (c) 2016 AT&T
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

extern crate torc_fboss_client;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate rustc_serialize;

use publisher::Publisher;
use service::Service;
use machine::Machine;
use route::Route;
use port::Port;
use root::Root;

use self::torc_fboss_client::api as fbossapi;
use torc_snaproute_client::api as snaprouteapi;
use std::thread;
use std::hash::{Hash, SipHasher, Hasher};
use std::time::Duration;
use self::rustc_serialize::json::Json;
use std::io::Read;
use self::hyper::Client;
use std::sync::{Arc, Mutex};


lazy_static! {
    static ref LOCAL_CACHE: Arc<Mutex<Root>> = {
        Arc::new(Mutex::new(Root::new()))
    };
    
    static ref CLIENT: Client = Client::new();
    static ref PUBLISHER: Publisher = Publisher::new();
}

pub struct StateSync {
    etcd_url: String,
    base_dir: String,
    network_agent_type: String,
    network_agent_url: String,
    service_api_url: String,
    machine_api_url: String,
}

impl StateSync {
    pub fn new(etcd_url: String,
               base_dir: String,
               network_agent_type: String,
               network_agent_url: String,
               service_api_url: String,
               machine_api_url: String)
               -> StateSync {
        let statesync = StateSync {
            etcd_url: etcd_url,
            base_dir: base_dir,
            network_agent_type: network_agent_type,
            network_agent_url: network_agent_url,
            service_api_url: service_api_url,
            machine_api_url: machine_api_url,
        };
        
        statesync
    }

    pub fn test(&self) {
        println!("statesync test");
    }

    pub fn run(&self) {
        let etcd_url = self.etcd_url.clone();
        let base_dir = self.base_dir.clone();
        let network_agent_type = self.network_agent_type.clone();
        let network_agent_url = self.network_agent_url.clone();
        let service_api_url = self.service_api_url.clone();
        let machine_api_url = self.machine_api_url.clone();

        PUBLISHER.create_folder(self.etcd_url.to_string(), self.base_dir.to_string());

        let child = thread::spawn(move || {

            let mut services_hash_old = 0;
            let mut machines_hash_old = 0;
            let mut routes_hash_old = 0;
            let mut ports_hash_old = 0;

            loop {                    
                println!("\n");
                
                println!("Synchronizing SERVICES - cache size {}.", LOCAL_CACHE.lock().unwrap().get_services().len());
                let (services_hash_new, services): (u64, Vec<Service>) =
                    derive_services(service_api_url.clone());
                if services_hash_new != services_hash_old {
                    identify_service_delta(etcd_url.to_string(), base_dir.to_string(), services);
                    services_hash_old = services_hash_new;
                }

                
                println!("Synchronizing MACHINES - cache size {}.", LOCAL_CACHE.lock().unwrap().get_machines().len());
                let (machines_hash_new, machines): (u64, Vec<Machine>) =
                    derive_machines(machine_api_url.clone());
                if machines_hash_new != machines_hash_old {
                    identify_machine_delta(etcd_url.to_string(), base_dir.to_string(), machines);
                    machines_hash_old = machines_hash_new;
                }

                
                println!("Synchronizing ROUTES - cache size {}.", LOCAL_CACHE.lock().unwrap().get_routes().len());
                let (routes_hash_new, routes): (u64, Vec<Route>) = get_routes(network_agent_type.clone(), network_agent_url.clone());
                if routes_hash_new != routes_hash_old {
                    identify_routes_delta(etcd_url.to_string(), base_dir.to_string(), routes);
                    routes_hash_old = routes_hash_new;
                }

                println!("Synchronizing PORTS - cache size {}.", LOCAL_CACHE.lock().unwrap().get_ports().len());
                let (ports_hash_new, ports): (u64, Vec<Port>) = get_ports(network_agent_type.clone(), network_agent_url.clone());
                if ports_hash_new != ports_hash_old {
                    identify_ports_delta(etcd_url.to_string(), base_dir.to_string(), ports);
                    ports_hash_old = ports_hash_new;
                }
                
                
                println!("Finished round in StateSync.");
                thread::sleep(Duration::from_secs(5));

            }
        });

        let res = child.join();
        res.unwrap();
    }
}

    /*
        Machines
    */
fn derive_machines(api_url: String) -> (u64, Vec<Machine>) {
    let mut machines: Vec<Machine> = Vec::new();
    let jsondata = query_json_from_api(api_url.to_string());
    
    let json = Json::String("".to_string());
    
    if jsondata.to_string() != json.to_string() {
        let json_vec = jsondata.as_array().unwrap();
        for json_object in json_vec.iter() {

            let entry_map = json_object.as_object().unwrap();
            
            let mut name = "".to_string();
            let mut ip = "".to_string();
            let mut node_type = "".to_string();

            for (key, value) in entry_map {
                let cleaned: String = match *value {
                    Json::U64(v) => format!("{}", v),
                    Json::Boolean(b) => format!("{}", b),
                    Json::String(ref v) => format!("{}", v),
                    Json::Object(ref _o) => format!("object"),
                    Json::Array(ref _a) => format!("array"),
                    _ => String::new(),
                };
            
                if key == "name" {
                    name = cleaned;
                } else if key == "ip" {
                    ip = cleaned;
                }
                else if key == "node_type" {
                    node_type = cleaned;
                }
            }

            let mut machine_entry: Machine = Machine::new(0, name, ip, node_type);
            machine_entry.set_hash();
            machines.push(machine_entry);
        }

        machines.sort();
    }
      
    let hash: u64 = hash(&machines);

    (hash, machines)
}

fn identify_machine_delta(etcd_url: String, base_folder: String, new_machines: Vec<Machine>) {
    let folder: String = format!("{}/{}", base_folder.to_string(), "machines".to_string());
    // TODO: check entry in ETCD

    let mut delta_remove: Vec<Machine> = LOCAL_CACHE.lock().unwrap().get_machines();
    let current_machines = LOCAL_CACHE.lock().unwrap().get_machines();

    for new_machine in new_machines {
        let current_machine_hash = get_current_machine_hash(current_machines.clone(), new_machine.get_name());
                
        if current_machine_hash==new_machine.get_hash_value() {
            let position = delta_remove.iter().position(|ref current_machines| current_machines.get_hash_value() == new_machine.get_hash_value());
            delta_remove.remove(position.unwrap());
        } else {
            println!("\tAdd new machine  - name: {}.", new_machine.get_name());    
            LOCAL_CACHE.lock().unwrap().add_machine(new_machine.clone());
            add_machine_to_etcd(etcd_url.to_string(), folder.to_string(), new_machine.clone());
        }
    }
    
    machine_delete_delta_in_etcd(etcd_url.to_string(), folder.to_string(), delta_remove);
}

fn add_machine_to_etcd(etcd_url: String, folder: String, machine: Machine) {
    let item_path = format!("{}/{}/", folder.to_string(), machine.get_name());
    
    PUBLISHER.publish_key_value(etcd_url.to_string(), "machine_hash".to_string(), format!("{}", machine.get_hash_value()), item_path.to_string());
    PUBLISHER.publish_key_value(etcd_url.to_string(), "machinename".to_string(), machine.get_name(), item_path.to_string());
    PUBLISHER.publish_key_value(etcd_url.to_string(), "ip".to_string(), machine.get_ip(), item_path.to_string());
    PUBLISHER.publish_key_value(etcd_url.to_string(), "node_type".to_string(), machine.get_node_type(), item_path.to_string());   
}

fn machine_delete_delta_in_etcd(etcd_url: String, folder: String, machines: Vec<Machine>) {
    for machine in machines {
        println!("\tDelete entry {}.", machine.get_name());
        LOCAL_CACHE.lock().unwrap().remove_machine(machine.clone());
        let item_path = format!("{}/{}", folder.to_string(), machine.get_name());
        PUBLISHER.delete_folder(etcd_url.to_string(), item_path.to_string());
    }
}

fn get_current_machine_hash(current_machines: Vec<Machine>, name: String) -> u64 {
    let mut hash: u64 = 0;
    
    for current_machine in current_machines.clone() {
        if name.to_string()==current_machine.get_name() {
            hash = current_machine.get_hash_value();
            break;
        }
    }
    
    hash
}


    /*
        Services
    */
fn derive_services(api_url: String) -> (u64, Vec<Service>) {
    let mut services: Vec<Service> = Vec::new();
    let jsondata = query_json_from_api(api_url.to_string());    
    let json = Json::String("".to_string());
    
    if jsondata.to_string() != json.to_string() {
        let json_vec = jsondata.as_array().unwrap();
        for json_object in json_vec.iter() {

            let entry_map = json_object.as_object().unwrap();
            
            let mut name = "".to_string();
            let mut controller = "".to_string();
            let mut node_name = "".to_string();

            for (key, value) in entry_map {
                let cleaned: String = match *value {
                    Json::U64(v) => format!("{}", v),
                    Json::Boolean(b) => format!("{}", b),
                    Json::String(ref v) => format!("{}", v),
                    Json::Object(ref _o) => format!("object"),
                    Json::Array(ref _a) => format!("array"),
                    _ => String::new(),
                };
            
                if key == "name" {
                    name = cleaned;
                } else if key == "controller" {
                    controller = cleaned;
                }
                else if key == "node_name" {
                    node_name = cleaned;
                }
            }

            let mut service_entry: Service = Service::new(0, name, controller, node_name);
            service_entry.set_hash();
            services.push(service_entry);
        }

        services.sort();
    }
      
    let hash: u64 = hash(&services);

    (hash, services)
}


fn identify_service_delta(etcd_url: String, base_folder: String, new_services: Vec<Service>) {
    let folder: String = format!("{}/{}", base_folder.to_string(), "services".to_string());
    let mut delta_remove: Vec<Service> = LOCAL_CACHE.lock().unwrap().get_services();
    let current_services = LOCAL_CACHE.lock().unwrap().get_services();

    for new_service in new_services {
        let current_service_hash = get_current_service_hash(current_services.clone(), new_service.get_name());
        
        if current_service_hash==new_service.get_hash_value() {
            let position = delta_remove.iter().position(|ref current_services| current_services.get_hash_value() == new_service.get_hash_value());
            delta_remove.remove(position.unwrap());
        } else {
            println!("\tAdd new service  - name: {}.", new_service.get_name());
            LOCAL_CACHE.lock().unwrap().add_service(new_service.clone());
            add_service_to_etcd(etcd_url.to_string(), folder.to_string(), new_service.clone());
        }
    }
    
    service_delete_delta_in_etcd(etcd_url.to_string(), folder.to_string(), delta_remove);
}

fn add_service_to_etcd(etcd_url: String, folder: String, service: Service) {
    let item_path = format!("{}/{}/", folder.to_string(), service.get_name());
    
    PUBLISHER.publish_key_value(etcd_url.to_string(), "service_hash".to_string(), format!("{}", service.get_hash_value()), item_path.to_string());
    PUBLISHER.publish_key_value(etcd_url.to_string(), "servicename".to_string(), service.get_name(), item_path.to_string());
    PUBLISHER.publish_key_value(etcd_url.to_string(), "servicecontroller".to_string(), service.get_controller(), item_path.to_string());
    PUBLISHER.publish_key_value(etcd_url.to_string(), "node_name".to_string(), service.get_node_name(), item_path.to_string());   
}


fn service_delete_delta_in_etcd(etcd_url: String, folder: String, services: Vec<Service>) {
    for service in services {
        println!("\tDelete entry {}.", service.get_name());
        LOCAL_CACHE.lock().unwrap().remove_service(service.clone());
        let item_path = format!("{}/{}", folder.to_string(), service.get_name());
        PUBLISHER.delete_folder(etcd_url.to_string(), item_path.to_string());
    }
}



fn get_current_service_hash(current_services: Vec<Service>, name: String) -> u64 {
    let mut hash: u64 = 0;
    
    for current_service in current_services.clone() {
        if name.to_string()==current_service.get_name() {
            hash = current_service.get_hash_value();
            break;
        }
    }
    
    hash
}



    /*
        FBOSS and SNAPROUTE routes
    */ 

fn get_routes(agent_type: String, url: String) -> (u64, Vec<Route>) {
    let mut routes: Vec<Route> = Vec::new();

    if agent_type.to_string() == "fboss" {
        let result_routes = fbossapi::get_routes(&url.clone());

        for route in &result_routes {
            let mut from = route.from.to_string();
            let mut to = route.to.to_string();

            if from.contains("/") {
                from = from.replace("/", "-");
            }
            if to.contains('/') {
                to = to.replace("/", "-");
            }
            let mut route: Route = Route::new(from.to_string(), to.to_string());
            route.set_hash();
            routes.push(route);
        }
    } else if agent_type.to_string() == "snaproute" {
        let result_routes = snaprouteapi::get_routes(&url.clone());
        for route in &result_routes {
            let mut from = route.from.to_string();
            let mut to = route.to.to_string();

            if from.contains("/") {
                from = from.replace("/", "-");
            }
            if to.contains('/') {
                to = to.replace("/", "-");
            }
            let mut route: Route = Route::new(from.to_string(), to.to_string());
            route.set_hash();
            routes.push(route);
        }
    }  



    routes.sort();
    
    let hash: u64 = hash(&routes);
    
    (hash, routes)
}

fn identify_routes_delta(etcd_url: String, base_folder: String, new_routes: Vec<Route>) {
    let folder: String = format!("{}/{}", base_folder.to_string(), "routes".to_string());
    // TODO: check entry in ETCD

    let mut delta_remove: Vec<Route> = LOCAL_CACHE.lock().unwrap().get_routes();
    let current_routes = LOCAL_CACHE.lock().unwrap().get_routes();

    for new_route in new_routes {
        let current_routes_hash = get_current_routes_hash(current_routes.clone(), new_route.get_from());
                
        if current_routes_hash==new_route.get_hash_value() { // found unmodified entry
            let position = delta_remove.iter().position(|ref current_routes| current_routes.get_hash_value() == new_route.get_hash_value());
            delta_remove.remove(position.unwrap());
        } else if current_routes_hash>0 { // found entry, which has been modified - update in ETCD
            println!("\tUpdate existing route  - name: {}.", new_route.get_from());    
            LOCAL_CACHE.lock().unwrap().update_route(new_route.clone());
            update_route_in_etcd(etcd_url.to_string(), folder.to_string(), new_route.clone());
        } else { // found new entry - add to ETCD
            println!("\tAdd new route  - name: {}.", new_route.get_from());    
            LOCAL_CACHE.lock().unwrap().add_route(new_route.clone());
            add_route_to_etcd(etcd_url.to_string(), folder.to_string(), new_route.clone());
        }
    }
    
    route_delete_delta_in_etcd(etcd_url.to_string(), folder.to_string(), delta_remove);
}

fn add_route_to_etcd(etcd_url: String, folder: String, route: Route) {
    let item_path = format!("{}/{}/", folder.to_string(), route.get_from());
    
    PUBLISHER.publish_key_value(etcd_url.to_string(), "route_hash".to_string(), format!("{}", route.get_hash_value()), item_path.to_string());
    PUBLISHER.publish_key_value(etcd_url.to_string(), "from".to_string(), route.get_from(), item_path.to_string());
    PUBLISHER.publish_key_value(etcd_url.to_string(), "to".to_string(), route.get_to(), item_path.to_string());   
}

fn update_route_in_etcd(etcd_url: String, folder: String, route: Route) {
    let item_path = format!("{}/{}/", folder.to_string(), route.get_from());
    
    PUBLISHER.publish_key_value(etcd_url.to_string(), "from".to_string(), route.get_from(), item_path.to_string());   
}

fn route_delete_delta_in_etcd(etcd_url: String, folder: String, routes: Vec<Route>) {
    for route in routes {
        println!("\tDelete entry {}.", route.get_from());
        LOCAL_CACHE.lock().unwrap().remove_route(route.clone());
        let item_path = format!("{}/{}", folder.to_string(), route.get_from());
        //PUBLISHER.delete_key(etcd_url.to_string(), item_path.to_string());
        PUBLISHER.delete_folder(etcd_url.to_string(), item_path.to_string());
    }
}

fn get_current_routes_hash(current_routes: Vec<Route>, from: String) -> u64 {
    let mut hash: u64 = 0;
    
    for current_route in current_routes.clone() {
        if from.to_string()==current_route.get_from() {
            hash = current_route.get_hash_value();
            break;
        }
    }
    
    hash
}


    /*
        FBOSS and SNAPROUTE ports
    */ 
fn get_ports(agent_type: String, url: String) -> (u64, Vec<Port>) {
    let mut ports: Vec<Port> = Vec::new();

    if agent_type.to_string() == "fboss" {
        let result_ports = fbossapi::get_ports_stats(&url.clone());
        for port in &result_ports {
            let mut id = format!("{}",port.id);
            let mut state = format!("{}",port.connected);

            if id.contains("/") {
                id = id.replace("/", "-");
            }
            if state.contains('/') {
                state = state.replace("/", "-");
            }
            let mut new_port: Port = Port::new(id.to_string(), state.to_string());
            new_port.set_hash();
            ports.push(new_port);
        }
    } else if agent_type.to_string() == "snaproute" {
        let result_ports = snaprouteapi::get_ports_stats(&url.clone());
        for port in &result_ports {
            let mut id = format!("{}",port.id);
            let mut state = format!("{}",port.connected);

            if id.contains("/") {
                id = id.replace("/", "-");
            }
            if state.contains('/') {
                state = state.replace("/", "-");
            }
            let mut new_port: Port = Port::new(id.to_string(), state.to_string());
            new_port.set_hash();
            ports.push(new_port);
        }
    }  

    ports.sort();
    
    let hash: u64 = hash(&ports);
    
    (hash, ports)
}

fn identify_ports_delta(etcd_url: String, base_folder: String, new_ports: Vec<Port>) {    
    let folder: String = format!("{}/{}/", base_folder.to_string(), "ports".to_string());
    // TODO: check entry in ETCD

    let mut delta_remove: Vec<Port> = LOCAL_CACHE.lock().unwrap().get_ports().clone();
    let current_ports = LOCAL_CACHE.lock().unwrap().get_ports().clone();

    for new_port in new_ports {
        let current_ports_hash = get_current_port_hash(current_ports.clone(), new_port.get_id());
                
        if current_ports_hash==new_port.get_hash_value() { // found unmodified entry
            let position = delta_remove.iter().position(|ref current_ports| current_ports.get_hash_value() == new_port.get_hash_value());
            delta_remove.remove(position.unwrap());
        } else if current_ports_hash>0 { // found entry, which has been modified - update in ETCD
            println!("\tUpdate existing port - id: {} and state {}.", new_port.get_id(), new_port.get_state());    
            LOCAL_CACHE.lock().unwrap().update_port(new_port.clone());
            update_port_in_etcd(etcd_url.to_string(), folder.to_string(), new_port.clone());
            println!("done updating ports");
        } else { // found new entry - add to ETCD
            println!("\tAdd new port  - id: {}.", new_port.get_id());    
            LOCAL_CACHE.lock().unwrap().add_port(new_port.clone());
            add_port_to_etcd(etcd_url.to_string(), folder.to_string(), new_port.clone());
        }
    }
}

fn add_port_to_etcd(etcd_url: String, folder: String, port: Port) {
    PUBLISHER.publish_key_value(etcd_url.to_string(), port.get_id(), port.get_state(), folder.to_string());
}

fn update_port_in_etcd(etcd_url: String, folder: String, port: Port) {
    println!("PUBLISHER.publish_key_value - {}", port.get_state());
    PUBLISHER.publish_key_value(etcd_url.to_string(), port.get_id(), port.get_state(), folder.to_string());
}

fn get_current_port_hash(current_ports: Vec<Port>, id: String) -> u64 {
    let mut hash: u64 = 0;
    
    for current_port in current_ports.clone() {
        if id.to_string()==current_port.get_id() {
            hash = current_port.get_hash_value();
            break;
        }
    }
    
    hash
}

    /*
        General
    */        

fn query_json_from_api(api_url: String) -> Json {
    let mut res = CLIENT.get(&api_url.to_string())
                    .send();
    let mut tmpjson = Json::String("".to_string());
    
    if res.is_ok() {
        let mut result = res.unwrap();
        let mut body = String::new();
        result.read_to_string(&mut body).unwrap();

        let jsondata = Json::from_str(&body.clone()).unwrap();

        jsondata
    } else {
        res = CLIENT.get(&api_url.to_string())
                    .send();

        if res.is_ok() {
            let mut result = res.unwrap();
            let mut body = String::new();
            result.read_to_string(&mut body).unwrap();

            let jsondata = Json::from_str(&body).unwrap();

            if jsondata.to_string() != "".to_string() {
                tmpjson = jsondata;
            }
        } else {
          println!("Error in querying json data");  
        }

        tmpjson
    }
}
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}