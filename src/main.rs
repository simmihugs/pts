mod pts_loader;
mod commandline;

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
		    dataset.print_si_errors(cmd.verbose(), cmd.utc());
		}

		if cmd.logoerror() {
		    dataset.print_logo_errors(cmd.verbose(), cmd.utc());
		}

		// if cmd.illegalevents() {
		//     dataset.look_for_illegals(&illegals, cmd.verbose()),
		// }
	    },
	    _ => Commandline::print_help(),
	}
    }
    
    Ok(())
}

