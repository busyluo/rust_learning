
use std::io::stdin;
use rand;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let guess_num = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("guess a number: ");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("reading from stdin won't fail");

        let num: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. input again.");
                continue
            }
        };

        match num.cmp(&guess_num) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win.");
                break;
            }
        }
    }
}
