#include <iostream> 

// Para realizar funciones recursivas, o llamar a funciones antes de que sean declaradas, es necesario indicar
// al compilador que estas existen, para lo cual existen los "prototipos". 

// Prototipo de "add ()" declarado antes de existir.
// Es necesario saber que los parámetros del prototipo deben coincidir exactamente con los de la función posterior. 
int add (int x, int y);

// Método principal. 
int main ()
{
    using namespace std; 
    cout << "La suma de 3 y 4 es " << add (3,4) << endl;
    return 0; 
}

// Sumar dos números.
int add (int x, int y)
{
    return x+y;
}
