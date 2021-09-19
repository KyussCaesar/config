use std::env;
use std::ffi::OsString;

pub enum Result {
  NoCommandLineArguments,
  SomeArgumentsWereNotValidUnicode {
    args: Vec<std::result::Result<String, OsString>>,
  },
  Success {
    args: Vec<String>,
  }
}

pub struct CommandLine {
  result: Result,
}

impl CommandLine {
  pub fn new() -> Self {
    use self::Result::*;
    Self {
      result: {
        let args: Vec<OsString> = env::args_os().collect();
        if args.len() == 0 { NoCommandLineArguments }
        else {
          let results: Vec<std::result::Result<_,_>> = args.into_iter()
            .map(|arg| arg.into_string())
            .collect();

          if results.iter().any(|x| x.is_err()) {
            SomeArgumentsWereNotValidUnicode { 
              args: results }}

          else {
            Success {
              args: results.into_iter().map(|x| x.unwrap()).collect() }}}}}
  }
}

pub fn new() -> CommandLine {
  CommandLine::new()
}
