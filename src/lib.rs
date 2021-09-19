pub mod file;
pub mod environment;
pub mod command_line;

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn it() {
    let cli = command_line::new();
    let env = environment::new("APPNAME_");
    let fil = file::new("config.txt");
  }
}

