
extern crate regex;
extern crate colored;

use colored::*;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use regex::Regex;
use std::collections::HashMap;


fn main(){
    let args =  env::args()
                        .into_iter()
                        .skip(1)
                        .collect::<Vec<String>>();
    if args.len() >= 2 {
        let pattern = &args[0];
        let filenames =  &args[1..];

        for filename in filenames{
            match search(filename, pattern) {
                Some(result) => {
                    for (fname, lines) in result {
			println!("\n{}: \n", fname);
			for line in lines{
				println!("{}\n", line );
			}
                    }
                },
                None => {}
            }
        }
    } else {
        println!("Usage: refd pattren files...");
    }
}


/// This function takes a file name and pattren and returns the name
/// of the file which contains the text that match the pattren 
fn search(filename: &str, pattren: &str) -> Option<HashMap<String, Vec<String>>> {
    let text = match read_file(filename){
        Ok(t) => t,
        Err(_) => return None
    };

    let lines: Vec<&str> = text.split('\n').collect();
    let mut result= Vec::new();
    let mut result_hash = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        //println!("{}: {}", i, line);
        if let Some(value) = is_found(&line, pattren) {
            let words: Vec<&str> = line.split(" ").collect();
            //println!("word: {:?}", words);
           
            let mut colored: Vec<String> = vec![];
            for word in words.iter() {
                if word.contains(&value) {
                    println!("value: {}", value);
                    colored.push(format!("{}", word.green()));
                    println!("word: {}", word);
                    println!("colored: {:?}", colored);
                } else {
                    colored.push(word.to_string());
                    println!("else word: {}", word);
                    println!("else colored: {:?}", colored);
                }
            }
            result.push(format!("\t{}: {}", i, colored.join(" ")).to_string());
        }

    }
    if result.len() >= 1 {
    	if let None = result_hash.get(&filename.to_string()){
        	result_hash.insert(filename.to_string(), result);
 		}
    }

    Some(result_hash)


}


/// This function takes a text and a pattern and uses regular expression to
/// test if they match
fn is_found(text: &str, pattern: &str) -> Option<String> {
    let re = Regex::new(pattern).unwrap();
    if let Some(result) = re.find(text)  {
        println!("{}", &text[result.start()..result.end()].green()); 
        Some( text[result.start()..result.end()].to_string())
    } else {
        None
    }
}


/// This function reads a file and return it's content
fn read_file(path: &str) -> Result<String, &str>{
    let mut file = match File::open(path) {
        Ok(f) => f, 
        Err(_) => return Err("Error reading file")
    };

    let mut content = String::new();
    let _c = file.read_to_string(&mut content);
    Ok(content)
}

#[test]
fn test_is_found() {
    assert_eq!(is_found("hi there", "hi"), Some(true));
    assert_eq!(is_found("hi there", "hello"), None);
    assert_eq!(is_found("Hi there", "[Hh]i"), Some(true));
}


















