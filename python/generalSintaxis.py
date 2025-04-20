# Importar paquetes con funciones: 
print ("------------- Paquetes");

## Indicar paquete de proveniencia - si procede - y luego el import. 

from estudio import moduloMiscelaneo

## Puede importarse la función concreta. 

from estudio.moduloMiscelaneo import raiz; 

## O todo 

from estudio.moduloMiscelaneo import *; 

## Instanciamos un objeto con la clase aparatoVolador ()

miAparato = aparatoVolador ("INC", "Zhugors", 120, 5); 

miAparato.estado();

# Imprimir información en pantalla
print ("------------- Imprimir info en pantalla");

## Imprimir información

print ("Hola mundo"); 

## Imprimir información sin salto de línea 

print ("Adiós mundo", end=""); # Siendo "end" con lo que terminará la cadena

print ("... ¡que no!");

## Crear variables 

miNombre = "Hugo";

## Imprimir la variable en pantalla 

print ("Mi nombre es "+miNombre);

# Operadores 
print ("------------- Operadores");

## Diferentes operadores

suma = 5+3; # 8 
resta = 5-3; # 2
multiplicacion = 5*3; # 15
division = 5/3; # 1.6666
divisionEntera = 5//3 # 1
potencia = 5**3; # 125

## Tipos de las variables
### Note que las variables numéricas están definidas por el contenido, no por el contenedor.
###Esto significa que la variable "suma" será de numero entero desde el momento en que la asociamos a este.

print (type (miNombre)); # str 
print (type (suma)); # int 
print (type (division)); # float

    
# Funciones 
print ("------------- Funciones");

## Función para imprimir hola mundo
### Las funciones no tienen cierre ¡bah!

def decirHolaMundo (): 
    print ("Hola mundo ¡esto es una función!");

decirHolaMundo();

## Funciones con parámetro

def sumarNumero (num1, num2):
    return num1+num2; 

print (sumarNumero(2, 3));

## * sirve para indicar que puede recibir uno o mas elementos (Mas elementos en forma de tupla)

def mostrarElementos (*lista):
    for i in lista:
        print (i);
        
mostrarElementos("Rosa","Javier","Sara");
    

# Condicionales 
print ("------------- Condicionales");

## Condicional sencillo. 

if suma > resta: 
    print ("El número asociado a la suma es mayor que la resta"); 
else:
    print ("El número 2 es mayor");
    
## Función con condicional 

def evaluacion (): 
    nota =int (input("Introduzca la nota del alumno: ")); # El input interpretará la entrada por teclado como un string. Esto arrojará un error.
    salida = "Aprobado"; 
    if nota<5: 
        salida ="Suspenso"
    return salida;
 
"""
print (evaluacion());
"""

## Función con else y ele if 

def mayorDeEdad ():
    
    edad = int ( input ("Introduzca su edad: ") ); 
    
    if edad<18:
        print ("Tienes menos de 18 años, no puedes pasar");      
    elif edad>100:
        print ("Edad incorrecta"); 
        
    else:
        print ("Enhorabuena, tienes más de 18 años"); 
        return True;
        
    return False;
"""
mayorDeEdad();
"""

## Operadores de comparacion 
### A diferencia de otros lenguajes, permite emplear varios operadores de comparación a la vez. 

def edadCorrecta ():
     
    edad = int ( input ("Introduzca su edad: ") ); 
    
    if 0<edad<100: 
        print ("Edad es correcta"); 
    else:
        print ("Edad es incorrecta"); 
"""    
edadCorrecta()
"""

## Ejercicio con operadores de comparación
### Cuatro clases de empleado: jefe, secretario, empleado y limpiadora. Cada cual con un salario mas bajo. Si no es asi, false. 

def comprobarSalarios ():
    print ("Iniciada utilidad para comprobar salarios"); 
    
    salarioJefe = int (input ("Introduzca el salario del jefe"));
    salarioSecretario = int (input ("Introduzca el salario del secretario"));
    salarioEmpleado = int (input ("Introduzca el salario del empleado"));
    salarioLimpiadora = int (input ("Introduzca el salario la limpiadora"));
    
    if salarioJefe>salarioSecretario>salarioEmpleado>salarioLimpiadora:
        return True; 
    else: 
        return False; 
