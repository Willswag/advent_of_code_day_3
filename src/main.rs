
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldnt open {}: {}",display,why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why)=> panic!("could read {}: {}",display,why),
        Ok(_)=> print!("contains:data\n{}",s ),
    }


    let split_sting = s.split("\n");

    let contents = split_sting.collect::<Vec<&str>>();

    for cont in contents{
        get_first_half(cont);
    }
}

fn get_first_half(target: &str ) -> String{
    let mut items = String::new();
    for (i,ch) in target.chars().enumerate(){
        //println!("{} {}", i, ch);
        if i < target.len()/2 {
            items.push(ch);
        }
    }
    println!("{}", items);
    return items;
    
}