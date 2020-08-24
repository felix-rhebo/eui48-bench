# Benchmarks for eui48 fork

## BinCode using String Serialization without "serde_bytes" feature

Cargo.toml:
```
[dependencies]
bincode = "1.3"
eui48 = { git = "https://github.com/felix-rhebo/eui48.git", features = ["serde"] }
```

Benchmark:
```
$> cargo bench                                                                                                                                                                                                       
   Compiling eui48 v1.1.0 (https://github.com/felix-rhebo/eui48.git#366a5360)
   Compiling eui48-bench v0.1.0 (/dataPool/rhebo/projects/eui48-bench)
    Finished bench [optimized] target(s) in 0.95s
     Running target/release/deps/eui48_bench-b9c1698cf4d7a38f                                                                                                                                                                                                         
                                                                                                                                                                                                                                                                      
running 2 tests                                                                                                                                                                                                                                                       
test tests::bench_deserialize ... bench:      25,632 ns/iter (+/- 6,134)                                                                                                                                                                                              
test tests::bench_serialize   ... bench:         215 ns/iter (+/- 47)                                                                                                                                                                                                 
                                                                                                                                                                                                                                                                      
test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
```

## BinCode using Binary Serialization with "serde_bytes" feature

Cargo.toml:
```
[dependencies]
bincode = "1.3"
eui48 = { git = "https://github.com/felix-rhebo/eui48.git", features = ["serde", "serde_bytes"] }
```

Benchmark:
```
$> cargo bench                                                                                                                                                                                                       
   Compiling eui48-bench v0.1.0 (/dataPool/rhebo/projects/eui48-bench)
    Finished bench [optimized] target(s) in 0.51s
     Running target/release/deps/eui48_bench-b8735f2b4d36a734                                                                                                                                                                                                         
                                                                                                                                                                                                                                                                      
running 2 tests                                                                                                                                                                                                                                                       
test tests::bench_deserialize ... bench:          25 ns/iter (+/- 2)                                                                                                                                                                                                  
test tests::bench_serialize   ... bench:          14 ns/iter (+/- 3)                                                                                                                                                                                                  
                                                                                                                                                                                                                                                                      
test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
```