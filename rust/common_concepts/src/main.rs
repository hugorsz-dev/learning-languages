fn main() {

    println! ("------------ VARIABLES Y MUTABILIDAD ---------");

    // Variable inmutable - no puede cambiar
    let x = 5;

    println! ("Variable inmutable {x}");

    // ... y sin embargo, se puede "ensombrecer". Porque con el let se crea OTRA variable llamada x que la sustituye. 
    let x: i32 = x + 1;

    println! ("Variable ensomrbecida {x}");

    /* 
      Es una estrategia sombrear cuando hacemos conversiones de tipos, por ejemplo:
      let spaces = "    "; 
      let spaces = spaces.len()
    */

    // Variable mutable - puede cambiar
    let mut y = 5;

    y = y +1;

    println! ("Variable mutable {y}");

    // Constante - común a todos los ficheros. 
    const PI: f32 = 3.141592;

    println! ("Constante {PI}");

    println! ("------------ TIPOS DE DATOS ---------");
    
    /* 
    NÚMEROS ENTEROS
      Length	Signed	Unsigned
      8-bit	  i8	    u8
      16-bit	i16	    u16
      32-bit	i32	    u32
      64-bit	i64	    u64
      arch	  isize	  usize
    -----------------------------
    LITERALES
      Number literals	Example
      Decimal	        98_222
      Hex	            0xff
      Octal	          0o77
      Binary	        0b1111_0000
      Byte (u8 only)	b'A'
    */

    let int_number: u16 = 1_000; // Pueden escribirse números decimales con "_" (1000), para mejorar legibilidad

    println! ("Número entero expresado con '_' {int_number}");

    // Tipos de punto flotante

    let r = 2.0;
    let p: f32 = 1.2;

    println! ("Tipos de punto flotante f64 {r} y f32 {p}");

    // Operaciones

    let suma = 5+10; 
    let resta = 23-43; 
    let multiplicacion = 7*5;
    let division = 24/7; 
    let resto = 24%7;

    println! ("Operaciones aritméticas:");
    println! ("\tSuma: {suma}");
    println! ("\tResta: {resta}");
    println! ("\tMultiplicación: {multiplicacion}");
    println! ("\tDivisión: {division}");
    println! ("\tResto: {resto}");

    // Tipo booleano

    let t = true;
    let f = false; 

    println! ("Booleano verdadero {t}");
    println! ("Booleano falso {f}");
    // Tipo caracter
    
    let c = 'a';  // Deben usarse las comillas simples ' ' 
    let b: char = '$';

    println! ("Caracteres {c} y {b}");

    let j = "Hola, me llamo Hugo"; // De lo contrario, será un string

    println! ("String que contiene: {j}");

    // DATOS COMPUESTOS

    // Tupla 
    // Sirve para almacenar valores de variables distintaas

    let tupla: (i32, f64, u8) = (500, 6.4, 1); // Debe declararse indicando todos sus tipos, o bien...
    let tupla2 = ("Mi gozo en un pozo", 21, 'a');

    let (x, y, z) = tupla; // Pueden asignarse variables a una tupla

    println!("Los valores 'x', 'y' y 'z' asignados en la tupla son: {x} {y} {z}"); // Y llamarlas, en este caso, siendo 6.4
    println! ("El primer valor de la tupla: {} ", tupla2.0); // O llamarlas indicando su índice

    // Array
    // El array es un grupo de un solo tipo de datos (no es extensible, no puedre crecer ni decrecer)

    let meses = ["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio"];

    let segundo_mes = meses[1];

    println! ("En el array de meses, el segundo mes del año es {segundo_mes}");

    println! ("------------ FUNCIONES---------");

    // A Rust le da igual donde declares tus funciones, solo que estén escritas en "alguna parte". Fuera del main, o dentro del main

    fn say_hello () {
      println! ("Esto es una función que imprime texto: ¡Hola!")
    }

    say_hello();

    // Parámetros de función

    fn add_number (number1: i32, number2: i32) {
      println!("Función que suma números, y la suma es {}", number1+number2)
    }

    add_number(21, 22);

    // Los cuerpos de función contienen declaraciones y expresiones

    // Sentencias y expresiones

    /*
      En RUST, el 'return' sobra, porque es un lenguaje basado en expresiones y sentencias. 
        Si lleva ";" al final, es una sentencia. 
        Si no lleva ";" al final, es una expresión 
        - La expresión retorna  valores (como si tuvieran un return delante)
        - La sentencia, no. 
    */

    // POR EJEMPLO, ESTO: 

    let y = {
      let x = 3;
      x + 1 // es como si fuera "return x+1"
    };

    println! ("Esta variable está cargada con una expresión, y su valor es {y}");

    // Funciones con retorno. 

    fn four () -> i32 {
      4
    }

    println! ("¿Variables con retorno? Así es, fíjate, porque retornará cuatro: {}", four());

    println! ("------------ FLUJO DE CONTROL ---------");

    // Ifs

    let number = 6; 

    if number < 5 {
      println! ("El número es inferior a 5 (condición falsa)");
    } 
    else {
      println! ("El número es superior a 5 (condición verdadera)")
    }

    if number != 0 {
      println! ("El número NO es cero...");
    }
    else if number ==0 {
      println! ("El número es cero.");
    }

    let condition = true; 

    if condition {
      println! ("La condición {condition} es verdadera ")
    }

    // Declaraciones let con ifs

    let number = if condition  {5} else {1};
    // let number = if condition  {5} else {"five"}; esto no funcionará porque los dos tipos a valorar serán distintos
    println! ("Puesto que la condición es verdadera, el número es {number}");
    
    // Bucles
    // En la lección "guessing name" se mostró un uso del bucle loop. 
    let mut counter =0;
    loop {
      println! ("Se repite!");

      // Esto no es nada conveniente, porque se supone que el loop está hecho para ser infinito. Pero lo pongo para limitarlo. 
      counter += 1;
      if counter >= 3 {break;};

    }

    counter =0;

    // Pero loop () también puede servir para calcular operaciones hasta cierto hilo ha terminado p.ej
    // "break" tiene la capacidad de retornar numeros.
    // El siguiente bucle no se detendrá hasta llegar a diez. Y cuando llega, devuelve el valor del contador multiplicado por dos, veinte. 
    let result = loop {
      counter +=1; 

      if counter == 10 {
        break counter*2;
      }
    };

    println! ("El resultado de la variable dentro del bucle es: {result}");

    // Utilizando continues y etiquetas en los loop
    counter =0;

    'counting_up: loop {
      println! ("count = {counter}");
      let mut remaining = 10; 
      
      loop {
        println! ("Remaining = {remaining}"); 

        if remaining == 9 {
          break; // cancelación de este bucle
        }

        if counter ==2 { 
          break 'counting_up; // cancelación del bucle indicado (fin de todo el bucle)
        }

        remaining -= 1;
      }

      counter +=1; 
    }

    // WHILE 

    counter =0; 

    while counter !=5 {
      counter+=1;
    }

    println! ("El valor de counter, después del bucle while, es {counter}");

    // FOR EACH

    let a = [10,20,30,40,50,60];

    for element in a {  
      println! ("El valor del array es {element}");
    }

    // Rango determinado

    for number in 1..4 {
      println! ("El valor del rango es {number}")
    }

    // ... e invertido

    for number in (1..4).rev() {
      println! ("El valor del rango invertido es {number}")
    }


}


