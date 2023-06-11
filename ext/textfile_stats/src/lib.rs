use magnus::{define_module, function, prelude::*, Error, exception};
use std::io::{BufRead, BufReader};
use std::fs::{File, self};

fn word_count(file_path: String) -> Result<u32, Error> {
    if let Err(e) = File::open(file_path.clone())  {
        return Err(Error::new(exception::io_error(), e.to_string()));    
    }

    let f_size = file_size(file_path.clone());

    if f_size == 0 {
        return Err(Error::new(exception::standard_error(), "File is empty".to_string()))
    }

    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let mut word_count = 0u32;

    for line in reader.lines() {
        if let Ok(line) = line {
            word_count += line.split_whitespace().count() as u32;
        }
    }

    Ok(word_count)
}

fn file_size(file_path: String) -> u64 {
    fs::metadata(file_path).unwrap().len()
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("TextfileStats")?;
    module.define_singleton_method("word_count", function!(word_count, 1))?;
    module.define_private_method("file_size", function!(file_size, 1))?;
    Ok(())
}
