use crate::command_line::CommandLine;
use crate::command_line;

use std::mpsc::Receiver;
use std::mpsc::Sender;
use std::mpsc::channel;

pub struct Flag {
  name_set: String,
  name_unset: String,
  help: String,
  tx: Sender<Option<bool>>,
  // val: Option<bool>,
}

pub struct Opt {
  name: String,
  help: String,
  tx: Sender<Option<String>>,
  // val: Option<String>,
}

pub struct OptMany {
  name: String,
  help: String,
  tx: Sender<Option<Vec<String>>>,
  // val: Option<String>,
}

pub struct Choice {
  name: String,
  help: String,
  choices: Vec<String>,
  tx: Sender<Option<String>>,
  // val: Option<String>,
}

pub struct ChoiceMany {
  name: String,
  help: String,
  choices: Vec<String>,
  tx: Sender<Option<Vec<String>>>,
  // val: Option<String>,
}

#[derive(Default)]
pub struct CLI {
  flag: Vec<Flag>,
  choice: Vec<Choice>,
  choice_many: Vec<ChoiceMany>,
  opt: Vec<Opt>,
  opt_many: Vec<OptMany>,
}

impl CLI {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn flag(&mut self,
    name_set: &str, name_unset: &str, help: &str
  ) -> Receiver<Option<bool>> {

    let (tx, rx) = channel();
    self.flag.push(Flag {
      name_set: name_set.into(),
      name_unset: name_unset.into(),
      help: help.into(),
      tx: tx });

    rx
  }

  pub fn opt(&mut self,
    name: &str, help: &str
  ) -> Receiver<Option<String>> {

    let (tx, rx) = channel();
    self.opt.push(Opt {
      name: name.into(),
      help: help.into(),
      tx: tx});

    rx
  }

  pub fn opt_many(&mut self,
    name: &str, help: &str
  ) -> Receiver<Option<Vec<String>>> {

    let (tx, rx) = channel();
    self.opt_many.push(Opt {
      name: name.into(),
      help: help.into(),
      tx: tx});

    rx
  }

  pub fn choice(&mut self,
    name: &str, help: &str, allowed_values: Vec<String>
  ) -> Receiver<Option<String>> {

    let (tx, rx) = channel();
    self.choice.push(Optl {
      name: name.into(),
      help: help.into(),
      allowed_values: allowed_values,
      tx: tx });

    rx
  }

  pub fn choice_many(&mut self,
    name: &str, help: &str, allowed_values: Vec<String>
  ) -> Receiver<Option<Vec<String>>> {

    let (tx, rx) = channel();
    self.choice.push(Optl {
      name: name.into(),
      help: help.into(),
      allowed_values: allowed_values,
      tx: tx });

    rx
  }

  pub fn args(self, args: Vec<String>) -> Result<(), Error> {
    // TODO flip it around, iterate over args and look for CLI items
    // change the tx types to non-option, just don't send if there isn't anything

    for flag in self.flag {
      let mut val = None;
      for arg in args {
        if arg == "--" { break; }
        else {
          if arg == flag.name_set { val = Some(true) }
          else if arg == flag.name_unset { val = Some(false) }}}
      flag.tx.send(val).ok();}

    for opt in self.opt {
      let mut val = None;
      let mut iter = args.iter();
      loop {
        let arg = iter.next();
        match arg {
          None => {
            opt.tx.send(val).ok();
            break; },
          Some(a) => {
            if a == "--" { break;}
            if let Some(tail) = a.strip_prefix(opt.name) {
              if tail.len() == 0 {
                match iter.next() {
                  None => return Err(no value for option),
                  Some(v2) => {
                    if let v2 == "--" {
                      return Err(no value for option);}
                    else {
                      val = Some(v2);}}}}
              else {
                if let Some(v2) = tail.strip_prefix("=") {
                  val = Some(v2);}}}}}}

      opt.tx.send(val).ok();}
}

pub fn new() -> CLI {
  CLI::new()
}

