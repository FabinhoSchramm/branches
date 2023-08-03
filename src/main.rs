fn main() {
   let numero = 3;

   if numero < 5 {
        println!("Codicao verdadeira");
   } else {
        println!("Condicao falsa");
   }
   divisao();
}

fn divisao() {
    let numero = 8;
    if numero % 4 == 0 {
        println!("o numero eh divisivel por 4");
    } else if numero % 3 == 0 {
        println!("o numero eh divisivel por 3");
    } else if numero % 2 == 0 {
        println!("o numero eh divisivel por 2");
    } else {
        println!("o numero nao eh divisivel por 4,3 e 2");
    }
}