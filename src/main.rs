//#1 listing the files in a vec       #done!
//#2 getting every file obeject       #done!
//#3 and opening each one             #done!
//#4 performing a regular exp in the file. #done
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

    let pattren = &args[0];
    let filenames =  &args[1..];
         
    for filename in filenames{
        match search(filename, pattren) {
            Ok(file_name) => println!("{}", file_name),
            Err(_) => {}
        }
    }                   
    


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

fn search(filename: &str, pattren: &str) -> Result<String, String> {
    let text = match read_file(filename){
        Ok(t) => t,
        Err(v) => return Err(v.to_string())
    };
    match is_found(&text, pattren) {
        Ok(_) => Ok(filename.to_string()),
        Err(_) => Err("Not found!".to_string()) 
    }

}

fn is_found(text: &str, pattren: &str) -> Result<bool, bool> {
    let re = Regex::new(pattren).unwrap();
    if re.is_match(text)  {
        Ok(true)
    } else {
        Err(false)
    }
}


fn read_file(path: &str) -> Result<String, &str>{
    let mut file = match File::open(path) {
        Ok(f) => f, 
        Err(_) => return Err("Error reading file")
    };

    let mut content = String::new();
    let _c = file.read_to_string(&mut content);
    Ok(content)
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
















