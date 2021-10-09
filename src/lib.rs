
pub mod file;
pub mod environment;
pub mod cli;
pub mod configuration_source;

/*[[[cog
import cog
from caseconverter import pascalcase, kebabcase, snakecase, macrocase

kind = "u32"
name = "num_iterations"
name_type = pascalcase(name)
name_cli = f"--{kebabcase(name)}"
name_envvar = macrocase(name)
description = "the number of iterations to perform."
default_value = "42"

envvar_app_name = "MYAPP"

cog.out(f"""
pub mod Configuration {{
  pub struct Config {{
    {name}: {kind}
  }}

  struct CLIRecv {{
    {name}: Receiver<Option<String>>,
  }}

  pub struct Initialise {{
    pub cli: CLI,
    cli_recv: CLIRecv,
    environment: Environment,
  }}

  impl Initialise {{
    pub fn new() -> Self {{
      let mut cli = cli::new();
      let mut cli_recv = CLIRecv {{
        {name}: cli.opt("{name_cli}", "{description}"),
      }};

      Self {{
        cli: cli,
        cli_recv: cli_recv,
        environment: Environment::new("{envvar_app_name}"),
      }}
    }}

    pub fn load(&self) -> Config {{
      self.cli.args(...);
      Config {{
        {name}: self.load_{name}(),
      }}
    }}

    fn load_{name}(&self) -> {kind} {{
      if let Ok(Some({name})) = self.cli_recv.{name}.try_recv().unwrap() {{
        return {name};
      }}

      if let Ok(Some({name})) = self.environment.lookup("{name_envvar}") {{
        return {name};
      }}

      return {default_value};
    }}
  }}
}}
""")
]]]*/

// pub mod Configuration {
//   pub struct Config {
//     num_iterations: u32
//   }

//   struct CLIRecv {
//     num_iterations: Receiver<Option<String>>,
//   }

//   pub struct Initialise {
//     pub cli: CLI,
//     cli_recv: CLIRecv,
//     environment: Environment,
//   }

//   impl Initialise {
//     pub fn new() -> Self {
//       let mut cli = cli::new();
//       let mut cli_recv = CLIRecv {
//         num_iterations: cli.opt("--num-iterations", "the number of iterations to perform."),
//       };

//       Self {
//         cli: cli,
//         cli_recv: cli_recv,
//         environment: Environment::new("MYAPP"),
//       }
//     }

//     pub fn load(&self) -> Config {
//       self.cli.args(...);
//       Config {
//         num_iterations: self.load_num_iterations(),
//       }
//     }

//     fn load_num_iterations(&self) -> u32 {
//       if let Ok(Some(num_iterations)) = self.cli_recv.num_iterations.try_recv().unwrap() {
//         return num_iterations;
//       }

//       if let Ok(Some(num_iterations)) = self.environment.lookup("NUM_ITERATIONS") {
//         return num_iterations;
//       }

//       return 42;
//     }
//   }
// }
//[[[end]]]

#[cfg(test)]
mod test {
  use super::*;

  // fn main() {
  //   let init = 
  // }

  // #[test]
  // fn it() {
  //   let cli = cli::new();
  //   let env = environment::new("APPNAME");
  //   let fil = file::new("config.txt");
  // }
}

