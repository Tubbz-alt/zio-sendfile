# zio-sendfile

Rust crate to provide a higher level abstraction over Linux's zero-copy I/O syscall:
[sendfile](http://man7.org/linux/man-pages/man2/sendfile.2.html). This provides a significantly
faster variant of `io::copy(&mut source, &mut dest)`, which only works on Linux -- the platform of
choice for the discerning programmer.

## Examples

If you're simply copying a file to a different file descriptor, the copy function can be used:

```rust
extern crate zio_sendfile;

let mut source = File::open("source_path").unwrap();
let mut dest = File::create("dest_path").unwrap();

zio_sendfile::copy(&mut source, &mut dest);
```

If you need a more elaborate configuration, the builder pattern is possible using either the `SendFile` or `SendFd` types:

```rust
extern crate zio_sendfile;

let mut source = File::open("source_path").unwrap();
SendFile::new(&mut source)
    .count(bytes_to_copy)
    .offset(bytes_to_offset)
    .send(&mut File::create("dest_path").unwrap()).unwrap();
```
```
