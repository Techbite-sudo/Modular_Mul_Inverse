// std::io is used in order to read from standard-input
use std::io;
// The function euclids_gcd finds the gcd of two integers `a` and `b`,
// and two integers `coeff_a` and `coeff_b`
// such that` ( coeff_a · x) + ( coeff_b · y )` is the greatest common divisor of `coeff_a` and `coeff_b · x`
// using the extended Euclidean algorithm.
fn euclids_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a != 0 {

        let remainder = b % a;

        let (gcd, coeff_a, coeff_b) = euclids_gcd(remainder, a);

        let quotient = b / a;

        let a_new_coeff = coeff_b - (quotient * coeff_a);

        let b_new_coeff = coeff_a;

        return (gcd, a_new_coeff, b_new_coeff);
    }

    return (b, 0, 1);
}

// modular_multiplicative_inverse calculates the [Modular Multiplicative Inverse]
// of an integer `a` such that `a·x ≡ 1 (mod m)`.
// In the event this function will return "None".
// Otherwise, the inverse will be returned wrapped up in a "Some".
// It can be done manually by the following syntax (a.x) mod m = 1 which is interpreted as 
// (a.x) divide by m then you will get a remainder of 1.
// so for you to get an efficient modular multiplicative inverse you have to have two numbers whose gcd is 1(relatively prime to each other)

fn modular_multiplicative_inverse(a: isize, m: isize) -> Option<isize> {
    let (gcd, coeff_a, _) = euclids_gcd(a, m);

    if gcd != 1 {
        return None;
    }

    return Some((coeff_a % m + m) % m);
}


// This is the main function that gets executed
fn main() {
    println!("Please input the two values you would wish to get the Modular Multiplicative Inverse for: \n");
    println!("example: 11 26 \n");

    loop {
        //Reading and collecting inputs from the user
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to capture user's inputs");
    
        let user_inputs: Vec<isize> = data
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        //check whether the user inputs less than 2 values    
        if user_inputs.len() < 2 {
            println!("Please input two values to continue");
            continue;
        }
        //passing the inputs into variables a and b
        let x = user_inputs[0];
        let y = user_inputs[1];

        let answer = modular_multiplicative_inverse(x, y);
        match answer {
            Some(final_answer) => {
                println!("The M.M.I of {x} and {y} ==> {final_answer} \n");
            }
            None => println!("M.M.I can't be computed for {x} mod {y}\n"),
        }
    }
}

