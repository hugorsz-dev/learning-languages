#include <iostream> 

int main (){
    
    using namespace std;

    // El sumatorio de 1 es tan común que tiene abreviaciones especiales. 

    int x = 5; 
    int y = ++x; // y = 6; x = 6

    int a = 5;
    int b = x++; // b = 5; a = 6

    // La evaluación de la suma será ++x (antes) o x++ (después) 
    // Una demostración en cout es más ilustrativa: 
    
    int c = 0, d = 0; 
    cout << "'c' sin nada: " << c << endl;  
    cout << "'c++': " << ++c << endl; 
    cout << "'++c': " << c++ << endl; 
    cout << "'c' sin nada: " << c << endl;  

    // Todas estas reglas aplican también para la resta (x--, --x). 

    // Es necesario insistir en que todas estas funciones tienen efectos secudarios. Tienen un impacto sobre las
    // variables, por lo que no es bueno aplicarlas en funciones, como por ejemplo: int nValuie = Add (x, ++x); No solo
    // sumará el valor de "x", lo modificará. 
    


    return 0;
}
