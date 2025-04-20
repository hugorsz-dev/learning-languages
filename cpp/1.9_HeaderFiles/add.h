/*
 * Leer: lección 1.10, sobre las directivas para el preprocesador:
 * Para evitar que dos ".h" incluyan dos includes iguales por error (ya que produciría una duplicación del código),
 * utilizamos definiciones del prerocesador. Puesto que el preprocesador y sus directivas son comunes a todo el
 * proceso de compilación (globales), cuando le indicamos un condicional que verifique la existencia de la definición
 * evitamos la duplicación de los prototipos.
 *
 * Esta práctica se denomina "header guard"
*/

#ifndef ADD_H
#define ADD_H

int add(int x, int y); 

#endif 
