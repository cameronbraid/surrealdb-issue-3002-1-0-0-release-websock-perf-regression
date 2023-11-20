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
