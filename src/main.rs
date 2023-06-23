extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use ferris_says::say;

use http::Uri;

fn show_win(message : String) {
    let message = String::from(message);
    let width = message.chars().count();
    let mut writer = io::BufWriter::new(io::stdout().lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn bye() {
    println!("Pressione enter para sair...");
    let mut bye = String::new();
    io::stdin().read_line(&mut bye)
            .expect("Falha ao ler entrada");
}

fn main() {
    println!("Advinhe o número!");
    let mut last_try : u32 = 0;
    let mut message : String = String::new();
    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Último palpite {}",&last_try);
        println!("{}", &message);
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        last_try = palpite;

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => message = "Muito baixo".to_string(),
            Ordering::Greater => message = "Muito alto!".to_string(),
            Ordering::Equal => {
                let message = String::from("Acertou miseravi");
                show_win(message);
                bye();

                break;
            }
        }
        
        for _i in 0..30 {
            println!("");  
        }
    }

    let uri = "https://www.rust-lang.org/install.html".parse::<Uri>().unwrap();
    assert_eq!(uri.scheme_str(), Some("https"));
    assert_eq!(uri.host(), Some("www.rust-lang.org"));
    assert_eq!(uri.path(), "/install.html");
}