use std::hash::{Hash, SipHasher, Hasher};

#[derive (Ord, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct Route {
    hash_value: u64,
    from: String,
    to: String, 
}

impl Route {
    pub fn new(from: String, to: String) -> Route {
        let route = Route {
            hash_value: 0,
            from: from,
            to: to,
        };
        
        route
    }
    
    pub fn get_hash_value(&self) -> u64 {
        self.hash_value.clone()
    }

    pub fn get_from(&self) -> String {
        self.from.clone()
    }
    
    pub fn set_to(&mut self, to: String) {
        self.to = to;
    }
    
    pub fn get_to(&self) -> String {
        self.to.clone()
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

