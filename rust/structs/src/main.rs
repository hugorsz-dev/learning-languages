fn main() {
    // Los "structs" son los objetos de rust. El siguiente es un struct para un usuario

    // Creación de objeto
    struct Usuario {
        activo: bool, 
        apodo: String, 
        email: String, 
    }

    // Crear una variable con la forma del objeto 
    let mut usuario1 = Usuario {
        activo: true,
        apodo: String::from("Hugo Ruiz"), 
        email: String::from("someone@example.com"),
    };

    // Mostrando y modificando objeto
    println! ("{}", usuario1.apodo);
    usuario1.email = String::from("hugoruizschz@gmail.com");

    // Creando un constructor
    // -> nótese que en vez de escribir: email: email, escribimos "email" sin más, es una forma de abreviatura
    fn build_user (email: String, apodo: String) -> Usuario {
        Usuario {
            activo: true, 
            apodo, 
            email
        }
    };

    let usuario2 = build_user(String::from("someone@example.com"), String::from("manuel.fernandez"));
    
    // Actualización de objetos
    // Todos será igual excepto el email, que hemos modificado. 
    let usuario2 = Usuario {
        email: String::from("manuelfernandez@gmail.com"),
        ..usuario2
    }; 

    // Structs de tuplas
    // -> Como los structs, pero sin nombres en las variables

    struct Rgb(i32, i32, i32); 
    let negro = Rgb (0,0,0);

    // Structs de unidad 
    struct undefined;
    let cosa = undefined;

    // Programa de ejemplo: calcular el área de un rectángulo con structs. 
    #[derive(Debug)]
    struct Rectangulo {
        ancho: u32, 
        alto: u32
    }

    fn area (rectangulo:&Rectangulo) -> u32 {
        rectangulo.alto*rectangulo.ancho
    }

    let rectangulo1 = Rectangulo {
        ancho: 32, 
        alto: 56
    };

    println! ("{}", area(&rectangulo1));

    println!("rectangulo1 is {rectangulo1:?}"); // Para mostrar objetos en el print, sería necesario activar el debug en el objeto y añador ":?"

    dbg! (&rectangulo1); // Salida de la consola, paralela al println,!; que ofrece información más detallada del objeto

    // MÉTODOS EN LOS STRUCTS

    // Pueden incrustarse métodos en los structs. Podríamos hacer un struct triángulo que lo incorporase

    struct Triangulo {
        base: u32, 
        altura: u32,
    }

    impl Triangulo {

        // Método para calcular el área 
        fn area (&self) -> u32 {
            (self.base*self.altura)/2
        }

        // Método para comprobar el area de otro
        fn puede_albergar (&self, triangulo: &Triangulo) -> bool {
            if triangulo.area()<self.area() {
                return true; 
            }
            false
        }

        // Constructor para crear un triángulo equilátero
        fn equilatero (size: u32) -> Self {
            Self {
                base: size,
                altura: size,
            }
        }

    }

    // Llamando al constructor

    let triangulo0 = Triangulo::equilatero(23);

    let triangulo1 = Triangulo {
        base: 8,
        altura: 23,
    };

    let triangulo2 = Triangulo {
        base: 23,
        altura: 14,
    };

    println! ("{}", triangulo1.area());
    println! ("{}", triangulo2.puede_albergar(&triangulo1));


    



    



}
