mod commandline;
mod pts_loader;

use commandline::{Commandline, CommandlineOptions::*, ProgrammMode::*};
use pts_loader::dataset::DataSet;

fn main() -> std::io::Result<()> {
    match Commandline::parse() {
        StopRightHere => Commandline::print_help(),
        Repl => println!("{}", "run interactively"),
        SingleFile(cmd) => {
            let filename = cmd.filename();
            match DataSet::init(filename) {
                Ok(dataset) => match cmd.options() {
                    SiErrors => dataset.print_si_errors(cmd.verbose(), cmd.utc()),
                    VaEventLogoErrors => println!("Show me all logo errors"),
                    GrepIllegalEvents(illegals) => {
                        dataset.look_for_illegals(&illegals, cmd.verbose(), cmd.utc())
                    }
                },
                Err(e) => println!("{e}"),
            }
        }
    }
    Ok(())
}
