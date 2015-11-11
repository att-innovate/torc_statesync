# merger of the projects
# -> https://github.com/coreos/etcd - A Rust client library for [etcd]
# -> https://github.com/iron/router
# -> https://github.com/brson/httptest


# etcd

A Rust client library for [etcd](https://github.com/coreos/etcd).

* [etcd](https://crates.io/crates/etcd) on crates.io
* [Documentation](http://jimmycuadra.github.io/rust-etcd/) for the latest crates.io release

# License

[MIT](http://opensource.org/licenses/MIT)

##Usage##
###Post values to etcdclient###
$ curl -X POST -d '{"/att/x":"x2", "/att/y":"c1"}' http://localhost:3000/set

###Check results###
$ curl -L http://10.16.0.31:2379/v2/keys/att/x

{"action":"get","node":{"key":"/att/x","value":"x2","modifiedIndex":28,"createdIndex":28}}

$ curl -L http://10.16.0.31:2379/v2/keys/att/y

{"action":"get","node":{"key":"/att/y","value":"c1","modifiedIndex":29,"createdIndex":29}}

###Simple get without parameter support yet###
$ curl -L http://localhost:3000/

$ curl -L http://localhost:3000/?name#Julius&company#ATT

###Check health status of the etcd cluster###
$ curl -L http://localhost:3000/health

{"health": "true"}
