
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use regex::Regex;

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
                Ok(result) => {
                    for r in result {
                        println!("{}\n", r);
                    }
                },
                Err(_) => {}
            }
        }             
    } else {
        println!("Usage: refd pattren files...");
    } 
          
    
}


/// This function takes a file name and pattren and returns the name
/// of the file which contains the text that match the pattren 
fn search(filename: &str, pattren: &str) -> Result<Vec<String>, String> {
    let text = match read_file(filename){
        Ok(t) => t,
        Err(v) => return Err(v.to_string())
    };

    let lines: Vec<&str> = text.split('\n').collect();
    let mut result= Vec::new();
    for (i, line) in lines.iter().enumerate() {
        //println!("{}: {}", i, line);
        if let Ok(_) = is_found(&line, pattren) {
            result.push(format!("{} \n\t{}: {}", filename.to_string(), i, line).to_string()) 
        }
  
    }
    Ok(result)


}


/// This function takes a text and a pattern and uses regular expression to
/// test if they match
fn is_found(text: &str, pattern: &str) -> Result<bool, bool> {
    let re = Regex::new(pattern).unwrap();
    if re.is_match(text)  {
        Ok(true)
    } else {
        Err(false)
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
    assert_eq!(is_found("hi there", "hi"), Ok(true));
    assert_eq!(is_found("hi there", "hello"), Err(false));
    assert_eq!(is_found("Hi there", "[Hh]i"), Ok(true));
}


















