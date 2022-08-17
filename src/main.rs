use std::{fs::File,io::prelude::*,path::Path,env,process};
use colored::*;

fn main(){
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!("[{} error] : {}",args[0],"No arguments provided".red());
        process::exit(1);
    }
    let filename = &args[1];    
   
    let path = Path::new(filename);
    let display = filename.split("/").collect::<Vec<&str>>();   
    let filename = display[display.len()-1];
    let mut file = match File::open(&path) {
        Err(why) => {
            eprintln!("couldn't open {}: {}", filename, why);
            process::exit(1);
        },
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            eprintln!("couldn't read {}: {}", filename, why);
            process::exit(1);
        },
        Ok(_) => {}
    }
    let mut i:u16 = 1;
    println!("File: {}\n",filename.red());
    for line in s.lines(){
        println!("{}\t{line}",i.to_string().yellow());
        i += 1;
    }
}

#[cfg(test)]
mod Test{

}
