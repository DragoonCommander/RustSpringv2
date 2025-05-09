/*
// Define our data structure
struct Data {
    value: i32,
}

// Higher-order function: defines what needs to be done
fn process_data(data: &mut [Data], operation: fn(&mut Data)) {
    for item in data.iter_mut() {
        operation(item);
    }
}

// Specific operations: actual functions which do the work
fn double_value(data: &mut Data) {
    data.value *= 2;
}

fn square_value(data: &mut Data) {
    data.value = data.value * data.value;
}

// Helper function to print values without closures
fn print_values(items: &[Data]) {
    print!("Values: ");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", item.value);
    }
    println!();
}

fn main() {
    let mut items = vec![
        Data { value: 1 },
        Data { value: 2 },
        Data { value: 3 },
        Data { value: 4 },
        Data { value: 5 },
    ];
    
    // The specific operation is decided here
    print!("Original ");
    print_values(&items);
    
    process_data(&mut items, double_value);
    print!("After doubling: ");
    print_values(&items);
    
    // We can easily switch to a different operation
    process_data(&mut items, square_value);
    print!("After squaring: ");
    print_values(&items);
}
*/


//In class Assignment

// Create a struct Student (major)
struct Student {
    major: String,
}

// First order functions, assig_major(student, major_declared)
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

// Higher order functions update major
fn update_majors(collection: &mut Vec<Student>, behavior: fn(&mut Student, String)) {
    let new_major = String::from("Computer Science");

    for student in collection.iter_mut() {
        behavior(student, new_major.clone());
    }
}

fn main() {
    //Create a vector of students 1,2,3 and update all students major
    let mut students = vec![
        Student { major: "Computer Engineering".to_string() },
        Student { major: "Electrical Engineering".to_string() },
        Student { major: "Mechanical Engineering".to_string() },
    ];
    
    // Print majors
    println!("Prior Update:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: Major = {}", i + 1, student.major);
    }
    println!("");

    // Update majors using the higher-order function
    update_majors(&mut students, assign_major);

    println!("After Update:");
    // Print updated majors to confirm
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: Major = {}", i + 1, student.major);
    }
}
