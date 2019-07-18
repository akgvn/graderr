mod class;

use class::Class;
use class::LetterGrade;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let courses = read_and_create();
    println!("{:#?}", courses);
}

fn read_and_create() -> Vec<Class> {
    let f = File::open("grades.txt").unwrap();

    let f = BufReader::new(f);

    let mut data: Vec<Class> = Vec::new();

    // TODO this should be its own fn.
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
            None,
        ));
    }
    data
}
