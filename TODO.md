# TODO for Grade Calculator

- [x] Create a Class type
    - [x] Delete the grade item type for initial backwards compatibility.
- [x] Read data from old files and create objects (are they called objects in Rust?).
    - [x] Read course code.
    - [x] Read credit.
    - [x] Read letter grade.
- [ ] Add menu
    - [ ] Operations
        - [ ] Class
            - [ ] Add class
            - [ ] Update class
            - [ ] Select semester(?)
        - [ ] Semester
            - [ ] Add semester
            - [ ] Select classes to add to semester?
- [ ] Add semester data type, holds multiple classes and gpa for that semester.
- [ ] Save data to and read data from JSON files.
- [ ] Build executables for distribution.

## High-level features that'd be nice to have 

- a TUI or a GUI
    - termbox / ncurses for TUI (?)
    - nuklear / fltk / tk-tcl for GUI (?)
- Importance calculator for classes and each exam (or homeworks etc.) using their overall impact on grades.
- Setting a goal GPA and having the software calculate the lowest possible grades to obtain that GPA. 