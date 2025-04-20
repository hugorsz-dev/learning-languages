#include <iomanip>
#include <iostream> 

int main ()
{
    using namespace std; 

    // INTEGERS --------------------------------

    int nInt; // (4 bytes) -2,147,483,648 to 2,147,483,647
    
    int short nShort; // (2 bytes) -32,768 to 32,767
    // Recomendable usar "short" en vez de "int short"
    short nShort2; // (2 bytes) -32,768 to 32,767
    
    int long nLong; // (8 bytes) -9,223,372,036,854,775,807 to 9,223,372,036,854,775,807
    // Recomendable usar "long" en vez de "int long"
    long nLong2; // (8 bytes) -9,223,372,036,854,775,807 to 9,223,372,036,854,775,807

    /* Cuando introducimos el modificador "unsigned", obtenemos que los números negativos desaparecen, dduplicándose así
     * la cantidad de memoria disponible para los números positivos
     */

    unsigned int nIntUnsigned; //  (4 bytes) 0 to 4,294,967,296
    unsigned long nLongUnsigned; // (8 bytes) 0 to 18,446,744,073,709,551,615
    unsigned short nShortUnsigned; // (2 bytes) 0 to 65,535

    // Ahora vamos a hacer overflow (sobrepasar el límite de una variable).
    
    nShortUnsigned = 65535; // La variable está en su límite
    cout << "El valor de la variable short unsigned es " << nShortUnsigned << endl;
    nShortUnsigned = nShortUnsigned +1; 
    cout << "El valor de la variable short unsigned es " << nShortUnsigned << endl;
    // El valor que arroja una vez de sobrepasar el límite será 0.

    nShortUnsigned = nShortUnsigned -1;    
    cout << "El valor de la variable short unsigned es " << nShortUnsigned << endl;
    // El valor que arroja al sobrepasar por abajo el límite será 65534, dando la vuelta

    // Algunas operaciones
    cout << "Algunas operaciones: " << endl; 
    cout << "Suma\t" << 5+6 << endl; 
    cout << "Resta\t" << 5-6 << endl; 
    cout << "División\t" << 4/2 << endl; 
    cout << "Multiplicación\t" << 5*6 << endl; 
    cout << "Resto\t" << 5%6 << endl; 

    // FLOATING POINT NUMBERS --------------------
   
    /*
     * Internamente, C++ usa para cada variable flotante una forma diferente de escribirse:
     * "3.0" para double
     * "3.0f" para float
     * "3.0L" para long double
     *
     * También pueden ser usados con notación científica los doubles, es decir, "5e-2" o "5e2" (p.ej)
     */

    float fValue = 0.5; 
    double dValue = 5e-2; // También compatible con notación científica (negativa)
    long double dValue2 =  5e2; // ''                                      (positiva)

    /* Ahora bien, los niveles de precisión de float no son los mismos que double o long double, lo que significa que
    * estos segundos serán capaces de tener más decimales
    */

    cout << setprecision(16); // Ordenamos a la consola que tenga una precisión de 16 digitos decimales (de lo contrario, cortara los decimales)

    fValue = 3.333333333333333333333333333333333333f; 
    dValue2 = 3.33333333333333333333333333333333333; 

    cout << "El valor de float es\t" << fValue << endl; // La precisión en este caso es menor (hasta 7 decimales)
    cout << "El valor de double es\t" << dValue2 << endl; 

    // Para hacer una comparativa más exacta de la precisión, podríamos compararlo con un número infinito
    fValue = 1.0L/3.0L; 
    dValue = 1.0L/3.0L; 
    dValue2 = 1.0L/3.0L;

    cout << "El valor de float es\t" << fValue << endl;
    cout << "El valor de double es\t" << dValue << endl; 
    cout << "El valor de double2 es\t" << dValue2 << endl;

    /*
     * Internamente, la conversión de la base binaria a decimal flotante es complicada, y crea dilemas de conversión que
     * los programas que necesiten de una matemática estricta generarán fallos, como el siguiente 
     */

    cout << setprecision(17);
    dValue = 0.1; 
    cout << dValue << endl; // 0.10000000000000001
    dValue = 0.1+0.1+0.1+0.1+0.1+0.1+0.1+0.1+0.1+0.1;
    cout << dValue << endl; // 0.99999999999999989

    // Comparativa entre floats
    float fValue2 = 1.345f; 
    fValue = 1.123f;
    float fTotal = fValue+fValue2;

    if (fTotal == 2.468f) // Será necesario especificar que la comparativa es un "f".
        cout << "float es 2.468" << endl; 
    else
        cout << "float NO es 2.468" << endl;


}
