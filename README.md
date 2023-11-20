Standalone Reproduction for

https://github.com/surrealdb/surrealdb/issues/3002


Start Surreal DB instances

```
docker compose up -d
```

Beta-9 is fast :

```
git checkout beta9
cargo run
```

---

1.0.0 Release is slow :

```
git checkout master
cargo run
```


---

pr2999 Fixes :

build surreal binary
```
git checkout pr2999
cd ..

git clone https://github.com/surrealdb/surrealdb surrealdb-pr2999
cd surrealdb-pr2999
git checkout 3f43e741c2744a40948ca4109edbe30c2145ede6
cargo build --release
cd ../..
```

run server
```
SURREAL_WEBSOCKET_MAX_CONCURRENT_REQUESTS=1000 ../surrealdb-pr2999/target/release/surreal start --bind 0.0.0.0:8002
```

run client
```
cargo run
```
