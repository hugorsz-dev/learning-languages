
/* NOTA SOBRE LA COMPILACIÓN DE PROYECTOS CON VARIOS FICHEROS:
 * El compilador G++ necesita tener constancia de todos los ficheros a los que haga referencia el main, en este
 * caso, el comando necesario para compilar este proyecto será el siguiente: 
 * g++ main.cpp add.cpp -o load_cpp.out
 * 
 * ¿CÓMO SE COMPILA? FASES DE COMPILACIÓN:
 * 1º - G++ compila, es decir, pasa a código máquina todos y cada uno de los ficheros .cpp. Esta parte de la
 * construcción es solo "aparente", es decir, se comprueba que las funciones o sus prototipos estén presentes de
 * manera superficial, pero su funcionalidad se comprueba en la fase siguiente, el linking 
 * 2º - Este código máquina está separado en tres ficheros, pero estos necesitan ser ENLAZADOS (LINKING), para
 * constituir un único ejecutable. Aquí es cuando el verdadero contenido de las funciones que antes se
 * consideraban prototipos se introduce en el ejecutable
 
 RESUMEN

 main.cpp -----> COMPILACIÓN ----> main.o |
  add.cpp -----> COMPILACIÓN ---->  add.o |

  add.o (y el contenido de sus funciones) ----->   SE ENLAZA CON ---> main.o
  ==== load_main.out (ejecutable)

*/ 

#include <iostream> 

// Es necesario que el prototipo esté declarado para que la función importada de add.cpp funcione correctamente
int add (int x, int y);

int main ()
{
    using namespace std; 
    cout << "La suma de 3 y de 4 es: " << add(3,4) << endl; 
    return 0; 
}
