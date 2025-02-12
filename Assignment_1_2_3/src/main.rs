//Assignment 1: Temperature Converter
/*
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    let mut f = 32.0; 
    let c = fahrenheit_to_celsius(f);
    println!("{:.2}°F is {:.2}°C", f, c);

    for _count in 1..=5 {
        f += 1.0;
        let c = fahrenheit_to_celsius(f);
        println!("{:.2}°F is {:.2}°C", f, c);
    }
}
*/
///////////////////////////////////////////////////////////////////

//Assignment 2: Number Analyzer
/*
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 7, 15, 22, 30, 5, 18, 42, 9, 27];
    
    for num in numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: is {}", num, if is_even(num) { "even" } else { "odd" });
            println!("{}: FizzBuzz", num);
            println!("");
        } else if num % 3 == 0 {
            println!("{}: is {}", num, if is_even(num) { "even" } else { "odd" });
            println!("{}: Fizz", num);
            println!("");
        } else if num % 5 == 0 {
            println!("{}: is {}", num, if is_even(num) { "even" } else { "odd" });
            println!("{}: Buzz", num);
            println!("");
        } else {
            println!("{}: is {}", num, if is_even(num) { "even" } else { "odd" });
            println!("");
        }
    }

    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of numbers: {}", sum);

    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
*/
///////////////////////////////////////////////////////////////////

//Assignment 3: Guessing Game
/*
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 695; // Hardcoded secret number
    let mut attempts = 0;
    let mut max = 999; // Max value 999
    let mut min = 0; // Min value 0
    let mut guess = (max + min) / 2; // Start with the midpoint

    loop {
        let result = check_guess(guess, secret);
        
        if result == 0 {
            attempts += 1;
            println!("Correct! The number is {}.", secret);
            break;
        } else if result == 1 {
            println!("{}", guess);
            attempts += 1;
            max = guess - 1; // Update max boundary
            guess = (min + max) / 2; // New midpoint
            println!("Too high!");
        } else {
            println!("{}", guess);
            attempts += 1;
            min = guess + 1; // Update min boundary
            guess = (min + max) / 2; // New midpoint
            println!("Too low!");
        }
    }
    
    println!("It took {} attempts to guess the number.", attempts);
}
*/