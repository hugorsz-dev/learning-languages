#include <iostream> 

int main () {
    
    using namespace std; 

    // El operador comma (",") permie realizar operaciones en una misma línea, durante la asignación de variables

    int x = 0;
    int y = 2;
    int z = (++x, ++y); // Sube de valor a `x` (+1). Sube el valor a `y` (+1). Carga en `z` el resultado de  `y`

    cout << "x: " << x << endl; 
    cout << "y: " << y << endl; 
    cout << "z: " << z << endl; 

    // Operador ternario (o `if aritmético): similar a un if, en una sola linea, aplicable a valores numéricos.
        //Cond.//Si//No
    z = (x>y) ? 12 : 24; 

    cout << "z: " << z << endl; 
   
    // NOTA: No usar el operador ternario en estre estamentos como "<<", p.ej un "cout", o valorará incorrectamente la
    // expresión

    return 0;

}

