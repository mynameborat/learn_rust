use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    guess_games(secret_number);

    variables();
    control_structures();
}

fn guess_games(secret_number: u32) {
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

fn variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tuple : (i32, char, bool) = (1, 'a', true);

    println!("Values in tuple {} {} {}", tuple.0, tuple.1, tuple.2);

    let array = [1, 2, 3, 4, 5];
    println!("Length of the array {}", array.len());

    println!("Sum of first {} and {} is {}", array[0], array[1], compute_sum(array[0], array[1]));
}

fn control_structures() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("BlastOff!");

    let mut index = 5;

    while index > 0 {
        println!("{}!", index);
        index -= 1;
    }

    println!("BlastOff differently!");

    let cnt = loop {
        index += 1;

        if index > 4 {
            break index * 2;
        }
    };
}

fn compute_sum(x: i32, y: i32) -> i32 {
    match x.overflowing_add(y) {
        (_, true) => -1,
        (result, false) => result
    }
}
