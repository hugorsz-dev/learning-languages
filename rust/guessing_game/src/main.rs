use std::io; // Importar librería de Input/Output, todavía se podría usar sin el import con std::io::stdin()
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("¡Deduce el número!");
    loop {
        println! ("Por favor, introduzca un número:");

        // ENTRADA: USUARIO PIDE UN NÚMERO 

        let mut guess = String::new(); // "mut" de mutable. Sin el "mut", será invariable

        io::stdin() // "standard in"
            .read_line (&mut guess) // esto no es una variable, es una referencia
            .expect ("Failed to read line");

        println!("You guessed: {}", guess);

        // Conversión de String a Int

        /*
        Los 'match'  se comportan como swich cases. En este caso, la función parse se degrana en: 
            Si parse() puede devolver el numero, devolverá Ok (num) 
            Si no, Err(_)
        Al evaluar, si el número es correcto, el match devuelve el número. 
        Si no es correcto, 
        */

        let guess: u8 = match guess.trim().parse() {
            Ok(number) => number, 
            Err(_) => {
                println!("WTF, that`s not a number");
                continue
            }
        };

        // GENERACIÓN DE NÚMERO ALEATORIO

        let mut rng = thread_rng(); // Generador de números aleatorios
        let secret_number: u8 = rng.gen_range(0..=100); // 0..100 (0->99) 0..=100 (0->100) es un rango inclusivo

        // COMPARACIÓN 

        /*
        Los  'match' pueden ser como "switch (case)" pero basados en funciones. 
        Primero se plantea la condicion, y luego, la función asociada a la misma. 
        */

        match guess.cmp (&secret_number) { // Deben compararse los mismos tipos
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println! ("Too big!"),
            Ordering::Equal => {
                println! ("You win!");
                break;
            }
        };
    }
}