"""
print (comprobarSalarios());
"""

## Ejercicio con operadores AND 
### Herramienta para saber si tienes una beca: solo +2 hermanos +40km y salario <=20000

def comprobarBeca (hermanos, distancia, salario):
    
    if hermanos>2 and distancia>40 or salario<=20000:
        return True; 
    else:
        return False;

print (comprobarBeca (3,50,12));

## Ejercicio con operadores IN 
### Es parecido a un "==" con multiples resultados, fue empleado en los arrays

curso = "DAW"; 
if curso in ("DAW","ASIR","DAM"):
    print ("Curso reconocido")
else:
    print ("Curso no reconocido"); 
    
# Listas (Arrays dinámicos)
print ("------------- Listas");

miLista = [] # Lista vacía, inicialización

miLista = ["Rojo", "Verde", "Azul", "Morado"]; 

## Mostrar lista 
### Imprime la lista a modo de resumen

print (miLista [:]); # También sirve print (miLista); 

## Mostrar elemento de la lista 

print (miLista [3]); 

## Acceso a la inversa 
### Rojo (-4), Verde (-3), Azul (-2), Morado (-1)

print (miLista [-3]);

## Acceso a rangos de información
### Datos entre la posición 1 y 3

print (miLista[1:3]); 

print (miLista[:3]); # Es decir, de 0:3

print (miLista [3:]); # Es decir, de 3:n (siendo "n" el ultimo elemento)

## Añadir elementos a una lista

miLista.append("Rosa");  # Introduce "rosa" al final del array. 

miLista.insert(2, "Naranja") # Introduce "Naranja" en una posicion determinada desplazando todos los elementos. 

miLista.extend(["Negro", "Blanco", "Violeta","Granate"]); # Extiende el array, requiere un array como argumento

miLista = miLista *3 # Triplica los elementos de la lista (Repite 3 veces la cadena)

miLista = miLista + ["Negro", "Blanco", "Violeta","Granate"]; # Lo mismo, pero concatenando listas

## Borrar elementos de una lista 

miLista.remove ("Negro"); # Borra el color negro. 

del miLista [0]; # Borra el color asociado al indice 0

miLista.pop() # Elimina el ultimo registro

print (miLista);

## Saber el índice de un determinado elemento de una lista 

print (miLista.index("Azul")); 

## Saber si un elemento se encuentra en la lista (true/false)

print ("Rosa" in miLista); 

## Contar la repetecion de ciertos elementos de una lista 

print (miLista.count("Negro")); # "Negro" está 3 veces en la lista

## Contar la longitud de una tupla 

print (len (miLista)); ## 31 elementos


# Tuplas 
print ("------------- Tuplas");

### Las tuplas son listas inmutables: no se pueden añadir, eliminar ni mover elementos.
### Permiten las búsquedas por índice (ultimas versiones), si un elemento está presente y rangos de busqueda
### Sus ventajas son: mas rapidas, ocupan menos memoria, pueden usarse como claves en un diccionario
### Compatible con todos los métodos de visionado anteriormente vistos. 

miTupla = ("Javier", 13, 1, 1995); # Se hace como un array, pero con "(" ")" o sin ellos.

## Imprimir tupla 

print (miTupla); 
print (miTupla[1]);
print (miTupla[2:]);

## Saber si un elemento se encuentra en la tupla

print ("Javier" in miTupla); 

## Convertir tupla a lista

miLista = list(miTupla);

## Convertir lista a tupla 

miTupla = tuple(miLista);

## Tupla unitaria
### Tiene un solo elemento, por lo que debe ir indicado con una coma. 

tuplaUnitaria = ("Elemento",); 

## Desempaquetado de tupla
### Las variables se asignan respectivamente a la posición que tendrian en la tupla. 

nombre, dia, mes, agno = miTupla; 

print (nombre);
print (dia); 
print (mes);
print (agno); 

# Diccionarios 
print ("------------- Diccionarios");

## Los diccionarios son similares a los hashmaps, con una asociación de clave:valor. 

miDiccionario={"Alemania":"Berlín","Francia":"París", "Reino Unido":"Londres", "España":"Madrid"}

