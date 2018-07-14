//#1 listing the files in a vec       #done!
//#2 getting every file obeject       #done!
//#3 and opening each one             #done!
//#4 performing a regular exp in the file. #done
extern crate regex;

use std::io;
use std::path::Path;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;

use regex::Regex;
fn main(){
    let args =  &env::args()
                        .into_iter()                 
                        .skip(1)
                        .take(2)
                        .collect::<Vec<String>>();

    let filename = &args[0];
    let pattren = &args[1];                    
    println!("{:?}", search(filename.to_string(), pattren));
                        /*
    let paths = fs::read_dir("../").unwrap();
    let text_files: Vec<_> = paths.collect();
    */

    //multi files
    /*
    for path in &text_files {
        match path{
            Ok(a) => {
                let f = a.path();
                let p = Path::new(&f);
                match p.extension() {
                    Some(ext) => {
                        if ext == "rs"  {
                            if is_found(&read_file(&f), pattren) {
                                println!("{:?}", p.file_name().unwrap());
                            }
                        }
                    },
                    _ => {}
                }
            },
            Err(_) => println!("Error!"),
        };
    }  
    */      
}

fn search(filename: String, pattren: &str) -> Result<String, &str> {
    if is_found(&read_file(&filename), pattren) {
        Ok(filename)
    } else {
        Err("Error reading file")
    }

}

fn is_found(text: &str, pattren: &str) -> bool {
    let re = Regex::new(pattren).unwrap();
    re.is_match(text)
}


fn read_file(path: &str) -> String{
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    let _c = file.read_to_string(&mut content);
    return content;
}

/*
fn read_file(path: &PathBuf) -> String{
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    let _c = file.read_to_string(&mut content);
    return content;
}
*/





fn _print_result(data: String) {
    println!("{}", data);
    println!("--------------------------");
    println!("--------------------------");
}
















