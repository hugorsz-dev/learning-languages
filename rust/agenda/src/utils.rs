pub struct Cin;
pub struct Menu;
pub struct StringUtils;

impl StringUtils {
    pub fn utf8_slice<'a>(&self, utf8_str: &'a str, start_char: usize, end_char: usize) -> &'a str {
        let mut indices = utf8_str.char_indices().map(|(i, _)| i);
        let start = indices.nth(start_char).unwrap();
        let end = indices
            .nth(end_char - start_char - 1)
            .unwrap_or(utf8_str.len());
        &utf8_str[start..end]
    }
}

impl Cin {
    pub fn string(&self) -> String {
        let mut input = String::with_capacity(50_000); // esta estructura, a diferencia de let mut selection ="", permite vivir en la memoria dinámica

        std::io::stdin()
            .read_line(&mut input) // esto no es una variable, es una referencia
            .expect("Failed to read line");

        input
    }

    pub fn i8(&self) -> i8 {
        loop {
            let input = self.string();

            match input.trim().parse() {
                Ok(number) => {
                    return number;
                }
                Err(_) => {
                    println!("You have not entered a i8 number!");
                }
            };
        }
    }

    pub fn u32(&self) -> u32 {
        loop {
            let input = self.string();

            match input.trim().parse() {
                Ok(number) => {
                    return number;
                }
                Err(_) => {
                    println!("You have not entered a u32 number!");
                }
            };
        }
    }

    pub fn i32(&self) -> i32 {
        loop {
            let input = self.string();

            match input.trim().parse() {
                Ok(number) => {
                    return number;
                }
                Err(_) => {
                    println!("You have not entered a u32 number!");
                }
            };
        }
    }
    
    
    pub fn u128(&self) -> u128 {
        loop {
            let input = self.string();

            match input.trim().parse() {
                Ok(number) => {
                    return number;
                }
                Err(_) => {
                    println!("You have not entered a i8 number!");
                }
            };
        }
    }

    pub fn usize(&self) -> usize {
        loop {
            let input = self.string();

            match input.trim().parse() {
                Ok(number) => {
                    return number;
                }
                Err(_) => {
                    println!("You have not entered a i8 number!");
                }
            };
        }
    }

    pub fn f64(&self) -> f64 {
        loop {
            let input = self.string();

            match input.trim().parse() {
                Ok(number) => {
                    return number;
                }
                Err(_) => {
                    println!("You have not entered a f64 number!");
                }
            };
        }
    }
}

impl Menu {
    pub fn display_menu(menu_options: &[&str]) -> i8 {
        let mut counter = 0;
        for option in menu_options {
            println!("{counter} {option}");
            counter += 1;
        }
    
        // Entrada de un número para seleccionar en el menú.
    
        let selection: i8 = Cin.i8();
    
        // Comprobación de que el número no excede a la cantidad de selecciones
    
        if selection > menu_options.len() as i8 {
            println!("Your selection exceeds the number of options!");
            return -1;
        }
    
        // Devolver el formato
    
        selection
    }

}





