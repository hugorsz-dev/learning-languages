#include <iostream> 

int ReadNumber ()
{
    using namespace std; 
    
    int output = -1;
    cout << "Introduzca un valor: ";
    cin >> output; 
    
    return output; 
}

void WriteAnswer (int answer)
{
    using namespace std;
    cout <<  "El resultado de la suma de ambos números es: " << answer << endl;
}
