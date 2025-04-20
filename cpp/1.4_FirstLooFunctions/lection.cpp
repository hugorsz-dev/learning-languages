#include <iostream> 

void sayHello () {
    using namespace std; // Esta advertencia al compilador debe llamarse en cada función donde se quiera utilizar. 
    cout << "¡Hola! He sido llamado desde la función sayHello()" << endl; 
}

int add (int number, int sum) {
    return number+sum; 
}


int main (){
    using namespace std; 

    sayHello();

    int x = 23; 
    cout << "El número " << x << " pasado por la función add() es " << add(x, 12) << endl; 

    return 0; 
}
