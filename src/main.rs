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

extern crate statesync;
extern crate clap;

use clap::{App, Arg};
use statesync::statesync::StateSync;

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    println!("\nStatesync v{}\n", VERSION);

    let matches = App::new("etcd rust fbos")
        .about("handles state of fboss in etcd.")
        .arg(Arg::with_name("ETCD_IP")
            .short("e")
            .long("etcd")
            .required(true)
            .help("IP of etcd service")
            .takes_value(true))
        .arg(Arg::with_name("BASE DIR")
            .short("b")
            .long("basedir")
            .required(true)
            .help("base dir in etcd to store infos")
            .takes_value(true))
        .arg(Arg::with_name("AGENT TYPE")
            .short("a")
            .long("agenttype")
            .required(true)
            .help("Agent type - fboss or snaproute")
            .takes_value(true))
        .arg(Arg::with_name("AGENT URL")
            .short("u")
            .long("agenturl")
            .required(false)
            .help("Path to agent url")
            .takes_value(true))
        .arg(Arg::with_name("SERVICE API")
            .short("s")
            .long("service")
            .required(true)
            .help("API to retreive service stats")
            .takes_value(true))
        .arg(Arg::with_name("MACHINE API")
            .short("m")
            .long("machine")
            .required(true)
            .help("API to rereive machine stats")
            .takes_value(true))
        .get_matches();

    println!("Configure RUST_ETCD_FBOSS service.");
    let etcd_ip = matches.value_of("ETCD_IP").unwrap();
    println!("\t* Using ETCD unning on IP: {}", etcd_ip);

    let base_dir = matches.value_of("BASE DIR").unwrap();
    println!("\t* Base dir to store infos in ETCD: {}", base_dir);

    let service_api = matches.value_of("SERVICE API").unwrap();
    println!("\t* API to retreive service stats: {}", service_api);

    let machine_api = matches.value_of("MACHINE API").unwrap();
    println!("\t* API to retreive machine stats: {}", machine_api);

    let agent_type = matches.value_of("AGENT TYPE").unwrap();
    println!("\t* NETWORK AGENT TYPE on url: {}", agent_type);

    let agent_url = matches.value_of("AGENT URL").unwrap();
    println!("\t* Querying NETWORK AGENT on url: {}", agent_url);

    println!("Starting statesync - call APIs.");
    let sync = StateSync::new(etcd_ip.to_string(),
                              base_dir.to_string(),
                              agent_type.to_string(),
                              agent_url.to_string(),
                              service_api.to_string(),
                              machine_api.to_string());

    sync.run();

}