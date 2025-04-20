
/*
 * HEADERS: 
 * Los headers son avisos para la fase de compilación; utilizamos #incluide <iostream> para que durante este proceso 
 * estén indicados los prototipos, sin la necesidad de declararlos en el main (como se hizo en el ejercicio anterior)
 * 
 * A nivel de directiva ("#"), lo que se está haciendo es llamar al PREPROCESADOR  para incluir estas declaraciones en
 * el documento. En lugar de #include <iostream> tenemos los prototipos que lo conforman.
 *
 * ESTO ES ¿declaramos el prototipo de cout? ¿o el de cin? NO. No lo hacemos porque estos prototipos ya están
 * registrados previamente en un HEADER. Y DEBEN estarlo, porque de lo contrario el compilador - que solo estudia las
 * referencias para hacer un codigo máquina superficialmente correcto - NO se confunda. 
 *
 * El contenido, es decir, el código efectivo que no constituye un simple identificador, se introduce en la fase LINKED. 
 * 
 * Ahora trataremos de hacer nuestros propios headers, indicados con nombre.h
*/ 

#include <iostream> // Cuando usamos <>, estamos empleando headers incrustados por defecto en el compilador. 
#include "add.h" // Cuando usamos "", estamos llamando a archivos que nosotros incrustamos. 


// Es necesario que el prototipo esté declarado para que la función importada de add.cpp funcione correctamente
int add (int x, int y);

int main ()
{
    using namespace std; 
    cout << "La suma de 3 y de 4 es: " << add(3,4) << endl; 
    return 0; 
}

// Información sobre include <iostream>
/*
 * Si include <iostream> no tiene una extensión ".h" es porque se decidió, hace años en la historia de C++, introducir
 * todas estas funciones en el namespace std. Para llamar a estas funciones, ya no tenemos que introducir solamente su
 * nombre, también el prefijo de su namespace (std::cout) o (using namespace std).
 *
 * ADVERTENCIA IMPORTANTE:
 * Debido a estos cambios en el lenguaje, es recomendable que para los headers incrustados en el compilador (runtime),
 * busquemos idealmente aquellos que NO tienen una extensión ".h", pues están deprecados. 
 * Debido a que algunas librerías provienen de C, que no usa namespaces, forzosamente algunos headers incrustados en el
 * compilador tendrán, sin alternativa, extensiones ".h"
*/

