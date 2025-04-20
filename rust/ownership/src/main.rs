fn main() {

    // HEAP Y STACK

    // Primer caso

    let mut s = String::from("hola"); // Un string es una pieza de texto indeterminada, por lo que se localiza en el "heap" usando un puntero. 
    
    // Segundo caso

    s.push_str("another thing");


    /* Diferencias entre String::from("hola") y ="hola"
        En el primer caso, se está depositando la memoria en el "heap", donde se guarda la memoria indeterminada
        En el segundo caso, se está depositando en la memoria determinada, puesto que el valor el string será conocido por el compilador
            -> ¡Por eso cuando pedimos un input, es necesario usar un objeto string, en vez de un literal!
    */

    // DESPLAZAMIENTO DE LA MEMORIA o "trait Copy"

    let x: i8 = 5; // Puesto que 'n' es de un tipo determinado, la cantidad de memoria reservada es exacta,
    let y = x; // Al equivaler "x" a "y", se copia su valor. Y dado que es un tipo de dato específico, el proceso es extremadamente rápido

    // En Rust, las equivalencias de variables con un tamaño fijo son sencillas. Los problemas suceden con los tipos de tamaño no determinado. 
    
    // OTRO CASO 

    let mut s1 = String::from("hola"); // Ahora, "hola" es un conjunto de datos no determinado...
    let s2 = s1; // ... por lo que esta equivalencia necesita usar entre medias un puntero, que es una estimación de la memoria que ocupará el string. Este proceso consume más memoria, y resulta menos óptimo
    
    // Para paliar la lentitud de la equivalencia, al equivalerse los datos (s1 a s2), se CEDE el puntero ya calculado. 
    //Es decir no se está haciendo un copiado literal de los datos, sino redirigiendo la variable al valor del puntero.
    // Como no se necesita calcular nuevamente el puntero, trae consigo un gran rendimiento.
    
    println! ("{}",s2); // El valor que devolverá será "hola", el puntero que se mantiene es el anterior
    
    // println! ("{}", s1);// El puntero de s1 se ha cedido a s2, por lo que esta operación no funcionará, s1 ha dejado de funcionar
    
    // CON FUNCIONES


    fn tomar_ownership (un_string: String) {
        println!("ownership tomado de {un_string}");
    }

    tomar_ownership(s2);  // Entregamos el puntero a la función
    // println! ("{}",s2);// Ahora este código dejará de funcionar, porque "s2" se ha cedido a la anterior función

    // Puede subsanarse este comportamiento devolviendo el string

    fn toma_y_devuelve (un_string:String) -> String {
        println! ("copiado {un_string}");
        un_string
    }
    
    let mut s3 = String::from("cosa");
    s3 = toma_y_devuelve(s3);
    println! ("{}",s3);

    // O MEJOR: Empleando REFERENCIAS, evitaremos todos estos problemas de la pérdida de ownership

    fn tomar_ownership_ref (un_string: &String) {
        println!("ownership referenciado de {un_string}");
    }

    let mut s4 = String::from("flipante");
    tomar_ownership_ref(&s4); // La "referencia" (&) te permite tomar un valor sin apropiarte de él, ==SOLO LECTURA==
    println!("{}",s4);

    // REFERENCIAS MUTABLES
    // Acabamos de decir que las referencias son de solo lectura.
    // ... salvo que sean referencias mutables!

    fn toma_y_modifica_ref (un_string: &mut String) {
        un_string.push_str(" modificado");
    }
    
    toma_y_modifica_ref(&mut s4);
    println! ("{}",&mut s4);

    // La gran restricción de las variables mutables es que no se pueden asignar varias veces
    let e1 = &mut s4;
    
    //let e2 = &mut s4; // ERROR
    //println! ("{}, {}", e1,e2); // ERROR

    // Tampoco se pueden combinar referencias mutables con inmutables. Esto es porque rust aspira a ejecutar los procesos directamente
    // Variables inmutables de por medio implican conversiones entre medias del código, que generan ralentizaciones
    
    // LIMPIADO DE LA MEMORIA

    // En cuanto una variable sale de su contexto de ejecución - en este caso, main - se destruye, liberando la memoria (se ejececuta una función no explícita, llamada "drop")

    // EL TIPO SLICE
    let mut x5: String = String::from("Hola, soy hugo");
    let word = first_word(&x5);
    x5.clear();

    // Su supone que esta es una mala praxis, porque la variable word no está alineada con "x5". Limpiar "x5" no limpiará word. 
    // Esto se soluciona con los... ¿slices? A ver.

    let mut x6 = String::from("Holá mundo"); // Si utilizamos á en vez de a, lo considerara un caracter multibyte no compatible con UTF8, por lo que se saltará el slice. 
    let hola = &x6[0..5];
    let mundo = &x6[5..10];
    println! ("{hola} - - - {mundo}");

    /*
        En definitiva, los slices son una forma de referir rangos en los arrays de rust, similares a los substrings de java o los rangos de python
    */
    let x12 = &x6[..5]; // desde el inicio de la cadena hasta el quinto caracter
    let x13 = &x6 [3..]; // desde el tercer caracter hasta el fin (len) de la cadena
    // Aquí, "x13" y "x12" son referencias de "x6", por lo que al limpiar "x6" obtendremos un error


    // OTROS SLICES
    // Como puede verse, siempre se realizan con referencias &
    let b1 = [1,2,3,4,5,6];
    let b2 = &b1[2..5];







}









// Los slices son de tipo referencia &str, por lo que se deben de devolver también como &str
// En conclusión, para hacer string slices siempre es conveniente tener el tipo &str, en nuestras funciones
// ya que es un tipo compatible tanto con literales "" como indefinidos objetos String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Conversión del string a un array de bytes

    for (i, &item) in bytes.iter().enumerate() { // Mediante un .iter().enumerate() podremos iterar el conjunto con una tupla de iteracion
        if item == b' ' {
            return &s[..i]; // Retornará la posición del primer espacio
        }
    }

    &s[..] // ... y si no lo encuentra, el numero de caracteres del string. 

}

/* 
fn first_word (s: &String) -> usize {
    let mut counter: usize =0;
    for character in s.chars() {
        if character == ' ' {
            break
        }
        counter += 1;
    }
    counter
}
*/
/* 
fn first_word(s: &String) -> String {
    match s.split(" ").next() {
        Some(part) => part.to_string(), 
        None => String::new()
    }
}
*/