## Consultar clave

print (miDiccionario["Francia"]);

## Añadir elemento 

miDiccionario["Italia"]="Lisboa"; 

## Modificar valor 

miDiccionario["Italia"]="Roma"; 

## Eliminar valor 

del miDiccionario["Italia"]; 

## Diccionario con varias claves y valores
### Podemos introducir, literalmente, cualquier variable o tipo de mapa (diccionarios, arrays, tuplas...)
miDiccionario = {23:"Jordan","Nombre":"Michael", "Equipo":"Chicago","Trofeos":[1991,1992,1993,1996,1997,1998]}

print (miDiccionario["Trofeos"]);

## Longitud del diccionario 

print (len(miDiccionario)); 


# Bucles 
print ("------------- Bucles");

## Bucle for para recorrer listas
### Asociándose "i" al valor que contiene en cada iteración de la lista.
for i in miLista:
    print (i);
    
## Bucle for para recorrer strings
### El tratamiento del string es como de un array de caracteres. 
for i in "Hugo":
    print (i)
    
## Bucle for con contadores 
### Como en los lenguajes clásicos, establece un sistema de contador 

for i in range(5):
    print ("El bucle ha dado "+ str(i) +" vueltas");


## Bucle while
contador =0; 
while contador<10:
    contador+=1; # contador = contador+1; 
    print (contador); 

## Bucle con funciones 

"""
contador =0;
while not mayorDeEdad() and contador<2:
    contador+=1;
    print ("Vuelva a intentarlo"); 
    
if contador>=2:
     print ("Número de intentos superado"); 
"""

## Función continue (común a todos los bucles)
### Salta a la siguiente iteración

for letra in "Hugo":
    
    if (letra=="u"): # Se salta la "u", no mostrándose en pantalla
        continue
    print (letra); 

def contarCaracteres (texto):
    salida =0;
    for letra in texto: 
        if letra==" ":
            continue
        salida+=1
    return salida; 

print (contarCaracteres("Hola, soy Hugo")); 

## Función break, rompe el bucle
contador=0;
while True: # Bucle infinuto
    print (contador);
    contador+=1
    if contador>5:
        break; 
    
## Junto con la función break, usar el else
### Permite ejecutar un codigo SOLO NO se ha interrumpido

email="hugoruinsanchezgmail.com"
arroba = True;
for caracter in email: 
    if caracter=="@":
        break; # Si encuentra la arroba, se cancela el bucle, incluyendo el else. Si no la encuentra, llega al final y carga el else, siendo este la asignacion a false
else: 
    arroba=False;
    
print(arroba);

# Generadores 
print ("------------- Generadores");
###  Los generadores son funciones iterables, capaces de devolver valores 
### concretos aun cuando estan programadas para presentar miles de numeros
### de modo que se reduce la carga en memoria y permite el flujo de datos infinitos
### Es como una funcion que se comporta como una lista.

## Función que genera números pares

def listaPares(limite):
    
    salida = []; 
    
    for i in range(limite*2):
        
        if i%2==0:
            salida.append(i);
            
    return salida;
            
print (listaPares(8));
 
## Generador que genera numeros pares
### El generador ya tiene implícita la lista, a la que se accede por yield

def generarPares(limite):
    
    for i in range(limite*2):
        
        if i%2==0:
            yield i
            
""" Esta función sería infinita, y mas compatible con generador
def generarPares():
    
    contador =0; 
    while True: 
        if contador%2==0: 
            yield contador;
    contador+=1;
"""

### Depositar generador en un objeto, en el que se encontrará la información

objetoPares = generarPares(8); 

### LLamar al objeto para obtener la información cada vez. 

print (next(objetoPares)); # Sucesivamente podria ir sacando mas pares hasta el limite.   

### Iterar el objeto 

for i in objetoPares: 
    print (i); 
    
## Yield from: profundizar en los elementos
### Con una sintaxis así, profundizaríamos en el elemento "ciudad" para hallar cada una compuesta de caracteres
"""
def devolverCiudades (*ciudades):
    for ciudad in ciudades: 
        for caracter in ciudad:
            yield caracter;
"""
### Para ahorrar este proceso, podemos usar yield from

