extern crate libc;
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};

/// Convenience function for quickly copying a file from the source to the dest.
pub fn copy<S: AsRawFd, D: AsRawFd>(count: usize, source: &mut S, dest: &mut D) -> io::Result<()> {
    let mut sendfile = SendFile::new(source, count)?;

    loop {
        match sendfile.send(dest)? {
            0 => return Ok(()),
            _ => (),
        }
    }
}

/// Convenience function for quickly copying a file from the source to the dest.
pub fn copy_callback<CB: FnMut(&mut SendFile, u64), S: AsRawFd, D: AsRawFd>(
    count: usize,
    source: &mut S,
    dest: &mut D,
    mut cb: CB,
) -> io::Result<()> {
    let mut sendfile = SendFile::new(source, count)?;

    loop {
        match sendfile.send(dest)? {
            0 => return Ok(()),
            wrote => cb(&mut sendfile, wrote),
        }
    }
}

/// Abstraction for interacting with Linux's zero-copy I/O sendfile syscall.
///
/// ```rust,no_run
/// # extern crate zio_sendfile;
/// # use std::os::unix::io::{AsRawFd, RawFd};
/// # use std::io;
/// # use zio_sendfile::SendFile;
/// #
/// # pub fn copy<S: AsRawFd, D: AsRawFd>(count: usize, source: &mut S, dest: &mut D) -> io::Result<()> {
/// let mut sendfile = SendFile::new(source, count)?;
///
/// loop {
///     match sendfile.send(dest)? {
///         0 => return Ok(()),
///         _ => ()
///     }
/// }
/// # }
/// ```
pub struct SendFile {
    pub file: RawFd,
    pub count: usize,
    pub offset: libc::off_t,
}

impl SendFile {
    /// Set a file to be used as the sendfile source.
    pub fn new<F: AsRawFd>(file: &mut F, count: usize) -> io::Result<Self> {
        Ok(Self {
            file: file.as_raw_fd(),
            count,
            offset: 0,
        })
    }

    /// Set the number of bytes to write per call.
    pub fn count(mut self, count: usize) -> Self {
        self.count = if count < 1024 { 1024 } else { count };
        self
    }

    /// Set the offset to begin copying bytes from.
    pub fn offset(mut self, offset: libc::off_t) -> Self {
        self.offset = offset;
        self
    }

    /// Begin copying from the the sendfile to the destination file.
    ///
    /// - This will write up to `self.count` amount of bytes at a time.
    /// - The `self.offset` value is updated by the syscall on return.
    pub fn send<F: AsRawFd>(&mut self, to: &mut F) -> io::Result<u64> {
        let result = unsafe {
            libc::sendfile(
                to.as_raw_fd(),
                self.file,
                &mut self.offset as *mut libc::off_t,
                self.count,
            )
        };

        if result == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(result as u64)
        }
    }
}
