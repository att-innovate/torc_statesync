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

use service::Service;
use machine::Machine;
use route::Route;
use port::Port;
use std::hash::{Hash, SipHasher, Hasher};

#[derive (Ord, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct Root {
    root_hash: u64,
    
    services_hash: u64,
    services: Vec<Service>,
    
    machines_hash: u64,
    machines: Vec<Machine>,
    
    routes_hash: u64,
    routes: Vec<Route>,
    
    ports_hash: u64,
    ports: Vec<Port>,    
}

impl Root {
        pub fn new() -> Root {
            
        let service_init: Vec<Service> = Vec::new();
        let machine_init: Vec<Machine> = Vec::new();
        let route_init: Vec<Route> = Vec::new();
        let port_init: Vec<Port> = Vec::new();
            
        let root = Root {
            root_hash: 0,
            services_hash: 0,
            services: service_init,
            machines_hash: 0,
            machines: machine_init,
            routes_hash: 0,
            routes: route_init,
            ports_hash: 0,
            ports: port_init,
        };

        root 
    }
    

    /*
        Services
    */
    
    pub fn add_service(&mut self, service: Service) {
        &self.services.push(service.clone());
        &self.services.sort();
        &self.update_hash();
    }
    
    pub fn remove_service(&mut self, service: Service) {
        let hash = service.get_hash_value();
        let position = &self.services.iter().position(|ref service| service.get_hash_value() == hash);
        &self.services.remove(position.unwrap());
        &self.update_hash();
    }
    
    pub fn get_services(&self) -> Vec<Service>{
        self.services.clone()
    }


    /*
        Machines
    */
    pub fn add_machine(&mut self, machine: Machine) {
        &self.machines.push(machine.clone());
        &self.machines.sort();
        &self.update_hash();
    }
    
    pub fn remove_machine(&mut self, machine: Machine) {
        let hash = machine.get_hash_value();
        let position = &self.machines.iter().position(|ref machine| machine.get_hash_value() == hash);
        &self.machines.remove(position.unwrap());
        &self.update_hash();
    }   
    
    pub fn get_machines(&self) -> Vec<Machine>{
        self.machines.clone()
    }
    
   
    /*
        Routes
    */
   
    pub fn add_route(&mut self, route: Route) {
        &self.routes.push(route.clone());
        &self.routes.sort();
        &self.update_hash();
    }
    
    pub fn update_route(&mut self, route: Route) {
        &self.remove_route(route.clone());
        &self.add_route(route.clone());
                
        &self.routes.sort();
        self.update_hash();
    }
    
    pub fn remove_route(&mut self, route: Route) {
        let hash = route.get_hash_value();
        let position = &self.routes.iter().position(|ref route| route.get_hash_value() == hash);
        &self.routes.remove(position.unwrap());
        &self.update_hash();
    }
    
    pub fn get_routes(&self) -> Vec<Route>{
        self.routes.clone()
    }
    
    
    /*
        Ports
    */
   
    pub fn add_port(&mut self, port: Port) {
        self.ports.push(port.clone());
        self.ports.sort();
        self.update_hash();
    }
    
    pub fn update_port(&mut self, port:Port) {
        self.remove_port(port.clone());
        self.add_port(port.clone());
        self.ports.sort();
        self.update_hash();
    }
    
    pub fn remove_port(&mut self, port: Port) {
        let id = port.get_id();
        println!("remove_port vec size {}", self.ports.len());
        let mut position = self.ports.iter().position(|ref port| port.get_id() == id);
        
        match position.as_mut() {
            Some(v) => {
                println!("determining pos remove_port {}", *v);
                let pos:usize = *v;
                println!("pos {}", pos);
                self.ports.remove(pos);
                self.update_hash();
            },
            None => {
                println!("nothing to do here.");
            },
        }
        println!("vec size {}", self.ports.len());


    }
    
    pub fn get_ports(&self) -> Vec<Port>{
        self.ports.clone()
    }
    
    
    /*
        General
    */
    pub fn null_root_hash(&mut self) {
        self.root_hash = 0;
    }
    
    pub fn null_services_hash(&mut self) {
        self.services_hash = 0;
    }
    
    pub fn update_hash(&mut self) {
        self.null_root_hash();
        self.null_services_hash();
        self.services_hash = hash(&self.services);
    }
}
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}