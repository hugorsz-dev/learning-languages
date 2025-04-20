import math;


def sumar (n1,n2):
    return n1+n2; 

def restar (n1,n2):
    return n1-n2; 

def multiplicar (n1,n2):
    return n1*n2; 

def raiz (n):
    return math.sqrt(n);

class aparatoVolador ():
    
    ### CONSTRUCTOR 
    #### En el constructor declaramos también los atributos.
    
    def __init__ (self, marca, modelo, largoChasis, anchoChasis):
        self.marca = marca; 
        self.modelo = modelo;
        self.largoChasis = largoChasis; 
        self.anchoChasis = anchoChasis; 
        #### Por defecto: 
        self.numeroRuedas = 4 ;  
        self.enMarcha = False; 
        #### Privados 
        self.__matricula ="BFNAJFP"; # Esto no se heredará.
        
    
    ### MÉTODOS 
    #### Los métodos se distinguen de las funciones en cuanto a que estan restringidas a la clase.
    #### El parametro self define a la funcion como un metodo de la clase, y es necesario ponerlo.
    def arrancar (self):
        if not self.enMarcha: 
            self.enMarcha = True; # Declarar "enMarcha" como true, siempre con el "self" para hacer referencia a la propia clase
        else: 
            return "El coche ya está arrancado"; 
    
    
    def estado (self):
        print("Marca: ", self.marca,"\nModelo: ",self.modelo,  "\nLargo chasis: ", self.largoChasis, "\nAncho chasis: ", self.anchoChasis, "\nNúmero de ruedas: ", self.numeroRuedas, "\nEn marcha: ", self.enMarcha); 
        
    #### Para tener un método vacío podemos usar "pass", como con los bucles. 
    def metodoVacio (self):
        pass