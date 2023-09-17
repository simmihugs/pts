mod commandline;
mod pts_loader;

<<<<<<< HEAD
use commandline::Commandline;
use pts_loader::dataset::DataSet;

fn main() -> std::io::Result<()> {
    let cmd = Commandline::parse();
    if cmd.filename() == "DEFAULT" && !cmd.repl() {
	Commandline::print_help();
    } else {
	let filename = cmd.filename();
	match DataSet::init(filename) {
	    Ok (dataset) => {
		if cmd.sierror() {
		    dataset.print_si_errors(cmd.verbose());
		}

		if cmd.logoerror() {
		    dataset.print_logo_errors(cmd.verbose());
		}

		// if cmd.illegalevents() {
		//     dataset.look_for_illegals(&illegals, cmd.verbose()),
		// }
	    },
	    _ => Commandline::print_help(),
	}
=======
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
>>>>>>> 790aac3e66d56a27fc2db4e3950f49da9a6e7f03
    }
    
    Ok(())
}
