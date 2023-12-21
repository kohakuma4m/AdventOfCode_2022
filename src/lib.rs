use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::result::Result;

pub mod circuit;
pub mod navigation;
pub mod navigation3d;
pub mod solutions;

#[derive(Debug)]
pub enum Script {
    Generate,
    Run
}

pub struct ScriptConfig {
    pub script_type: Script,
    pub solution_folder: Option<String>,
    pub solution_number: Option<String>
}

impl fmt::Display for ScriptConfig {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(fmt, "{:?} {:?} {:?}", self.script_type, self.solution_folder.as_deref().unwrap(), self.solution_number.as_deref().unwrap_or_default())
    }
}

impl ScriptConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<ScriptConfig, &'static str> {
        args.next(); // Skipping first argument which is the program name...

        // First param: either "generate" or "dayXX"
        let param1 = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing \"(generate|dayXX)\" argument")
        };

        // Second param: either the "dayXX" or "solutionY"
        if param1 == "generate" {
            let param2 = args.next();
            return Ok(ScriptConfig { script_type: Script::Generate, solution_folder: param2, solution_number: None });
        }
        else {
            let param2 = match args.next() {
                Some(arg) => arg,
                None => return Err("Missing \"solutionY\" argument")
            };

            return Ok(ScriptConfig { script_type: Script::Run, solution_folder: Some(param1), solution_number: Some(param2) });
        }
    }
}

/////////////////////////////////////////////////

pub fn find_next_solution_dir() -> Result<Option<String>, Box<dyn Error>> {
    let mut dir_names = fs::read_dir("./src/solutions")
        .unwrap()
        .filter_map(|entry| entry.ok().and_then(|e| e.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s)))))
        .filter(|dir| dir.contains("day"))
        .collect::<Vec<String>>();

    dir_names.sort();

    return match dir_names.last() {
        Some(name) => {
            let next_day_number = name.replace("day", "").parse::<i32>().unwrap() + 1;
            let next_day_folder = format!("day{:02}", next_day_number);
            Ok(Some(next_day_folder))
        },
        None => Ok(None)
    };
}

pub fn is_solution_dir(dir: &str) -> bool {
    let dir_path = format!("./src/solutions/{}", dir);
    Path::new(&dir_path).is_dir()
}

pub fn create_solution_dir(dir: &str) -> Result<(), Box<dyn Error>> {
    let dir_path = format!("./src/solutions/{}", dir);
    fs::create_dir_all(dir_path)?;
    Ok(())
}

pub fn create_solution_files(dir: &str) -> Result<(), Box<dyn Error>> {
    let template_file = format!("./templates/default.rs");
    let target = format!("./src/solutions/{}/mod.rs", dir);
    fs::copy(template_file, target)?;

    let input_file = format!("./src/solutions/{}/input.txt", dir);
    fs::File::create(input_file)?;

    let test_file = format!("./src/solutions/{}/test.txt", dir);
    fs::File::create(test_file)?;
    Ok(())
}

/////////////////////////////////////////////////

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let data = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to read file: {err}");
            return Err(Box::new(err));
        }
    };

    Ok(data)
}
