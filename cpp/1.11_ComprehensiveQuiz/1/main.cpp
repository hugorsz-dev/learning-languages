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

int main ()
{
   using namespace std;

   int userInput1 =-1;
   int userInput2 =-1; 
   
   cout << endl; 
   userInput1 = ReadNumber ();
   userInput2 = ReadNumber (); 
   cout << endl; 
   WriteAnswer (userInput1+userInput2);
}
