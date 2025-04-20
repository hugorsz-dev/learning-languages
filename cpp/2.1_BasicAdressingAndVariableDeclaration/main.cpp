/*
 * Los datos se almacenan en forma de bytes (010101) en la memoria RAM, la interpretación de estas ristras de números
 * es gracias a los "data types", gracias a estos, el compilador sabe que esa localización en la memoria pertenece a un
 * número entero, a un string, a un carácter...
 */

int main ()
{
    // Declaración de variables (booleano, char, int, float, double)
    bool bValue; 
    char chValue; 
    int nValue; 
    float fValue; 
    double dValue; 

    // Asignación posterior de una variable
    nValue = 0;

    // Declaración y asignación de una variable al mismo tiempo. 
    int nValue2 = 0; 

    // Asignación explícita (mismo resultado que antes)
    int nValue3(0);

    // Asignación de valores en la misma línea
    int nValue4(0), nValue5=0, nvalue6=0;

    /*
     * Es bueno considerar que las variables no deben declararse al principio del main, sino declararse a medida que el
     * programa avanza, para que el uso de estas sea contextual y comprensible. 
     */
    /* 
     * Convenciones sobre el uso de variables en C++: 
     * > Los identificadores de una sola palabra, se escribirán en minúscula (value)
     * > Si conrtiene más de dos palabras, la primera en minúscula, la segunda en mayúscula (myValue)
     * > También es válido (aunque más confuso) usar guiones bajos (my_value)
     * > La notación húngara especifia el tipo de la variable en su nombre [nvalue (int), bValue (bool), chValue (char),
     * dValue (double), fValue (float)
     */


}

