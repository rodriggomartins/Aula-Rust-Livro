use rand:: Rng;
use std::cmp::Ordering;
use std::io;

fn main(){
  println!("Adivinhe o numero");

  let numero_secreto = rand::thread_rng().gen_range(1..=100);

  println!("{numero_secreto}");

  loop {
    println!("Por favor insira um numero: ");

    let mut numero = String::new();

    io::stdin()
      .read_line(&mut numero)
      .expect("Falha ao ler essa linha");

    let numero: u32 = match numero.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("Voce escolheu: {numero} ");

    match numero.cmp(&numero_secreto) {
      Ordering::Less => println!("Numero muito baixo"),
      Ordering::Greater => println!("Numero muito alto"),
      Ordering::Equal => {
        println!("Voce acertou!");
        break;
      }
    }
    
  }
  
}
