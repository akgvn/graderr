mod class;

use class::Class;
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    println!("Welcome to Grade Calc!");

    let courses = read_and_create(&args[1]);

    let mut cum_gpa: f32 = 0.0f32;
    let mut credits: u32 = 0;

    for c in courses {
        println!("{}", c);
        cum_gpa += c.grade * c.credit as f32;
        credits += c.credit;
    }

    cum_gpa /= credits as f32;

    println!("cumGPA = {}, Sum of Credits = {}", cum_gpa, credits)
}

fn read_and_create(path: &String) -> Vec<Class> {
    let f = File::open(path).expect("Grades couldn't be found.");

    let f = BufReader::new(f);

    let mut data: Vec<Class> = Vec::new();

    reader(f, &mut data);

    data
}

fn reader(f: BufReader<File>, data: &mut Vec<Class>) {
    for line in f.lines() {
        let line_data = line.unwrap();

        if line_data.starts_with("-") {
            // Comment line
            continue;
        }

        let mut class_data: Vec<String> = Vec::new();
        for item in line_data.split_whitespace() {
            class_data.push(item.to_string());
        }

        data.push(Class::new(
            &class_data[0],
            class_data[1].parse().unwrap(),
            &class_data[2],
        ));
    }
}
