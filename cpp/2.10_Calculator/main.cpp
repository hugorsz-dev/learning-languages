#include <iostream>

double dCalculateBySymbol (double dValue1, double dValue2, char chOperation)
{
    
    if (chOperation == '+')
        return dValue1+dValue2; 
    else if (chOperation == '-')
        return dValue1-dValue2; 
    else if (chOperation == '*')
        return dValue1*dValue2;
    else if (chOperation == '/')
        return dValue1/dValue2; 
    else 
        return -1; 
}


int main () 
{
   using namespace std;  
   
   double dValue1(0.0);
   double dValue2(0.0);
   char chOperation (0);

   cout << "Introduzca el 'dValue1    '\t   : ";
   cin >> dValue1; 
   
   cout << "Introduzca el 'dValue2    '\t   : ";
   cin >> dValue2; 

   cout << "Introduzca el 'chOperation'\t   : "; 
   cin >> chOperation;

   if (dCalculateBySymbol (dValue1, dValue2, chOperation)!=-1)
       cout << "El resultado de su operación es: " << dCalculateBySymbol (dValue1, dValue2, chOperation) << endl; 
   else
       cout << "Ha introducido un símbolo incorrecto o no reconocido por la herramienta"; 

}
