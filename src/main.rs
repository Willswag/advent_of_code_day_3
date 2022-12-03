
use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;
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
        Ok(_)=> print!("contains:\n{}",s ),
    }


    let split_sting = s.split("\n");

    let contents = split_sting.collect::<Vec<&str>>();
    println!();
    println!();

    // let mut total_value =0;
    // for cont in contents{
    //     println!("{}",cont);
    //     let container1 = get_first_half(cont);
    //     let container2 = get_second_half(cont);
    //     total_value = total_value + get_matching_data(&container1, &container2);

    //     println!();
    // }

    // println!("total value {}",total_value);

    let mut i =0;
    let mut valsum = 0;
    while i < contents.len(){
        let s = contents.index(i);

        println!("{}",s);
        let s2 = contents.index(i+1);
        println!("{}",s2);

        let s3 = contents.index(i+2);
        println!("{}",s3);

        valsum = valsum + get_common_item(s, &s2, &s3);
        i = i+3;
        println!();
    }
    println!("the sum of the vals is {}",valsum);


}

fn get_common_item(bag1:&str,bag2:&str,bag3:&str) -> i32{
    let mut val = 0;
    for ch in bag1.chars(){
        if bag2.contains(ch) && bag3.contains(ch){
            println!("found matching item {}", ch);
            val = get_value(ch);
        }
    }
    return val;
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
fn get_second_half(target: &str ) -> String{
    let mut items = String::new();
    for (i,ch) in target.chars().enumerate(){
        //println!("{} {}", i, ch);
        if i >= target.len()/2 {
            items.push(ch);
        }
    }
    println!("{}", items);
    return items;
    
}

fn get_matching_data(comp1: &str,comp2: &str) ->i32{
    let mut prev_match = String::new();
    let mut values = 0; 
    for ch in comp1.chars()
    {
        if comp2.contains(ch) && !prev_match.contains(ch){
            println!("match {}",ch);
            values = get_value(ch);
            prev_match.push(ch);
        }
    }
    return values;
}

fn get_value(item: char) -> i32 {
    let mut val = 0;
    if item.is_ascii_uppercase(){
        val = item as i32 - 65 + 27;
        println!("{}", val);
    }else if item.is_ascii_lowercase() {
        val = item as i32 - 96;
        println!("{}", val);
    }
    return val;
}