# cassandra-rs

This is a (hopefully) maintained rust project that unsafely
exposes the cpp driver at https://github.com/datastax/cpp-driver/
in a somewhat-sane crate.

For the wrapper to work, you must first have installed the datastax-cpp driver.

Follow the steps on the cpp driver [docs](https://github.com/datastax/cpp-driver/blob/15215e170810433511c48c304b9e9ca51ff32b2f/topics/building/README.md)  to do so. 

Make sure that the driver (specifically `libcassandra_static.a` and `libcassandra.so`) are in your `/usr/local/lib64/` directory

You can use it from cargo with

```toml
    [dependencies.cassandra]
    git = "https://github.com/tupshin/cassandra-rs"
```

Or just

```toml
    [dependencies]
    cassandra="*"
```


Here's a straightforward example found in simple.rs:


```rust
    #[macro_use(stmt)]
    extern crate cassandra;
    use cassandra::*;
    use std::str::FromStr;
    
    
    fn main() {
        let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");
        let col_name = "keyspace_name";
    
        let contact_points = ContactPoints::from_str("127.0.0.1").unwrap();
    
        let mut cluster = Cluster::default();
        cluster.set_contact_points(contact_points).unwrap();
        cluster.set_load_balance_round_robin();
    
        match cluster.connect() {
            Ok(ref mut session) => {
                let result = session.execute(&query).wait().unwrap();
                println!("{}", result);
                for row in result.iter() {
                    let col: String = row.get_col_by_name(col_name).unwrap();
                    println!("ks name = {}", col);
                }
            }
            err => println!("{:?}", err),
        }
    }
```

There's additional examples included with the project in src/examples.
