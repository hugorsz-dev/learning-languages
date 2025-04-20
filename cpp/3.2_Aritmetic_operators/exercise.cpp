#include <iostream> 

int main ()
{
    using namespace std; 

    // Un programa que imprima cada numero de 1 al 100 con 20 números en cada línea.
    
    int nCount =1; 

    while (nCount<=100) 
    {   
        cout << nCount << " | ";

        if (nCount%20==0)
            cout << endl;
               
        nCount = nCount + 1; 
    }
}
