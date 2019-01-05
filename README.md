# zio-sendfile

Rust crate to provide a higher level abstraction over Linux's zero-copy I/O syscall:
[sendfile](http://man7.org/linux/man-pages/man2/sendfile.2.html). This provides a significantly
faster variant of `io::copy(&mut source, &mut dest)`, which only works on Linux -- the platform of
choice for the discerning programmer.

```rust
extern crate zio_sendfile;

let mut source = File::open("source_path").unwrap();
let mut dest = File::create("dest_path").unwrap();

zio_sendfile::copy(&mut source, &mut dest);
```