def devolverCiudades (*ciudades):
    for ciudad in ciudades: 
        yield from ciudad; 
        
ciudades = devolverCiudades ("Berlín","Madrid"); 

print (next(ciudades));
print (next(ciudades));

# Excepciones 
print ("------------- Excepciones");
### Las excepciones son errores en tiempo de ejecución, para omitir los errores. 

## Error típico. 
dividendo = 5;
divisor = 0;

try:
    print (dividendo/divisor);
except ZeroDivisionError: 
    print ("No se puede dividir entre 0");

## Error en bucle
"""
while True:
    try:
        dividendo=int (input("Introduzca dividendo: "));   
        divisor=int (input("Introduzca divisor: "));
        break; # Si llega hasta aquí - no hay error - entonces se rompe el bucle.
    except ValueError: 
        print ("¡No ha introducido números!"); 
"""

## Varios errores a la vez.
"""
try:
    dividendo=int (input("Introduzca dividendo: "));   
    divisor=int (input("Introduzca divisor: "));
    print (dividendo/divisor);
except ValueError:
    print ("¡No ha introducido números!"); 
except ZeroDivisionError:
    print ("No se puede dividir entre 0");
""" 
## Error cualquiera - Finally: siempre se ejecuta ¡inclusi cuando ocurre un error! 

try:
    print (dividendo/divisor);
except:
    print ("Ha ocurrido un error");
finally:
    print ("Fin de la ejecución del programa");

## Lazar la excepcion que queramos
'''
def eresGay (respuesta):
    
    if respuesta==True:
        raise TypeError ("¡NO ERES GAY!"); 
    else:
        print ("No, no eres gay");
        
eresGay(True); 
'''
### Función que calcula raiz con excepcion personalizada

def calculaRaiz (numero):
    if numero<0:
        raise ValueError ("El número no puede ser negativo")
    else:
        return raiz(numero);  

### Encapsular la funcion en un try

try:
    calculaRaiz(-123); 
except ValueError as ErrorDeNumeroNegativo: 
    print (ErrorDeNumeroNegativo); # Permite imprimir el texto programado para ese error (mas arriba)
   
# Clases 
### Es bueno que las clases estén separadas en forma de módulos (archivos.py)
### a fin de organizar el código y facilitar su reutilización lo más posible. 
print ("------------- Clases");

## Declaración de clase 
    
class Coche ():
    
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
    
## Instanciar clase 

miCoche = Coche ("Seat","Ibiza",120, 230); 

## Llamar a los métodos y consultar atributos. 

miCoche.estado()

print (miCoche.enMarcha); 

miCoche.arrancar(); 

print (miCoche.enMarcha); 

print (miCoche.arrancar())

## Herencia
### Clase moto hereda los métodos y propiedades de Coche. 

class Moto (Coche):
    
    ### Declarar un nuevo atributo (caballito) junto a un nuevo metodo que lo asigne. 
    ### Modificar constructor. 
    
    def __init__ (self, marca, modelo, largoChasis, anchoChasis):
        super().__init__(marca, modelo, largoChasis, anchoChasis); # Heredar todo lo que estaba en el anterior constructor
        self.numeroRuedas = 2; # Cambiamos el atributo por defecto de numero de ruedas a 2. 
     
    caballito = False; 

    def hacerCaballito (self):
        self.caballito = True;
        
    ### Sobreescribir método estado. 
    
    def estado (self):
        print("Caballito: ", self.caballito, "\nMarca: ", self.marca,"\nModelo: ",self.modelo,  "\nLargo chasis: ", self.largoChasis, "\nAncho chasis: ", self.anchoChasis, "\nNúmero de ruedas: ", self.numeroRuedas, "\nEn marcha: ", self.enMarcha); 


miMoto = Moto ("NRG","500",125,125); 
miMoto.hacerCaballito(); 
miMoto.estado();

## Herencia múltiple

class Electrico ():
    
    ### CONSTRUCTOR
    def __init__ (self,autonomia):
        self.autonomia = autonomia; 
        self.bateria = 0;
        
    ### MÉTODOS
    
    def cargarBateria (self):
        
        if self.bateria>=self.autonomia:
            raise ValueError ("Batería más grande que el límite"); 
        
        self.bateria = self.bateria +10; 
     
