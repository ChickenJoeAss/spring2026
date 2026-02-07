//Assignment1
const FREEZING_FAHRENHEIT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn ass1() {
    let mut temp_f = FREEZING_FAHRENHEIT;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.0} F = {:.2} C", temp_f, temp_c);

    let mut count = 0;
    while count < 5 {
        temp_f = temp_f + 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{:.0} F = {:.2} C", temp_f, temp_c);
        count = count + 1;
    }

    let back_to_f = celsius_to_fahrenheit(0.0);
    println!("0 C = {:.0} F", back_to_f);
}


//Assignment2
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn ass2() {
    let numbers = [3, 5, 6, 10, 15, 8, 9, 12, 7, 20];

    for num in numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum = sum + numbers[i];
        i = i + 1;
    }
    println!("Sum: {}", sum);

    let mut largest = numbers[0];
    for num in numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}

//Assignment3
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn ass3() {
    let secret = 7;
    let guesses = [3, 10, 5, 7]; 
    let mut count = 0;

    loop {
        let guess = guesses[count];
        count = count + 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {} is correct!", guess);
            break;
        } else if result == 1 {
            println!("Guess {} is too high", guess);
        } else {
            println!("Guess {} is too low", guess);
        }
    }

    println!("It took {} guesses.", count);
}


fn main() {
    ass1();
    ass2();
    ass3();
}