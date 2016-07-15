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

use std::hash::{Hash, SipHasher, Hasher};

#[derive (Ord, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct Port {
    hash_value: u64,
    id: String,
    state: String, //read 'state'' as String from FBOSS and write it in ETCD as String, too
}

impl Port {
    pub fn new(id: String, state: String) -> Port {
        let port = Port {
            hash_value: 0,
            id: id,
            state: state,
        };
        
        //set_hash();
        port
    }
    
    pub fn get_hash_value(&self) -> u64 {
        self.hash_value.clone()
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    
    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }
    
    pub fn get_state(&self) -> String {
        self.state.clone()
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

