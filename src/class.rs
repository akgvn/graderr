#[derive(Debug)]
pub struct Class {
    code: String,
    credit: u8,
    letter_grade: Option<String>,
    grade_items: Vec<GradeItem>,
}

#[derive(Debug)]
struct GradeItem {
    grade: u8,
    out_of: u8,
    percent: u8,
}

impl Class {
    pub fn new(code: &str, credit: u8) -> Class {
        Class {
            code: String::from(code),
            credit,
            letter_grade: None,
            grade_items: Vec::new()
        }
    }
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