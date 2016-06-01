# djb33-rs
DJB33 hash algorithm write in Rust

### Installation

Add djb33 via your Cargo.toml

    [dependencies]
    djb33 = "*"

### Usage

    extern crate djb33;

    fn main() {
        let s = "HelloWorld.";
        
        let mut h1 = djb33::DJB33_INIT;
        h1 = djb33::djb33(h1, s.as_bytes());
        
        let h2 = djb33::djb33_xor(djb33::DJB33_INIT, s.as_bytes());
        
        println!("djb33 {}", h1);
        println!("djb33 xor {}", h2);
    }

### License

MIT
