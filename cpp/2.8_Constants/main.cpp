#include <iostream> 

int main ()
{
    /* 
     * C++ tiene varios tipos de constantes. La constante literal es aquel tipo de valor que nunca cambiará. 
     * Los números son constantes literales: "5" siempre será "5" indistintamente de cuando se use
     */

    int nValue = 5; 

    // Algunos pueden contener el sufijo "u" (unsigned) o "L" (long), pero estos típicamente son insertados por el
    // compilador. 
    
    int nValue2 = 5u;
    long lValue = 5L; 

    // Por esta razón, por ejemplo, sabemos que hay que introducir el sufijo "f" para los float, porque por defecto los
    // números como "5.2" se asignan a los double.
    
    float fValue = 5.2f; // Debemos especificar el float "f"
    double dValue = 5.2; // El compilador especifica el double "d"

    // Las constantes también pueden introducirse a nivel de preprocesador, pero son directivas globales y pueden ser de
    // difícil lectura. Para hacer constantes, podemos utiliza el modificador const 

    const int nValue3 = 45; 
    nValue3 = 12; // Error de compilador, dado que es una constante (inmodificable)
}
