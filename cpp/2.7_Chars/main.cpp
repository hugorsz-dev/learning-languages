#include <iostream>

int main ()
{

    using namespace std; 

    /*
     * Los valores de tipo 'char' son numéricos, pero las especificaciones de su tipo de dato indican que su salida no
     * debe ser numérica, sino que debe basarse en los códigos ASCII (American Standard Code for Information
     * Interchange), lo que quiere decir que estos códigos se traducirán a caracteres; el código de 'a' es '97'
     */

    char chValue = 'a'; 
    char chValue2 = 97;
    
    cout << chValue << endl; // Salida : a
    cout << chValue2 << endl; // Salida : a
    
    // Hagamos un conversor de ASCII a letra

    int iAsciiCode(-1);

    cout << "Introduzca un código ASCII: "; 
    cin >> iAsciiCode;
    
    cout << "El carácter ASCII resultante de " << iAsciiCode << " es " << char(iAsciiCode) << endl; ;
    
    // Caracteres de escape
    cout << "Este carácter\nEs para hacer un salto de línea" << endl; 
    cout << "Este carácter\tEs para hacer una tabulación" << endl; 

}
