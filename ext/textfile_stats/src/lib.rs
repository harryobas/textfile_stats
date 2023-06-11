use magnus::{define_module, function, prelude::*, Error, exception};
use std::io::{BufRead, BufReader};
use std::fs::{File, self};

fn words_count(file_path: String) -> Result<u32, Error> {

    let file = match File::open(file_path.clone()) {
        Ok(file) => file,
        Err(e) => return Err(Error::new(exception::io_error(), e.to_string()))

    };
    
    let f_size = file_size(file_path.clone());

    if f_size == 0 {
        return Err(Error::new(exception::standard_error(), "File is empty".to_string()))
    }

    let reader = BufReader::new(file);
    let mut count = 0u32;

    for line in reader.lines() {
        if let Ok(line) = line {
            count += line.split_whitespace().count() as u32;
        }
    }

    Ok(count)
}

fn chars_count(file_path: String) -> Result<u32, Error> {
    let file = match File::open(file_path.clone()) {
        Ok(file) => file,
        Err(e) => return Err(Error::new(exception::io_error(), e.to_string()))

    };
    
    let f_size = file_size(file_path.clone());

    if f_size == 0 {
        return Err(Error::new(exception::standard_error(), "File is empty".to_string()))
    }

    let reader = BufReader::new(file);
    let mut count = 0u32;

    for line in reader.lines() {
        if let Ok(line) = line  {
            line.split_whitespace().for_each(|w| count += w.chars().count() as u32);
                    
        }
    }

    Ok(count)

}

fn file_size(file_path: String) -> u64 {
    fs::metadata(file_path).unwrap().len()
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("TextfileStats")?;
    module.define_singleton_method("words_count", function!(words_count, 1))?;
    module.define_singleton_method("chars_count", function!(chars_count, 1))?;
    module.define_private_method("file_size", function!(file_size, 1))?;
    Ok(())
}
