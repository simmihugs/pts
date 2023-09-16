mod pts_loader;
mod commandline;

use commandline::{CommandlineOptions::*, Commandline, ProgrammMode::*};
use pts_loader::dataset::DataSet;

fn main() -> std::io::Result<()> {
    match Commandline::parse() {
	StopRightHere => Commandline::print_help(),
	Repl => println!("{}", "run interactively"),
	SingleFile(cmd) => {
	    let filename = cmd.filename();
	    match DataSet::init(filename) {
		Ok (dataset) => {
		    match cmd.options() {
			SiErrors => dataset.print_si_errors(cmd.verbose()),
			VaEventLogoErrors => println!("Show me all logo errors"),
			GrepIllegalEvents(..) => println!("Grep for illegal events"),
		    }
		}
		Err(e) => println!("{e}"),
	    }
	}
    }

    Ok(())
}

