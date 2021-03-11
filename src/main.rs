use std::io;
use std::process::Command;

fn main() {

    loop {
        clear_screen();
        let mut num_1 = String::new();
        let mut num_2 = String::new();
        let mut somma: i8 = 0;
        let mut op = String::new();
        let mut choice: String = String::new();

        println!("First number:");
        io::stdin().read_line(&mut num_1).expect("Error");
        let num_1: i8 = num_1.trim().parse().expect("Error");

        println!("Second number:");
        io::stdin().read_line(&mut num_2).expect("Error");
        let num_2: i8 = num_2.trim().parse().expect("Error");

        println!("Operation [1 = `+` | 2 = `-` | 3 = `*` | 4 = `/`]:");
        io::stdin().read_line(&mut op).expect("Error");
        let op: i8 = op.trim().parse().expect("Error");

        match op {
           1 => somma = num_1 + num_2,
           2 => somma = num_1 - num_2,
           3 => somma = num_1 * num_2,
           4 => somma = num_1 / num_2,
           _ => println!("Error! Operation not valid"),
        }
        clear_screen();
        println!("Result: {}", somma);

        println!("Again? Yes = Enter | No = `n`");
        io::stdin().read_line(&mut choice).expect("Error");
        let choice: String = choice.trim().to_string();
        
        if choice.to_uppercase() == "N" {
            break;
        }

        pause();
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn pause() {
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
