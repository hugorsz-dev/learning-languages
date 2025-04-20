#include <iostream>

int main(){
    // La variable "x" es un left-value, siempre estará a la izquierda, porque representa un espacio a rellenar en la
    // memoria. Mientras que el 5 representará un right-value, siendo que "5" siempre será "5" para el compilador,
    // mientras que de otro modo "x" será un valor que podrá modificarse.
    // 
    // (Los valores introducidos en memoria son volátiles, es decir, no se asignan permanentemente a ese trozo. Un
    // proceso puede asigar a la variable "x" el lugar "123" de la memoria, pero en la siguiente ejecución, este podrá
    // cambiar al valor "224", p.ej)
    
    int x;
    x=5; 
    x=2+5; 

    int y; 
    y=x; 
    y=y;
    x=x+1;

    int z;

    std::cout << "El valor de x es: " << x << std::endl;  
    std::cout << "El valor de y es: " << y << std::endl; 
    std::cout << "El valor de z es: " << z << std::endl;

    // Debido a que algunos compiladores ni siquiera arrojan errores de no-inicialización de variables, o debido a las
    // dificultades que trae consigo encontrar errores relacionados con la no-inicialización, será recomendable iniciar
    // todas las variables nada más declararlas, de este modo. 
    
    int g = 23; 

    std::cout << "El valor de g es: " << g << std::endl;

    // Pidámosle al usuario que nos indique dos números y se los sumaremos. 
    // Esto se logrará gracias a cin, otra utilidad de la biblioteca iostream. 
    
    int variable1=0;
    int variable2=0;

    std::cout << "Introduzca el primer número: "; 
    std::cin  >> variable1; 
    
    std::cout << "Introduzca el segundo  número: "; 
    std::cin  >> variable2;

    std::cout << std::endl << "El resultado de la suma es: " << variable1+variable2;

}
