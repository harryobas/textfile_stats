use magnus::{define_module, function, prelude::*, Error, exception};
use std::io::{BufRead, BufReader, Read};
use std::fs::{File, self};
use content_inspector::{inspect, ContentType};
use stopwords::{Language, Spark, Stopwords};
use std::collections::HashSet;


fn words_count(file_path: String) -> Result<u32, Error> {

    let file = File::open(file_path.clone())
        .map_err(
            |e| Error::new(
                exception::io_error(), 
                e.to_string()
            )
        )?;

    
    let f_size = file_size(file_path.clone())?;
            
    if f_size == 0 {
        return Err(
            Error::new(exception::standard_error(),
             "File is empty".to_string()))
    }

    let is_binary = is_binary_file(file_path.clone());

    if is_binary {
        return Err(Error::new(
            exception::standard_error(), 
            "Not a text file".to_string()
        ))
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
    let file = File::open(file_path.clone())
        .map_err(
            |e| Error::new(
                exception::io_error(), 
                e.to_string()
            )
        )?;

    let f_size = file_size(file_path.clone())?;
    
    if f_size == 0 {
        return Err(
            Error::new(exception::standard_error(),
             "File is empty".to_string()))
    }

    if is_binary_file(file_path.clone()) {
        return Err(
            Error::new(exception::standard_error(),
             "Not a text file".to_string()))
    }

    let reader = BufReader::new(file);
    let mut count = 0u32;

    for line in reader.lines() {
        if let Ok(line) = line  {
            line
            .split_whitespace()
            .for_each(|w| count += w.chars().count() as u32);
                    
        }
    }

    Ok(count)

}

fn lines_count(file_path: String) -> Result<u32, Error> {
    
    let file = File::open(file_path.clone())
    .map_err(
        |e| Error::new(
            exception::io_error(), 
            e.to_string()
        )
    )?;
    
    let f_size = file_size(file_path.clone())?;

    if f_size == 0 {
        return Err(
            Error::new(exception::standard_error(),
             "File is empty".to_string()))
    }

    if is_binary_file(file_path.clone()) {
        return Err(
            Error::new(exception::standard_error(),
             "Not a text file".to_string()))
    }

    let reader = BufReader::new(file);
    let mut count = 0u32;

    for line in reader.lines() {
        if let Ok(_line) = line {
            count += 1;
        }
    }

    Ok(count)
}

fn word_count(file_path: String, word: String) -> Result<u32, Error> {
    let file = File::open(file_path.clone())
    .map_err(
        |e| Error::new(
            exception::io_error(), 
            e.to_string()
        )
    )?;

    let f_size = file_size(file_path.clone())?;

    if f_size == 0 {
        return Err(
            Error::new(exception::standard_error(),
             "File is empty".to_string()))
        
    }

    if is_binary_file(file_path.clone()) {
        return Err(
            Error::new(exception::standard_error(),
             "Not a text file".to_string()))
        
    }

    let reader = BufReader::new(file);
    let mut count = 0u32;

    for line in reader.lines() {
        if let Ok(line) = line {
            count += line.split_whitespace().filter(|w| w == &word).count() as u32;
        }
    }

   Ok (count)
}

fn char_count(file_path: String, chracter: char) -> Result<u32, Error> {
    let file = File::open(file_path.clone())
    .map_err(
        |e| Error::new(
            exception::io_error(), 
            e.to_string()
        )
    )?;

    let f_size = file_size(file_path.clone())?;

    if f_size == 0 {
        return Err(
            Error::new(exception::standard_error(),
             "File is empty".to_string()))
        
    }

    if is_binary_file(file_path.clone()) {
        return Err(
            Error::new(exception::standard_error(),
             "Not a text file".to_string()))
        
    }

    let reader = BufReader::new(file);
    let mut count = 0u32;

    for line in reader.lines() {
        if let Ok(line) = line {
            line
            .split_whitespace()
            .for_each(|w| count += w.chars().filter(|c| c == &chracter).count() as u32);
        }
    
    }

    Ok(count)
}

fn extract_keywords(file_path: String) -> Result<Vec<String>, Error> {

    let text = fs::read_to_string(file_path.clone())
        .map_err(
            |e|Error::new(
                exception::io_error(), e.to_string())
            )?;

    let f_size = file_size(file_path.clone())?;

    if f_size == 0 {
        return Err(
            Error::new(exception::standard_error(), 
            "File is empty".to_string()))
    }

    if is_binary_file(file_path.clone()) {
        return Err(
            Error::new(exception::standard_error(),
             "Not a text file".to_string()))
    }

    let text_tokens = text
        .split_whitespace()
        .map(|word|word.to_lowercase())
        .collect::<Vec<String>>();
    let words = text_tokens 
        .iter()
        .map(|word|word.as_str())
        .collect::<Vec<&str>>();
    
    let stopwords = Spark::stopwords(Language::English).unwrap();
    let mut frequencies = std::collections::HashMap::new();

    for word in words {
        if!stopwords.contains(&word) {
            *frequencies.entry(word).or_insert(0) += 1;
        }
    }

    let mut freq_vec: Vec<(&str, i32)> = frequencies.into_iter().collect();
    freq_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let freq_vec:Vec<String> = freq_vec
        .iter()
        .map(|(word, _)| word.to_string())
        .collect();

    Ok(freq_vec)



}

fn file_size(file_path: String) -> Result<u64, Error> {
    fs::metadata(file_path)
    .map(|metadata| metadata.len())
    .map_err(|e| Error::new(exception::io_error(), e.to_string()))
}

fn is_binary_file(file_path: String) -> bool {

    let mut file = File::open(file_path).unwrap();
    let mut buffer = vec![];

    file.read_to_end(&mut buffer).unwrap();

    match inspect(&buffer) {
        ContentType::BINARY => true,
        _ => false
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("TextfileStats")?;
    module.define_singleton_method("words_count", function!(words_count, 1))?;
    module.define_singleton_method("chars_count", function!(chars_count, 1))?;
    module.define_singleton_method("lines_count", function!(lines_count, 1))?;
    module.define_singleton_method("word_count", function!(word_count, 2))?;
    module.define_singleton_method("char_count", function!(char_count, 2))?;
    module.define_singleton_method("extract_keywords", function!(extract_keywords, 1))?;
    module.define_private_method("file_size", function!(file_size, 1))?;
    module.define_private_method("is_binary_file", function!(is_binary_file, 1))?;
    Ok(())
}
