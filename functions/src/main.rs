use std::io;

fn sum(x: i32, y: i32) -> i32 {
    x+y
}

fn read_integer(input: &str) -> Option<i32>{
    let input: i32 = match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => return None,
    };
    Some(input)

}

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    println!("Type the first number: ");
    io::stdin().read_line(&mut x).expect("Failed to read input");
    
    println!("Type the second number: ");
    io::stdin().read_line(&mut y).expect("Failed to read input");
    
    let x = match read_integer(&x) {
        Some(x) => x,
        None => {println!("Wrong input for x, type integers"); return;}
        };
    
    let y = match read_integer(&y) {
        Some(y) => y,
        None => {println!("Wrong input for y, type integers"); return;}
        };

    println!("{}", sum(x, y));
}
