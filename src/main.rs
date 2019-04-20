
extern crate regex;
extern crate colored;

use colored::*;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::env;
use regex::Regex;
use std::thread;



#[derive(Debug)]
struct Search {
    filename: String,
    pattren: String
}
impl Search {
    fn new(filename: String, pattren: String) -> Search{
        Search{
            filename: filename,
            pattren: pattren
        }
    }

    /// This method takes a file name and a pattren and returns a list
    /// of the lines that contain the matched words 
    fn search(&self) -> Option<Vec<String>>{

        let text = match read_file(self.filename.to_string()){
            Ok(t) => t,
            Err(why) => panic!(why)
        };

        let lines: Vec<&str> = text.split('\n').collect();

        let mut result: Vec<String>= Vec::new();
        /*Searching for matches*/
        for (i, line) in lines.iter().enumerate() {
            if let Some(value) = match_found(&line, &self.pattren) {
                let words: Vec<&str> = line.split(" ").collect(); 
                let mut colored: Vec<String> = vec![];
                /*Coloring the matched words*/
                for word in words.iter() {
                    if word.contains(&value) {
                        colored.push(format!("{}{}", word[0..value.len()].red(), &word[value.len()..]));
                    } else {
                        colored.push(word.to_string());
                    }
                }
                /*Adding the matched lines with the colored words to the "result" Vec*/
                result.push(format!("\t{}: {}", i, colored.join(" ")).to_string());
            }
        }
        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
        
    }
}


/// This function reads a file and return it's content
fn read_file(path: String) -> Result<String, std::io::Error>{
    let mut file = match File::open(&path) {
        Ok(f) => f, 
        Err(why) => return Err(why)
    };

    let mut content = String::new();
    let _c = file.read_to_string(&mut content);
    Ok(content)
}


/// This function takes a text and a pattern and uses regular expression to
/// test if they match
fn match_found(text: &str, pattern: &str) -> Option<String> {
    let re = Regex::new(pattern).unwrap();
    if let Some(result) = re.find(text)  { 
        Some( text[result.start()..result.end()].to_string())
    } else {
        None
    }
}


fn main(){
    let args =  env::args()
                        .into_iter()
                        .skip(1)
                        .collect::<Vec<String>>();
   

    if args.len() >= 2 {
        let pattern = &args[0];
        let filenames =  &args[1..];

        let mut searches: Vec<Search> = Vec::new();
        for fname in filenames{
            searches.push(Search::new(fname.to_string(), pattern.to_string()));
        }
        let mut handles = vec![];
        for s in searches.into_iter() {
            
            handles.push(thread::spawn(move || {
                let result = match s.search() {
                    Some(v) => v,
                    None => vec!()
                };
                println!("{}", s.filename.green());
                for m in result.iter() {
                    println!("{}", m);
                }
                println!("");
                
            }));
        }
        for handle in handles.into_iter(){
            handle.join().unwrap();
        }
        

    }
    
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_search(){
		fs::write("test", "something" ).expect("Could not write to the config file.");
		let s = Search::new("Something".to_string(), "test".to_string());
		let expected = Some(vec!("Something".to_string()));
		assert_eq!(expected, s.search());
	}

	#[test]
	fn test_is_found() {
	    assert_eq!(match_found("hi there", "hi"), Some("hi".to_string()));
	    assert_eq!(match_found("hi there", "hello"), None);
	    assert_eq!(match_found("Hi there", "[Hh]i"), Some("Hi".to_string()));
	}

}





















