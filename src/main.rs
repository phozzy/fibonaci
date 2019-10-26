use std::io;
fn get_position() -> u32 {
    loop {
        let mut fibonaci = String::new();
        println!("Which fibonaci number do you want?");
        io::stdin().read_line(&mut fibonaci)
            .expect("Failed to read line");
        break match fibonaci.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong value! {}", fibonaci);
                continue;
            },
        };
    }
}
fn fibonaci(position: u32, f0: u32, f1: u32) -> u32 {
    if position == 0 {
        f0
    } else if position == 1 {
        f1
    } else {
        fibonaci(position - 1, f1, f0 + f1)
    }
}
fn main() {
    let position = get_position();
    let fibonaci_number = fibonaci(position, 0, 1);
    println!("Your fibonaci number is {}!", fibonaci_number);
}
