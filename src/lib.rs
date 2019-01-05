extern crate libc;
use std::os::unix::io::{AsRawFd, RawFd};
use std::io;
use std::fs::File;
use std::ptr;

/// Convenience function for quickly copying a file from the source to the dest.
pub fn copy<D: AsRawFd>(source: &mut File, dest: &mut D) -> io::Result<u64> {
    SendFile::new(source).and_then(|mut sf| sf.send(dest))
}

/// Abstraction for interacting with Linux's zero-copy I/O sendfile syscall.
pub struct SendFile<'a> {
    file: &'a mut File,
    count: usize,
    offset: Option<libc::off_t>
}

impl<'a> SendFile<'a> {
    /// Set a file to be used as the sendfile source.
    pub fn new(file: &'a mut File) -> io::Result<Self> {
        let count = file.metadata()?.len() as usize;
        Ok(Self { file, count, offset: None })
    }

    /// Set the number of bytes to transfer from the source to the destination.
    pub fn count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    /// Set the offset to begin copying bytes from.
    pub fn offset(mut self, offset: libc::off_t) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Begin copying from the the sendfile to the destination file.
    pub fn send<F: AsRawFd>(&mut self, to: &mut F) -> io::Result<u64> {
        SendFd { fd: self.file.as_raw_fd(), count: self.count, offset: self.offset }.send(to)

    }
}

/// Similar to `SendFile`, but operates on a RawFd instead.
pub struct SendFd {
    fd: RawFd,
    count: usize,
    offset: Option<libc::off_t>
}

impl SendFd {
    /// Set a file to be used as the sendfile source.
    pub fn new<F: AsRawFd>(fd: F) -> io::Result<Self> {
        Ok(Self { fd: fd.as_raw_fd(), count: 0, offset: None })
    }

    /// Set the number of bytes to transfer from the source to the destination.
    pub fn count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    /// Set the offset to begin copying bytes from.
    pub fn offset(mut self, offset: libc::off_t) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Begin copying from the the sendfile to the destination file.
    pub fn send<F: AsRawFd>(&mut self, to: &mut F) -> io::Result<u64> {
        let result = unsafe {
            libc::sendfile(
                to.as_raw_fd(),
                self.fd,
                self.offset.as_mut().map_or(ptr::null_mut(), |off| off as *mut libc::off_t),
                self.count
            )
        };

        if result == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(result as u64)
        }
    }
}
