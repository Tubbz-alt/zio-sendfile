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
let bytes_per_write = 100 * 1024 * 1024;

zio_sendfile::copy(&mut source, &mut dest, bytes_per_write);
```

Note that the source and destination does not need to be a `File`, but can be any type which implements `AsRawFd`.

If you need a more elaborate configuration, the builder pattern is possible using the `SendFile` type:

```rust
extern crate zio_sendfile;

let mut source = File::open("source_path").unwrap();
SendFile::new(&mut source, 100 * 1024 * 1024)
    .offset(bytes_to_offset)
    .send(&mut File::create("dest_path").unwrap()).unwrap();
```

Each write will update the offset integer stored within the `SendFile`, so it can be
used to track the progress of a copy.
