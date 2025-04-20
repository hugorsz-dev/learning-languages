mod utils;
fn main() {
    // Solicitar un texto 
    println!("Text cutter! Enter your text:\n||||||||\nvvvvvvvv");
    
    let mut input = utils::Cin.string();
    
    // Asegurarse de que no está vacío
    loop {
        if input.trim()=="" {
            println!("Please! Your previous text was empty:\n||||||||\nvvvvvvvv");
            input = utils::Cin.string();
        }
        else {
            break;
        }
    }
    
    // Solicitar intervalo de caracteres
    println!("Great! Now, enter the character range:\n||||||||\nvvvvvvvv");

    let mut range = utils::Cin.usize();

    // Asegurarse de que no es superior al input
    loop {
        if range>(input.len()-1) {
            println!("Please! The entered interval is bigger than the lenght of your text:\n||||||||\nvvvvvvvv");
            range = utils::Cin.usize();
        }
        else {
            break;
        }
    }

    // Calcular el número de iteraciones

    let iterations = input.len()/range;

    // Mostrar el texto iterado

    let mut cursor: usize = 0; // Cursor que guardará el progreso del texto
    let auxrange = range; // Retención de la variable de rango. 
    
    for i in 0..iterations  {
        println! ("({i}/{iterations}) _______________________");
        println!("{}",utils::StringUtils.utf8_slice(&input, cursor, range));
        cursor = cursor + auxrange;
        range = range + auxrange;

        utils::Cin.string();
    }

    println! ("(REMAINDER) _______________________");

    if input.len()%range !=0 {
        println!("{}",input[cursor..].to_string());
    }

}
