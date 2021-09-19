use std::fs;
use std::io;
use std::path::PathBuf;
use std::path::Path;
use std::io::Read;

/// Represents the ways in which file discovery can fail.
#[derive(Debug)]
pub enum Result {
  ErrorAttemptingToOpenFile {
    err: io::Error,
  },
  ErrorAttemptingToReadFileMetadata {
    err: io::Error,
  },
  PathRefersToADirectory {
    metadata: fs::Metadata,
  },
  FileIsTooLarge {
    file_size_bytes: u64,
    metadata: fs::Metadata,
  },
  ErrorAttemptingToReadFileContents {
    err: io::Error,
    metadata: fs::Metadata,
  },
  Success {
    buf: Vec<u8>,
    num_bytes_read: usize,
    metadata: fs::Metadata,
  }
}

/// Represents an attempt to load data from a file.
#[derive(Debug)]
pub struct File {
  path: PathBuf,
  max_file_size_bytes: u64,
  result: Result
}

impl File {
  pub fn new<P: AsRef<Path>>(
    // The path to attempt to read.
    path: P,
    // Threshold for maximum file size (bytes). If the file is larger than this, we will not load it.
    max_file_size_bytes: u64,
  ) -> Self {
    use self::Result::*;
    Self {
      path: path.as_ref().to_path_buf(),
      max_file_size_bytes: max_file_size_bytes,
      result:
        match fs::File::open(path) {
          Err(e) => ErrorAttemptingToOpenFile { err: e },

          Ok(mut f) => {
            match f.metadata() {
              Err(e) => ErrorAttemptingToReadFileMetadata { err: e },

              Ok(m) => {
                if m.is_dir() {
                  PathRefersToADirectory { metadata: m }}
                else {

                  let len = m.len();
                  if len > max_file_size_bytes {
                    FileIsTooLarge { file_size_bytes: len, metadata: m }}
                  else {

                    let mut v = Vec::new();
                    match f.read_to_end(&mut v) {
                      Err(e) => ErrorAttemptingToReadFileContents {
                        err: e, metadata: m },
                      Ok(num_bytes_read) => Success {
                        buf: v, num_bytes_read: num_bytes_read, metadata: m }}}}}}}}}
  }
}

pub fn new<P: AsRef<Path>>(path: P) -> File {
  File::new(path, 4 * 1024)
}

