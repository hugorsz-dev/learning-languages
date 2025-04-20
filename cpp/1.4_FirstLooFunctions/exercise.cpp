
/*
Write a complete program that reads an integer from the user (using cin, discussed in section 1.3), doubles it using the doubleNumber() function you wrote for question 4, and then prints the doubled value out to the console.
*/

#include <iostream>

// Función que duplica el número introducido por parámetro. 
int doubleNumber (int number)
{
    return number*2; 
}

int main ()
{
    using namespace std;

    int userInput = 0;

    cout << "Escriba un número para duplicar: "; 
    cin >> userInput; 
    cout << "El número duplicado es: " << doubleNumber(userInput) << endl;

}
