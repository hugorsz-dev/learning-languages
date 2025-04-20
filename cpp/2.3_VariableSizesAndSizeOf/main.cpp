#include <iostream> 

int main () {

    /* Las variables ocupan un espacio en memoria que debe ser cuidadosamente medido en el caso de los programas muy
     * largos, pues es limitada. Cada datatype alberga siempre un rango predeterminado de valores que es conveniente
     * acotar siempre que sea posible, para no ocupar más memoria de la necesaria
     */

    using namespace std; 
    
    cout << "bool:\t\t" << sizeof(bool) << " bytes" << endl; 
    cout << "char:\t\t" << sizeof(char) << " bytes" << endl;
    cout << "wchar_t:\t" << sizeof(wchar_t) << " bytes" << endl;
    cout << "short:\t\t" << sizeof(short) << " bytes" << endl;
    cout << "int:\t\t" << sizeof(int) << " bytes" << endl;
    cout << "long:\t\t" << sizeof(long) << " bytes" << endl;
    cout << "float:\t\t" << sizeof(float) << " bytes" << endl;
    cout << "double:\t\t" << sizeof(double) << " bytes" << endl;
    cout << "long double:\t" << sizeof(long double) << " bytes" << endl;

    // Se está usando el caracter \t que va guardando en celdas las palabras, (dos celdas, p.ej, para double, y una sola
    // para long double)
}

