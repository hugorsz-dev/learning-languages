"""
Las interfaces gráficas o GUI pueden mostrarse a través de librerías
de python que acceden a intermediarios, algunas de estas librerías
son: 

- Tkinter  (este será en el que aprendamos)
- WxPython 
- PyQT
- PyGTK 

Estructura: 
RAÍZ (tk) es el marco
Dentro de la raíz, existirá un FRAME (que se considera un disget)
Dentro del FRAME, podremos encontrar los WIDGETS (desplegables, ventanas, botones...

"""

# RAÍZ ------------------

# Importar la librería tkinder

from tkinter import *; 
from inkex.elements._svg import width, height

# Asignar un objeto para controlar la raiz

raiz= Tk() # Método raiz de funcionamiento

# Configurar la raíz

raiz.title ("Ventana de prueba") # Insertar título

# raiz.iconbitmap("icono.ico") # Asignar un icono a la aplicación

raiz.resizable(True, True) # Tanto ancho como alto no se pueden redimensionar

raiz.geometry("800x600") # Dimensiones a la pantalla

raiz.config (bg="gray") # Fondo

# (Si no queremos usar una consola de comandos mientras se abre la interfaz, renombrar archivo a .pyw)

# FRAME ------------------
# -> La config puede aplicarse también a la raíz, si así se desea

miFrame = Frame (); 

# El método pack sirve para acoplar el frame y adecuar la raiz a sus dimensiones

miFrame.pack(
    side="top",
    fill="y", 
    expand="True")  

# El side controla el alineamiento del frame (right, left, bottom, top)
# El anchor el segundo alineamiento: (n, s, e, w) (norte, sur, este, oeste)
# El fill es como el "100%" de CSS, puede ser para "x" (fill="x") o para "y" (fill="y", expand="True")

miFrame.config(
    bg="white", 
    width="700px", 
    height="500px", 
    relief="groove",
    bd="10"
    )

# Widget label -> puede mostrar texto e imagenes con esta etiqueta

tituloInterfaz = Label (miFrame, text="Ejemplo interfaz gráfica", font=("Liberation Sans", 23))

# El método place facilita la introducción de elementos absolutos

tituloInterfaz.place(x=220, y=0)

imagen = PhotoImage (file="imagen.png")

imagen_reducida = imagen.subsample(3, 3)# Se multiplica el ancho y el alto *1.n segun parametro
Label(miFrame, image=imagen_reducida).place(x=250, y=50)

raiz.mainloop() # El main loop es el bucle infinito de muestreo de ventana, implica el despliegue

"""
# Cuadro de texto

miFrame.columnconfigure(200, weight=1)  # Ajustar el ancho de la primera columna
miFrame.columnconfigure(1, weight=1)  # Ajustar el ancho de la segunda columna

cuadro_nombre = Entry (miFrame)
cuadro_nombre.grid (row=0,column=1)
Label (miFrame, text="Nombre").grid (padx=5, sticky="e", row=0, column=0)

cuadro_apellido = Entry (miFrame)
cuadro_apellido.grid (row=1,column=1)
Label (miFrame, text="Apellido").grid (padx=5,sticky="e", row=1, column=0)

cuadro_password = Entry (miFrame)
cuadro_password.grid (row=2,column=1)
Label (miFrame, text="Contraseña").grid (padx=5,sticky="e", row=2, column=0)

cuadro_password.config (show="*")

"""


