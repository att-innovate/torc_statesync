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

#[macro_use]
extern crate lazy_static;
extern crate torc_snaproute_client;

pub mod service;
pub mod machine;
pub mod route;
pub mod port;
pub mod root;
pub mod statesync;
pub mod publisher;


#[cfg(test)]
mod tests {
    use service::Service;
    use root::Root;

    #[test]
    fn add_service_1() {
        let mut root = Root::new(0, "root_1".to_string(), 0, Vec::new());
        
        let service1 = Service::new(0, "service_name_1".to_string(), "controller_1".to_string());    
        root.add_service(service1);
        
        let service2 = Service::new(0, "service_name_2".to_string(), "controller_1".to_string());    
        root.add_service(service2);

        let mut root = Root::new(0, "root_1".to_string(), 0, Vec::new());
        
        let service2 = Service::new(0, "service_name_2".to_string(), "controller_1".to_string());    
        root.add_service(service2);

        let service1 = Service::new(0, "service_name_1".to_string(), "controller_1".to_string());    
        root.add_service(service1);
    }
}