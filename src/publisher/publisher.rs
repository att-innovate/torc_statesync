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

extern crate hyper;

use self::hyper::Client;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

#[derive(Clone, Debug)]
pub struct Publisher {
}

impl Publisher {
    pub fn new() -> Publisher {
        let publisher = Publisher {
        };
        publisher
    }

    pub fn test(&self) {
        println!("publisher test");
    }

    pub fn publish_key_value(&self, etcd_url: String, key: String, value: String, sub_path: String) {
        store_key_value(etcd_url.to_string(),
            sub_path.to_string(),
            key.to_string(),
            value.to_string()
        )
    }

    pub fn create_folder(&self, etcd_url: String, folder: String) {
        create_folder(etcd_url.to_string(), folder.to_string());
    }

    pub fn delete_folder(&self, etcd_url: String, folder: String) {
        delete_folder(etcd_url.to_string(), folder.to_string());
    }
    
    pub fn delete_key(&self, etcd_url: String, key: String) {
        delete_key(etcd_url.to_string(), key.to_string());
    }
    
    pub fn store_key_as_string(etcd_ip: String, folder: String, key: &String, value: &String) {
        store_key_value(etcd_ip, folder, key.to_string(), value.to_string());
    }
}



fn create_folder(etcd_ip: String, folder: String) {
    let mut uri = etcd_ip.to_string();
    uri.push_str(&folder.to_string());
    uri.push_str(&"?dir=true".to_string());

    let res = CLIENT.put(&uri.to_string())
                    .send();
    res.unwrap(); 
}

fn delete_folder(etcd_ip: String, folder: String) {
    let mut uri = etcd_ip.to_string();
    uri.push_str(&folder.to_string());
    uri.push_str(&"?recursive=true".to_string());

    let res = CLIENT.delete(&uri.to_string())
                    .send();
    res.unwrap();
}

fn delete_key(etcd_ip: String, key: String) {

    let mut uri = etcd_ip.to_string();
    uri.push_str(&key.to_string());

    let res = CLIENT.delete(&uri.to_string())
                    .send();
    res.unwrap();
}

pub fn store_key_value(etcd_ip: String, folder: String, key: String, value: String) {
    let mut uri = etcd_ip.to_string();
    uri.push_str(&folder.to_string());
    uri.push_str(&key.to_string());
    uri.push_str(&"?value=".to_string());
    uri.push_str(&value.to_string());

    let res = CLIENT.put(&uri.to_string())
                    .send();
    res.unwrap();
}
