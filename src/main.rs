use std::io;



fn plus(a: f64, b: f64) -> f64 {
    a + b
}

fn minus(a: f64, b: f64) -> f64 {
    a - b
}

fn deriver(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        return a / b
    } else {
        println!("Вы долбаеб");
        return 0.0;
    }
}

fn multiply(a: f64, b: f64) -> f64 {
    return a * b

}

fn calc() -> u8 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error incorrect input");
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        println!("Спасибо");
        return 2;
    }

    let mut anum: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Вы ввели хуйню");
            return 1;
        }

    };

    let mut action: char = match parts[1].parse() {
        Ok(char) => char,
        Err(_) => {
            println!("Вы ввели хуйню");
            return 1;
        }
    };


    let mut bnum: f64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Вы ввели хуйню");
            return 1;
        }
    };
    match action {
        '+' => println!("{}",plus(anum, bnum)),
        '-' => println!("{}",minus(anum, bnum)),
        '/' => println!("{}",deriver(anum, bnum)),
        '*' => println!("{}",multiply(anum, bnum)),
        _ => {
            println!("Вы ввели хуйню");
            return 1;
        }
    };
    return 0;
}

fn main() {
    loop {
        if calc() != 0 {
            break;
        }
    }
}
