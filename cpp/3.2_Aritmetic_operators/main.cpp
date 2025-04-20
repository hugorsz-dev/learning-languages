#include <iostream>


int main () {

    // Operadores aritméticos.
    int nAddition = 5+3; 
    int nSubstraction = 5-3; 
    int nMultiplication = 5*3; 
    
    // La división tiene dos modos: cuando divide dos integers o un integer con alguna operación de punto
    // flotante, trunca el resultado. Cuando ambas variables son de punto flotante, entonces resulta en 
    // un decimal
    // ADVERTENCIA: Cuando se dividen dos enteros, siendo uno de ellos negativo, es posible que dependiendo del
    // compilador esta operación sea truncada hacia arriba o hacia abajo (p.ej: -5/2 puede ser -3 o -2) 
    double dDivision = 5/3; 

    // Calcula el resto de una división, y su resultado siempre será un integer
    // ADVERTENCIA: Si uno de los operandos es negativo, el compilador puede hacer algo azaroso si hacer o no negativo
    // el resto.
    int dRemainder = 5%3; 

   // Operaciones rápidas (asignadores aritméticos)

   int x = 3, y=4;
    
   x += y; // x = x+y
   x -= y; // x = x-y
   x *= y; // x = x*y
   x /= y; // x = x/y
   x %= y; // x = x%y
}
