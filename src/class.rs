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
    code: String,
    credit: u8,
    letter_grade: Option<LetterGrade>,
    // grade_items: Vec<GradeItem>,
}

impl Class {
    pub fn new(code: &str, credit: u8, letter_grade: Option<LetterGrade>) -> Class {
        Class {
            code: String::from(code),
            credit,
            letter_grade,
            // grade_items: Vec::new()
        }
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
