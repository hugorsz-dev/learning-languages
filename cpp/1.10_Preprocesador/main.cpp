/*
 * El preprocesador es una funcionalidad integrada antes de la fase de compilación (o antes de la inicialización de la
 * misma, como parte de ella), haciendo operaciones sobre el fichero de texto, basadas en la información facilitada en
 * las directivas. 
 *
 * Las directivas siempre empiezan por "#", y terminan con el carácter de salto de línea (no ";"). Estas, como en la vez
 * anterior, sustituyen su declaración por prototipos de funciones, pero también tienen otros tipos con diferentes usos
 */

#include <iostream> 
#define MY_NAME "Hugo" // Esto modificará todas las ocurrencias de "MY_NAME" por "Hugo". No es un espacio en memoria que se pueda modificar, sino una orden previa a la compilación; puede considerarse como una suerte de "constante"; el empleo de este tipo de directiva clarifica el código, puesto que el programador ajeno comprenderá que MY_NAME responde a "Hugo", en lugar de que si hubiera puesto "Hugo" simplemente. 

int main () 
{
    using namespace std; 
    cout << "Hola " << MY_NAME << endl;

    #undef MY_NAME // Indica al preprocesador que destruya esa indicación
    //cout << "Hola " << MY_NAME << endl; (error)
    
    #define PRINT_HELLO

    #ifdef PRINT_HELLO
    cout << "Hola, el define PRINT_HELLO existe" << endl; 
    #endif

    #ifndef MY_NAME
    cout << "El define MY_NAME no existe" << endl; 
    #endif
}
