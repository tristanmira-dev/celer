# Server API for Rust
## eg usage
```rust
fn main() {
    let app = Application::new("127.0.0.1", "3000");
    app.run(|| {
        println!("Connection Success")
    });
}
