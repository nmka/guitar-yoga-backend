# guitar-yoga-backend
Axum guitar yoga api

### development server start with reload 
```
cargo watch -q -c -w src/
```

### run tests on each time tests changes

```
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
