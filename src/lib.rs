pub mod file;
pub mod environment;
pub mod cli;

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn it() {
    let cli = cli::new();
    let env = environment::new("APPNAME_");
    let fil = file::new("config.txt");
  }
}

