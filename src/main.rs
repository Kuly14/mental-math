use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Square,
}

impl Distribution<Operation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Operation {
        match rng.gen_range(0..=4) {
            0 => Operation::Plus,
            1 => Operation::Minus,
            2 => Operation::Multiply,
            3 => Operation::Divide,
            _ => Operation::Square,
        }
    }
}

fn main() {
    let mut count: i32 = 0;
    loop {
        count += 1;
        let op: Operation = rand::random();
        match op {
            Operation::Plus => plus(),
            Operation::Minus => minus(),
            Operation::Multiply => multiply(),
            Operation::Divide => divide(),
            Operation::Square => square(),
        }
        println!("Count: {}", count);
    }
}

fn plus() {
    let mut rng = rand::thread_rng();
    let n1 = rng.gen_range(1..=1000);
    let n2 = rng.gen_range(1..=1000);

    loop {
        let mut input = String::new();
        println!("{} + {} = ?", n1, n2);
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().parse::<i32>().unwrap() == n1 + n2 {
            print!("\x1B[2J\x1B[1;1H");
            println!("GJ");
            break;
        } else {
            println!("Try Again");
        }
    }
}

fn minus() {
    let mut rng = rand::thread_rng();
    let n1 = rng.gen_range(0..=1000);
    let n2 = rng.gen_range(0..=1000);

    loop {
        let mut input = String::new();
        println!("{} - {} = ?", n1, n2);
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().parse::<i32>().unwrap() == n1 - n2 {
            print!("\x1B[2J\x1B[1;1H");
            println!("GJ");
            break;
        } else {
            println!("Try Again");
        }
    }
}

fn multiply() {
    let mut rng = rand::thread_rng();
    let n1 = rng.gen_range(1..=20);
    let n2 = rng.gen_range(1..=20);

    loop {
        let mut input = String::new();
        println!("{} * {} = ?", n1, n2);
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().parse::<i32>().unwrap() == n1 * n2 {
            print!("\x1B[2J\x1B[1;1H");
            println!("GJ");
            break;
        } else {
            println!("Try Again");
        }
    }
}

fn divide() {
    let mut rng = rand::thread_rng();
    let num1;
    let num2;
    loop {
        let n1 = rng.gen_range(0..=1000);
        let n2 = rng.gen_range(2..=20);
        if n1 % n2 != 0 {
            continue;
        } else {
            num1 = n1;
            num2 = n2;
            break;
        }
    }

    loop {
        let mut input = String::new();
        println!("{} / {} = ?", num1, num2);
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().parse::<i32>().unwrap() == num1 / num2 {
            print!("\x1B[2J\x1B[1;1H");
            println!("GJ");
            break;
        } else {
            println!("Try Again");
        }
    }
}

fn square() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..=20);

    loop {
        let mut input = String::new();
        println!("{} ^ 2 = ?", num);
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().parse::<i32>().unwrap() == num * num {
            print!("\x1B[2J\x1B[1;1H");
            println!("GJ");
            break;
        } else {
            println!("Try Again");
        }
    }
}
