# How to run 
## 1. Run server
```command
cd server && cargo build && cargo run
```

## 2. Run client 
```command 
cd client && cargo build && cargo run 
```

- Input request on console then receive response from server
- Example of console log on client:
``` 
1
Server response: OK pong

2
Server response: OK pong
```

# Improvement
The client code currently establishes a new TCP stream connection for each request instead of reusing a single connection for all requests.