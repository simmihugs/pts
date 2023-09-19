use super::commandline::Commandline;
use crate::pts_loader::dataset::DataSet;
use std::io::Write;

#[derive(Clone)]
pub struct Repl {}

impl Repl {
    fn print_help() {
        println!(":h | :help --> print help");
        println!(":q | :quit --> quit");
        println!(":l | :list --> list loaded files");
        println!(":f <FILE>| :load <FILE> --> try loading new file");
    }

    pub fn parse<'a>(
        cmd: &'a Commandline,
        user_io: String,
        files: &mut Vec<DataSet>,
    ) -> Result<String, ()> {
        let help: Vec<String> = [":help", ":h", "-h", "-help"]
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let quit: Vec<String> = [":quit", ":q", "-q", "-quit"]
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let load: Vec<String> = [":load", ":f", "-f ", "-load"]
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let list: Vec<String> = [":list", ":l", "-l", "-list"]
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let all: Vec<String> = [":all", ":a", "-a", "-all"]
            .iter()
            .map(|x| String::from(*x))
            .collect();

        let user_io_clean = user_io
            .replace("\r", "")
            .replace("\n", "")
            .trim_end()
            .to_string();
        let words = user_io_clean
            .split(' ')
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        if help.contains(&user_io_clean) {
            Repl::print_help();
            Ok("help".to_string())
        } else if list.contains(&user_io_clean) {
            for file in files {
                println!("{}", file.get_filename());
            }
            Ok("listed files".to_string())
        } else if words.len() > 1 {
            match words.iter().position(|x| load.contains(x)) {
                Some(index) => {
                    if words.len() > index + 1 {
                        let new_file = &words[index + 1];
                        if !files
                            .iter()
                            .map(|x| String::from(x.get_filename()))
                            .collect::<Vec<String>>()
                            .contains(&new_file)
                        {
                            match DataSet::init(&new_file) {
                                Err(e) => println!("{}", e),
                                Ok(data) => {
                                    println!("Loaded file: {}", new_file);
                                    files.push(data);
                                }
                            }
                        } else {
                            println!("file {} is allready loaded ", new_file);
                        }
                    }
                }
                None => (),
            }
            match words.iter().position(|x| all.contains(x)) {
                Some(index) => {
                    if words.len() > index + 1 {
                        match words[index + 1].parse::<i32>() {
                            Ok(file_index) => {
                                let _index: usize = file_index.try_into().unwrap();
                                if files.len() > _index {
                                    files[_index].print_si_errors(cmd.verbose(), cmd.utc());
                                    files[_index].print_special_events(cmd.verbose(), cmd.utc());
                                } else {
                                    println!("invalid index {}", file_index);
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                }
                None => (),
            }
            Ok("".to_string())
        } else if quit.contains(&user_io_clean) {
            Err(())
        } else {
            Ok(format!("{}", user_io_clean))
        }
    }

    pub fn start(cmd: &Commandline, _dataset: &DataSet) {
        use std::io;
        let mut user_io = String::new();
        let dataset = _dataset.clone();
        let mut files = vec![dataset];
        loop {
            io::stdout().write_all(b"> ").expect("Failed to write line");
            io::stdout().flush().expect("flush failed!");
            io::stdin()
                .read_line(&mut user_io)
                .expect("Failed to read line");

            match Repl::parse(&cmd, user_io, &mut files) {
                Ok(..) => (),
                Err(..) => break,
            }
            user_io = "".to_string();
        }
    }
}
