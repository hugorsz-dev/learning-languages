#include <iostream>

bool IsEqual (int x, int y) 
{
  return {x==y};
}

int main ()
{
    
    using namespace std; 
    //VALORES BOOLEANOS --------------------
    
    bool bValue = false; 
    bool bValue1 = !false; // Será "true"
    bool bValue2(!true); // Será "false"

    cout << bValue << endl; 
    cout << bValue1 << endl; 
    cout << bValue2 << endl; 

    // Normalmente se indican como "is" para las variables, e "Is" para las funciones. 
    
    int iInput1 = 0; 
    int iInput2 = 0; 

    cout << "Introduzca el primer valor: ";
    cin >> iInput1;
    

    cout << "Introduzca el segundo valor: ";
    cin >> iInput2; 


    if (IsEqual(iInput1, iInput2))
        cout << "Los valores introducidos son iguales" << endl; 
    else
        cout << "Los valores introducidos no son iguales" << endl; 
}
