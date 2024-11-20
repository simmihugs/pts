use crate::pts_loader::sistandard::*;
use csv::Reader;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::process::{Command, Output};

pub fn download_fluid_data_base(file_name: &str) -> Result<String, Box<dyn Error>> {
    println!("Trying to download fluid database");
    let venv_activation_script = if cfg!(target_os = "windows") {
        r"fluid_downloader\venv\Scripts\activate.bat" // Use .bat for Windows
    } else {
        r"source fluid_downloader\venv\bin\activate"
    };

    let output: Output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            //.args(&["/C", &venv_activation_script, "&&", "python", "-m", "fluid_downloader.app", "-o", file_name])
            .args(&[
                "/C",
                venv_activation_script,
                "&&",
                "python",
                "-m",
                "fluid_downloader.app",
                "-o",
                file_name,
            ])
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "{} && python -m fluid_downloader.app -o {}",
                venv_activation_script, file_name
            ))
            .output()?
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    //println!("stdout: {:?}", stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stderr: {:?}", stderr);

    let mut res = String::from("");
    for line in stdout.lines() {
        if line.contains(file_name) {
            res = String::from(line);
        }
    }
    Ok(res)
}

fn read_csv(path: &String) -> Option<Reader<File>> {
    let mut result = None;
    for delimiter in vec![b';', b',', b'\t'] {
        let rdr = ReaderBuilder::new().delimiter(delimiter).from_path(path);
        match rdr {
            Err(err) => {
                println!("{}: {}", "Error occured", err);
                break;
            }
            Ok(mut rdr) => {
                let headers = rdr.headers().ok()?;
                if headers.get(0)? == "Title" {
                    result = Some(rdr);
                    break;
                }
            }
        }
    }

    result
}

fn load_database(path: &String) -> Vec<HashMap<String, String>> {
    let mut vec = Vec::new();
    let rdr = read_csv(&path);

    match rdr {
        Some(mut rdr) => {
            for result in rdr.deserialize() {
                match result {
                    Err(..) => (),
                    Ok(result) => {
                        let record: HashMap<String, String> = result;
                        vec.push(record);
                    }
                }
            }
        }
        _ => (),
    }

    vec
}

pub struct Fluid {
    database: Vec<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
struct ContentDuration {
    #[serde(deserialize_with = "duration_from_str")]
    duration: i64,
}

impl Fluid {
    pub fn init() -> Self {
        Fluid { database: vec![] }
    }

    pub fn size(&self) -> usize {
        self.database.len()
    }

    pub fn load(&mut self, path: String) {
        self.database = load_database(&path);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if *&self.database.len() >= 3 {
            for e in &self.database[0..3] {
                println!("{:?}", e);
            }
        } else {
            println!("Error: Database has length {}", *&self.database.len());
        }
    }

    pub fn list_line(&self, line_index: usize) {
        if self.database.len() > 0 {
            let columns = vec![
                "Title",
                "Res.",
                "Registration",
                "RuntimeMs",
                "Format",
                "Class",
                "ContentId",
                "MaterialId",
                "Filename",
                "Customer",
                "ContentOwner",
                "InDustbin",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

            for (index, column) in columns.iter().enumerate() {
                print!("{} = {:?}, ", column, &self.database[line_index][column]);
                if index == columns.len() - 1 {
                    println!("");
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn query(&self, id: &str, title: &str) -> Option<String> {
        if 0 < self.database.len() && self.database.len() < 50 {
            for i in 0..&self.database.len() - 1 {
                self.list_line(i);
            }
            panic!("something with the csv does not work");
        }
        for entry in &self.database {
            if entry["Title"].contains(title)
                || title.contains(&entry["Title"])
                || entry["ContentId"].contains(id)
                || id.contains(&entry["ContentId"])
            {
                return Some(format!("{}", entry["Filename"]));
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn query_duration(&self, id: &str) -> Option<i64> {
        for entry in &self.database {
            if entry["ContentId"] == id {
                let duration_str = &entry["RuntimeMs"].to_string();
                let parts: Vec<&str> = duration_str.split(".").collect::<Vec<&str>>();
                let hours: String = parts[0].to_string();
                let ms: String = format!("{}", parts[1].parse::<i32>().unwrap());
                let new_duration_str = format!("00 {}.{}", hours, ms);

                return match serde_xml_rs::from_str::<ContentDuration>(&format!(
                    "<ContentDuration duration=\"{}\"></ContentDuration>",
                    new_duration_str
                )) {
                    Err(..) => None,
                    Ok(d) => Some(d.duration),
                };
            }
        }
        return None;
    }
}
