# TODO for Grade Calculator

- [x] Create a Class type
    - [x] Delete the grade item type for initial backwards compatibility.
- [x] Read data from old files and create objects (are they called objects in Rust?).
    - [x] Read course code.
    - [x] Read credit.
    - [x] Read letter grade.
- [ ] Add CLI arguments
    - [x] "gr grades.txt" should read "grades.txt" and output cGPA
    - [ ] "gr grades.txt -g 3.95" should calculate the credits and grades needed to obtain a cGPA of 3.95
