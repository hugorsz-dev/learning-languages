use std::{env::consts::DLL_SUFFIX, net::IpAddr};

fn main() {
    // "IpAddrKind" es el tipo de datos que engloba a su vez dos tipos, V4 y v6 (las direcciones IP)
    enum VersionDeDireccionIp {
        V4,
        V6,
    }

    let cuatro = VersionDeDireccionIp::V4;
    let seis = VersionDeDireccionIp::V6;

    // Creamos un struct indicando el tipo de datos, siendo "tipo" el enum que hemos creado. 
    struct DireccionIp {
        tipo: VersionDeDireccionIp,
        direccion: String
    }

    let localhost = DireccionIp {
        tipo: VersionDeDireccionIp::V4,
        direccion: String::from("192.168.1.132"),
    };

    let loopback = DireccionIp {
        tipo: VersionDeDireccionIp::V6,
        direccion: String::from("::1"),
    };

    // Los enums son como structs clasificables

    // Esto es un poco inútil, en realidad, la verdadera utilidad de los enums es organizativa. Obsérvese cómo los siguientes campos están ordenados:

    enum Color {
        RGB (u8,u8, u8), 
        HEX (String),
        HSL (u8,u8,u8),
        CMYK (u8,u8,u8,u8),
    }

    impl Color {
        fn imprimirColor (&self) { // Lo que entra por el método es precisamente el tipo definido en el enumerado
            match self { // Aquí puede filtrarse con un match del propio self
                Color::RGB(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
                Color::HEX(hex) => println!("HEX(#{})", hex),
                Color::HSL(h, s, l) => println!("HSL({}, {}%, {}%)", h, s, l),
                Color::CMYK(c, m, y, k) => println!("CMYK({}, {}, {}, {})", c, m, y, k),            
             }
        }
    }

    let green = Color::RGB(34, 139, 34);
    let red = Color::HEX(String::from("FF0000"));
    green.imprimirColor();

    // Presentemos otro ejemplo menos concreto

    enum MarioBros {
        Vivo (bool),
        Mover {x: i32, y: i32}, // varias variables en una
        Saltar(String),
        Crecer(String),
        Vidas(u8),
        Morir
    }



    // El Option <> es una forma de evitar errores de nulos
    // Si el valor no existe porque el usuario no lo ha introducido ¡no será problema al procesarse
    // En definitiva, "some" es una variable que puede contener un valor, o no... y es un enumerado del sistema

    let mut valor = Some (5);
    valor = None; // // Derrepente, el valor ha pasado a None. Pero mantiene su tipo, Entonces, en el match, no habrá errores cuando no exista ese dato

    match valor {
        Some(n) => println! ("Tenemos un valor: {}", n),
        None => println!("No hay ningún valor"),
    };

    // En vez de colocar "-1" para indicar que el valor no existe, simplemente no se declara. 

    // El operador match....

}