### Creación de la BicicletaElectrica, que auna todo. Por defecto, adquiere el constructor de la primera clase padre.
class BicicletaElectrica (Coche, Electrico):
    
    #### Para unificar ambos constructores:
    
    def __init__ (self,marca, modelo, largoChasis, anchoChasis, autonomia):
        super().__init__(marca, modelo, largoChasis, anchoChasis); 
        Electrico.__init__(self, autonomia); 
    
    #### Sobreescribir el método de estado. 
    
    def estado (self):
        super().estado() # Llamamos al estado original de Coche, para evitar reformularlo de nuevo
        print ("Autonomía: ",self.autonomia, "\nBatería: ", self.bateria); 
    
        
miBicicleta = BicicletaElectrica ("Sunnyside","Cycle",100, 200, 100); 
miBicicleta.cargarBateria()
miBicicleta.estado(); 

### Comprobar si un objeto se ha obtenido mediante herencia 

print (isinstance(miBicicleta, Coche));  # True: es una instancia de Coche
print (isinstance(miBicicleta, Electrico)); # True: tambien es una instancia de Electrico,
print (isinstance(miBicicleta, Moto)); # False: no es una instancia de Moto

## Polimorfismo
### Apelamos a un objeto hipotético dentro de una función para llamar a métodos

def verEstado (vehiculo):
    vehiculo.estado(); 
    
verEstado(miCoche);
verEstado(miMoto);
verEstado(miBicicleta);

# Métodos de cadenas https://pyspanishdoc.sourceforge.net/lib/string-methods.html
print ("------------- Algunos metodos cadenas");

miCadena = " hugo ruiz sánchez "; 

## upper () todo a mayúsculas 

print (miCadena.upper());

## lower () lo convierte a minúsculas 

print (miCadena.lower());

## capitalize () pone primera en mayúscula 

print (miCadena.capitalize())

## count () cuenta los caracteres indicados por parametro

print (miCadena.count("u"));

## find () indica el indice donde encuentra el parametro indicado

print (miCadena.find("ru"));

## rfind () ultima aparicion, sentido inverso al find

print (miCadena.rfind("ru"));

## isdigit () comprueba si es una cadena de numeros

print (miCadena.isdigit());

## isalnum () comprueba si es una cadena de caracteres y numeros (alfanumerico)

print (miCadena.isalnum());

## isalpha () comprueba si es una cadena de solo caracteres

print (miCadena.isalpha());

## split () Divide la cadena de caracteres cada vez que aparece un caracter determinado

print (miCadena.split(" ")); 

## strip () Quita espacio inicial y final

print (miCadena.strip())

## replace ()

print (miCadena.replace("u", "i"));

## Substring: funciona como un array.

print (miCadena[3:8]); 

## format (): sirve para sustituir los "{}" por las variables introducidas en orden

alimento = "pizza"

print ("Me gusta mucho la {} de cuatro quesos".format (alimento))

# Empaquetar módulos/paquetes distribuibles 

"""
(En un archivo setup.py FUERA del paquete - carpeta - que queremos empaquetar)
# Este archivo permite crear paquetes distribuibles, se ejecuta a través del comando 
## python setup.py sdist
# Lo cual generará - si se ejecuta correctamente - una carpeta dist donde se 
# encontrará el paquete distribuible en formato tar.gz
# Esto se tiene que instalar en nuestro python a través de: 
## pip3 install (nombre_del_paquete)
# siempre que lo importemos estará disponible, ¡como los paquetes preinstalados!
# Para desinstalarlo, solo tenemos que usar: 
## pip3 uninstall (nombre_del_paquete) 

# Cuando hablamos de PAQUETES estamos haciendo referencia a la carpeta donde están guardados nuestros py. 

from setuptools import setup 

setup (
    name ="paquetemiscelaneo", 
    version="1.0",
    description="Esto sirve para generar un paquete de prueba con el que compartir",
    author="Hugo Ruiz", 
    author_email="hugoruizsanchez@gmail.com",
    url="www.google.es",
    packages=["estudio"]    
    
)
"""


    
    
    
    
    
    
    
    
    
    
    
    
    
    
    