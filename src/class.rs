// TODO: Is this enum really needed?
#[derive(Debug)]
pub enum LetterGrade {
    AA,
    BA,
    BB,
    CB,
    CC,
    DC,
    DD,
    FD,
    FF,
}

#[derive(Debug)]
pub struct Class {
    pub code: String,
    pub credit: u32,
    pub letter_grade: LetterGrade,
    pub grade: f32,
    // grade_items: Vec<GradeItem>,
}

impl Class {
    pub fn new(code: &str, credit: u32, letter: &str) -> Class {
        let letter_grade = get_letter_enum(letter);
        let grade = enum_to_float(&letter_grade);
        Class {
            code: code.to_string(),
            credit,
            letter_grade,
            grade
            // grade_items: Vec::new()
        }
    }
}

use std::fmt;

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>8} - {} - {}", self.code, self.credit, self.grade)
    }
}

/* 
NOTE Can't implement external trait for external types. (no Display for Vec<Class>!)
Create a "Classes" type containing Vec<Class> called "Semester"
Maybe a "Year" (or "Term"?) struct containing Vec<Semester>?
*/

fn get_letter_enum(letter_grade : &str) -> LetterGrade {
    match letter_grade {
        "AA" => LetterGrade::AA,
        "BA" => LetterGrade::BA,
        "BB" => LetterGrade::BB,
        "CB" => LetterGrade::CB,
        "CC" => LetterGrade::CC,
        "DC" => LetterGrade::DC,
        "DD" => LetterGrade::DD,
        "FD" => LetterGrade::FD,
        _ => LetterGrade::FF
    }
}

fn enum_to_float(letter_grade : &LetterGrade) -> f32 {
    match letter_grade {
        LetterGrade::AA => 4.0,
        LetterGrade::BA => 3.5,
        LetterGrade::BB => 3.0,
        LetterGrade::CB => 2.5,
        LetterGrade::CC => 2.0,
        LetterGrade::DC => 1.5,
        LetterGrade::DD => 1.0,
        LetterGrade::FD => 0.5,
        LetterGrade::FF => 0.0,
    }
}

/*

#[derive(Debug)]
struct GradeItem {
    grade: u8,
    out_of: u8,
    percent: u8,
}

impl GradeItem {
    fn new(grade: u8, out_of: u8, percent: u8) -> GradeItem {
        GradeItem {
            grade,
            out_of,
            percent
        }
    }
}

*/
