extern crate chrono;
use chrono :: {DateTime, Utc};
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Person{
    first_name : String,
    last_name : String,
    date_of_birth : String
}

pub fn functions(){
    open_file();
}

pub fn open_file(){
    let args:Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];
    read_file(input,output);
}

pub fn read_file(i:&str , o:&str){
    let file = File::open(i).expect("Path does not already exist !");
    let mut new_file = File::create(o).expect("Output file could not be created");
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line.expect("Line Not Found !");
        let words:Vec<&str> = line.split(",").collect();
        task_file(words,&mut new_file);
    }
}

pub fn task_file(tokens:Vec<&str>,output_file:&mut std::fs::File){
    for i in 1..=tokens.len(){
        if i<2 {
            let p = Person {
                first_name : tokens[i-1].to_string(),
                last_name : tokens[i].to_string(),
                date_of_birth : tokens[i+1].to_string()
            };
            let age = birth_date(p.date_of_birth);
            let current_age = age.to_string();
            write_file(p.first_name,p.last_name,current_age,output_file);
        }
    }
}    

fn birth_date(dob : String) -> i32{
    let bday : Vec<&str> = dob.split("-").collect();
    let mut birth_year = bday[0].to_string().parse::<i32>().expect("bday[0] couldn't found");
    let mut birth_month = bday[1].to_string().parse::<i32>().expect("bday[1] couldn't found");
    let mut birth_day = bday[2].to_string().parse::<i32>().expect("bday[2] couldn't found");
    let age_now = age(birth_year,birth_month,birth_day);
    return age_now;
}

fn age(y:i32, m:i32, mut d:i32) -> i32{
    let now:DateTime<Utc> = Utc::now();
    let this_day = now.format("%Y-%m-%d").to_string();
    let today:Vec<&str> = this_day.split("-").collect();
    let mut yyyy = today[0].parse::<i32>().expect("today[0] not found");
    let mut mm = today[1].parse::<i32>().expect("today[1] not found");
    let mut dd = today[2].parse::<i32>().expect("today[2] not found");
    let month : [i32;12] = [31,28,31,30,31,30,31,31,30,31,30,31];
    if d>dd {
        mm = mm - 1;
    }
    if m > mm {
       yyyy = yyyy -1;
    }
    let age = yyyy - y;
    return age;
}

fn write_file(fname:String, lname:String, age:String,output_file:&mut std::fs::File){
    output_file.write_all(lname.as_bytes()).expect("`lname` couldn't write into file");
    output_file.write_all(",".as_bytes()).expect("`,` couldn't write into file");
    output_file.write_all(fname.as_bytes()).expect("`fname` couldn't write into file");
    output_file.write_all(",".as_bytes()).expect("`,` couldn't write into file");
    output_file.write_all(age.as_bytes()).expect("`age` couldn't write into file");
    output_file.write_all(b"\n");
}       
