use std::hash::{Hash, SipHasher, Hasher};

#[derive (Ord, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct Machine {
    hash_value: u64,
    name: String,
    ip: String,
    node_type: String,
}

impl Machine {
    pub fn new(hash_value: u64, name: String, ip: String, node_type: String) -> Machine {
        let machine = Machine {
            hash_value: hash_value,
            name: name,
            ip: ip,
            node_type: node_type,
        };
        
        machine
    }
    
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn get_ip(&self) -> String {
        self.ip.clone()
    }
    
    pub fn get_node_type(&self) -> String {
        self.node_type.clone()
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

