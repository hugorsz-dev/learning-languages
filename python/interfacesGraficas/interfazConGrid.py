# Importar librerías 

from tkinter import *; 
from inkex.elements._svg import width, height

# Asignar un objeto para controlar la raiz

raiz= Tk() 
raiz.geometry("500x300")  # Establece el tamaño de la ventana
    
# Configuración de raíz 

raiz.title ("Programa de gestión de bases de datos") ;
raiz.resizable(True, True);
raiz.config (bg="gray") 

# Configuración del frame 

miFrame = Frame (); 
miFrame.pack(fill="y", expand=True)
miFrame.config(bg="white", relief="groove", bd="10")

# Inserción del título 

Label (miFrame, text="Entrar en la base de datos", font=("Arial",25)).grid (pady=15,row=0, column=0,columnspan=2)

# Inserción de labels y cuadros

Label (miFrame, text="Usuario", font=("Arial",15)).grid (padx=5, sticky="w", row=1, column=0)
insertar_nombre = Entry (miFrame, font=("Arial",15))
insertar_nombre.grid (row=1,column=1)

Label (miFrame, text="Contraseña", font=("Arial",15)).grid (padx=5, sticky="w", row=2, column=0)
insertar_password = Entry (miFrame, font=("Arial",15))
insertar_password.grid (row=2,column=1)
insertar_password.config (show="*")


raiz.mainloop() 