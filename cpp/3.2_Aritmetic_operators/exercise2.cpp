#include <iostream> 

bool isEven (int nNumber)
{
    return {nNumber%2==0};
}

int main ()
{
    using namespace std; 

    int nNumber=0;
    
    cout << "Introduzca un número: "; 
    cin >> nNumber;
    
    if (isEven (nNumber)) 
        cout << "El número introducido es PAR.";
    else 
        cout << "El número introducido es IMPAR.";
}
