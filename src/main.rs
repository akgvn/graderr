mod class;

use class::Class;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Please specify the text file!");
        return;
    } else {
        let courses = read_and_create(&args[1]);
        let (gpa, credits) = calc_and_print_classes(&courses);
        println!("GPA = {}, Sum of Credits = {}", gpa, credits);

        if args.len() == 4 {
            let goal: f32 = args[3].parse().unwrap(); // Goal GPA
            let (additional_credits, additional_grades) = goal_calc(goal, gpa, credits);
            println!(
                "You need additional {} credits with grade of {}!",
                additional_credits, additional_grades
            );
        }
    }
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
        let line_data = line.unwrap().trim().to_string();

        if line_data.starts_with("-") {
            // Comment line
            continue;
        }

        let mut class_data: Vec<String> = Vec::new();
        for item in line_data.split_whitespace() {
            class_data.push(item.to_string());
        }

        if class_data.len() >= 3 {
            data.push(Class::new(
                &class_data[0],
                class_data[1].parse().unwrap(),
                &class_data[2],
            ));
        }
    }
}

fn calc_and_print_classes(classes: &Vec<Class>) -> (f32, u32) {
    let mut gpa: f32 = 0.0f32;
    let mut credits: u32 = 0;

    for c in classes {
        println!("{}", c);
        gpa += c.grade * c.credit as f32;
        credits += c.credit;
    }
    gpa /= credits as f32;

    (gpa, credits)
}

fn goal_calc(goal: f32, gpa: f32, tot_creds: u32) -> (u32, f32) {
    // Formula:
    // gpa * tot_creds = ac * (goal*tot_creds - ag)
    let ac: u32; // additional credits can be 3 or 4
    let ag: f32; // additional grades can be [0, 0.5, 1, ..., 3, 3.5, 4] (from FF to AA)

    let left_hand_side = gpa * tot_creds as f32;
    let right_hand_constant = goal * tot_creds as f32;

    todo!();
    (ac, ag)
}
