# El trabajo con los archivos de python
print ("------------- Archivos de texto plano");
## Concepto 
"""
Los ficheros de python deben pasar por varias etapas: 
[Creación] -> Apertura -> Manipulación -> Cierre
(Depende de si el archivo está creado o no)
"""
## Crear un archivo de texto, introduciremos texto, leeremos texto, y lo cerraremos

### Importar el módulo para hacerlo funcionar

from io import *; 

#### Crear un archivo y escribirlo (directiva write)

mi_archivo = open ("archivo.txt", "w"); # Esto crea nuestro archivo, borrando el que ya existía 

mi_archivo.write ("Esto es un ejemplo de txto \n HOLA MUNDO");

mi_archivo.close(); # Liberar espacio en la memoria

### Introducir nuevo contenido a un archivo

mi_archivo = open ("archivo.txt", "a");

mi_archivo.write ("\n Esto es una línea más abajo en el archivo, introducida con append, por lo que no hemos borrado el archivo en el proceso")

### Leer un archivo

mi_archivo = open ("archivo.txt", "r");

print (mi_archivo.read()) # Lee la totalidad del archivo, y la saca como output. Permite un parametro, que es el caracter limite, si no se coloca, lee todo el archivo desde la posicion del puntero

#### El método readlines() almacena en una lista las líneas del archivo, para que puedan ser consultadas fácilmente

##### Debemos saber que la lectura se hace por medio de punteros, y que cuando la lectura finaliza, estos deben reubicarse al principio del string: 

mi_archivo.seek(0) # Numero de caracter

lineas_texto = mi_archivo.readlines(); 

mi_archivo.close();

### Hasta ahora hemos trabajado con "r", "a", y "w". Podemos usar "r+" para tener todo

mi_archivo = open ("archivo.txt", "r+");

#### Si modificamos la lista previamente almacenada y la reintroducimos mediante writelines, podremos modificar líneas concretas del archivo, separandolas

lineas_texto[0] ="Linea modificada";
lineas_texto.append("\nNueva linea jejejejeje");

mi_archivo.writelines(lineas_texto);

mi_archivo.close();

# Serialización de listas

print ("------------- Serialización de listas");

## Concepto 

"""
La serialización nos permite el trabajo con archivos binarios, depositándolos en estructuras
de fácil y rápido acceso; es el metodo preferente para guardar listas, por ejemplo
"""

## Importar la librería necesaria

import pickle

## Volcado de información

### Crear una lista con nombres y serializarlas

lista_nombres = ["Hugo", "Marcos", "Rodrigo", "Mario", "Adrián"];

###  Vincular un archivo de escritura binaria ( write binary) 

archivo_binario=open ("lista_nombres", "wb")

### Volcar la lista creada en el archivo

pickle.dump(lista_nombres, archivo_binario)

archivo_binario.close();

## Leer información

archivo_binario=open ("lista_nombres", "rb")

lista_nombres_recibida = pickle.load(archivo_binario)

print (lista_nombres_recibida)

archivo_binario.close()

# Serialización de objetos

print ("------------- Serualización de objetos");

## Objeto de ejemplo 

efectividadAtaque = {
    "agua": {"agua": 0.5, "fuego": 2, "planta": 2, "electrico": 1},
    "fuego": {"agua": 2, "fuego": 0.5, "planta": 2, "electrico": 1},
    "planta": {"agua": 2, "fuego": 1, "planta": 0.5, "electrico": 1},
    "electrico": {"agua": 2, "fuego": 1, "planta": 1, "electrico": 0.5},
}

class Coche ():
    
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
        
    def arrancar (self):
        if not self.enMarcha: 
            self.enMarcha = True; # Declarar "enMarcha" como true, siempre con el "self" para hacer referencia a la propia clase
        else: 
            return "El coche ya está arrancado"; 
    
    def estado (self):
        print("Marca: ", self.marca,"\nModelo: ",self.modelo,  "\nLargo chasis: ", self.largoChasis, "\nAncho chasis: ", self.anchoChasis, "\nNúmero de ruedas: ", self.numeroRuedas, "\nEn marcha: ", self.enMarcha); 

coches = [Coche ("Tosh", "A50", 12,23),
          Coche ("Azda", "PLUS", 55,22),
          Coche ("Janso", "Elektro", 9,43)]

## Serializando

### Crear el archivo y grabar la lista de objetos

archivo_objeto_coches = open ("archivo_objeto_coches","wb")

pickle.dump(coches,archivo_objeto_coches)

archivo_objeto_coches.close ()

## Cargar el archivo y recoger la lista de objeto
archivo_objeto_coches = open ("archivo_objeto_coches","rb")

coches_recibidos = pickle.load(archivo_objeto_coches);

for coche in coches_recibidos:
    print (coche.modelo)
    
## Es importante tener en cuenta que la serialización de objetos no traslada la informacion esructural del objeto, solo su información; el programa debe tener la estructura del objeto clara (el objeto coches creado, en este caso) para funcionar







