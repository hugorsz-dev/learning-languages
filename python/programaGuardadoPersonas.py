import pickle; # Trabajar con ficheros binarios
from io import *; # Trabajar con archivos
import os.path # Trabajar con el sistema operativo (para saber si el archivo existe)

class Persona ():
    def __init__ (self, nombre, sexo, edad):
        self.nombre = nombre; 
        self.sexo = sexo;
        self.edad = edad; 
        print ("Creada una persona con el nombre {}".format(self.nombre))
        
    def __str__ (self):
        return ("Nombre: {} - Sexo: {} - Edad: {}".format (self.nombre, self.sexo, self.edad))


class registradorPersonas ():
    def __init__ (self, nombreArchivo):
        
        # Lista donde serán guardadas las personas
        self.listaPersonas = []; 
        
        # Nombre asociado al archivo
        self.nombreArchivo = nombreArchivo; 
        
        # Cargar información
        
        if os.path.exists(self.nombreArchivo):
            # Si el archivo existe, cargarlo a la lista actual
            print (f"El archivo {self.nombreArchivo} ya existe")
            
            # Si el archivo existe, pero no contiene información, omitir el cargado (evita error)
            try:
                archivo_guardado = open (self.nombreArchivo, "rb")
                self.listaPersonas = pickle.load(archivo_guardado)
                print (f"Información cargada con éxito, {len(self.listaPersonas)} personas registradas")
            except: 
                print ("Fichero vacío")
        else:
            # Si el archivo no existe, crearlo.
            print (f"Creando fichero de guardado para {self.nombreArchivo}")
            open (self.nombreArchivo, "wb").close
        
    def registrarPersona (self, persona):
        
        # Agrega las personas a la lista
        self.listaPersonas.append(persona)
        
        # Carga el archivo
        archivo_guardado=open (self.nombreArchivo, "wb")
        
        # Vuelca la lista en el archivo
        pickle.dump(self.listaPersonas, archivo_guardado)
        
        archivo_guardado.close();

    def mostrarPersonas (self):
        for persona in self.listaPersonas:
            print (persona)
            
registrador = registradorPersonas ("archivo_guardado_personas")
registrador.registrarPersona(Persona ("Antonio", "Masculino", 43))
registrador.mostrarPersonas()
