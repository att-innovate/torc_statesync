// example source - https://github.com/jimmycuadra/rust-etcd
extern crate iron;
extern crate serde;
extern crate serde_json;

use std::thread;
use client::Client;


/// Wrapper around client that automatically cleans up etcd after each test.
#[derive(Debug)]
pub struct TestClient {
	pub c: Client,
}

impl TestClient {
	/// Creates a new client for a test.
	pub fn new() -> TestClient {
		TestClient {
		c: Client::new("http://10.16.0.33:2379").unwrap(),
		//c: Client::new("http://localhost:2379").unwrap(),
		}
	}
}
