#include <iostream>
#include "io.h"

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
