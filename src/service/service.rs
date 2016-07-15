use std::hash::{Hash, SipHasher, Hasher};

#[derive (Ord, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct Service {
    hash_value: u64,
    name: String,
    controller: String,
    node_name: String,
}

impl Service {
    pub fn new(hash_value: u64, name: String, controller: String, node_name: String) -> Service {
        let service = Service {
            hash_value: hash_value,
            name: name,
            controller: controller,
            node_name: node_name,
        };
        
        service 
    }
    
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn get_controller(&self) -> String {
        self.controller.clone()
    }
    
    pub fn get_node_name(&self) -> String {
        self.node_name.clone()
    }
    
    pub fn get_hash_value(&self) -> u64 {
        self.hash_value.clone()
    }
    
    pub fn null_hash_value(&mut self) {
        self.hash_value = 0;
    }
    
    pub fn set_hash(&mut self) {
        self.hash_value = hash(&self);
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
 	let mut s = SipHasher::new();
 	t.hash(&mut s);
 	s.finish()
}

