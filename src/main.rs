mod vm;

use clap::{Parser, Subcommand};
use std::io::Write;
use std::process::Command;
use vm::VirtualMachine;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: ArgsCommand,
}

#[derive(Subcommand, Debug)]
enum ArgsCommand {
    ///
    New {
        path: String,
    },
    Build,
    Run,
}

fn main() {
    let args = Args::parse();

    match args.command {
        ArgsCommand::New { path } => {
            std::fs::create_dir(path.clone()).expect("could not create project directory");
            std::fs::create_dir(format!("{}/src", path.clone()))
                .expect("could not create src directory");
            let mut file = std::fs::File::create(format!("{}/src/main.krm", path.clone()))
                .expect("could not create main file");

            file.write(
                format!("node Main {{\n\nfn main() -> int {{\n\treturn 0;\n}}\n\n}}").as_bytes(),
            )
            .expect("could not write to file");
        }
        ArgsCommand::Build => {
            let o = Command::new("C:/Users/mihir/projects/karma/target/release/karma.exe")
                .args(["src/main.krm"])
                .output()
                .expect("could not run karma");

            println!("{:?}", String::from_utf8(o.stdout));
            println!("{:?}", String::from_utf8(o.stderr));
        }
        ArgsCommand::Run => {
            let mut vm = VirtualMachine::new("comp");
            vm.execute();
        }
    }
}
