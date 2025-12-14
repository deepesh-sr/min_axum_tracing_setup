# min_axum_tracing_setup

A minimal Rust web server using Axum with tracing and logging.

## What it does
- Creates a simple REST API with GET and POST endpoints at `/message`
- Logs HTTP requests and responses using `tracing` and `log`
- Uses `tower-http` TraceLayer for automatic request tracing

## How to run
```bash
cargo run
```

The server starts on `http://localhost:3000`

## Test it
```bash
# GET request
curl http://localhost:3000/message

# POST request
curl -X POST http://localhost:3000/message \
  -H "Content-Type: application/json" \
  -d '{"content":"Hello World"}'
```

## Dependencies
- **axum** - Web framework
- **tokio** - Async runtime
- **tracing** - Structured logging
- **tower-http** - HTTP middleware for tracing
- **serde** - JSON serialization
