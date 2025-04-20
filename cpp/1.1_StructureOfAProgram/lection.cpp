// Primer programa en C++ 
//
//
// Importación de librerías
#include <iostream> // El "standard" (std) contiene los métodos coyt y endl, para imprimir texto en pantalla.
#include <cstdio>

int main() {

    /*
     * PRIMERA LECCIÓN: IMPRIMIR EN PANTALLA
     *
     */


    // Si el compilador no encuentra la instrucción, llamará a "std". 
    using namespace std;

    // Imprimir texto en pantalla usando la librería iostream  
    // el símbolo "<<" es un operador, que representa que se le envía un String "" a la consola, representada en el
    // objeto cout. 
    std::cout << "Hola, mundo..." << std::endl;
    std::cout << "... todo en C++ funciona con bibliotecas, iostream es la que estoy usando." << std::endl; 
    std::cout << "Compilo desde GNU C++ Compiler, G++, ¿no es divertido?" << std::endl ;
     
    cout << "Esto ya está usando un namespace, lo que quiere decir que si el compilador NO encuentra la instrucción 'cout', la buscará en std" << endl;

   // Imprimir texto en pantalla usando la librería cstdio
   printf("La librería 'cstdio' te permite imprimir de manera diferente, con printf()\n");
   
   return 0; // La salida "0" normalmente implica que no ha habido errores de ejecución, 
}


