/*
Colecciones comunes
La biblioteca estándar de Rust incluye varias estructuras de datos muy útiles llamadas colecciones . La mayoría de los demás tipos de datos representan un valor específico, pero las colecciones pueden contener múltiples valores. A diferencia de los tipos de matriz y tupla integrados, los datos a los que apuntan estas colecciones se almacenan en el montón, lo que significa que no es necesario conocer la cantidad de datos en tiempo de compilación y puede aumentar o disminuir a medida que se ejecuta el programa. Cada tipo de colección tiene diferentes capacidades y costos, y elegir la más adecuada para su situación actual es una habilidad que desarrollará con el tiempo. En este capítulo, analizaremos tres colecciones que se usan con frecuencia en los programas de Rust:

Un vector le permite almacenar una cantidad variable de valores uno al lado del otro.
Una cadena es una colección de caracteres. Ya mencionamos el Stringtipo, pero en este capítulo lo profundizaremos.
Un mapa hash permite asociar un valor con una clave específica. Es una implementación particular de una estructura de datos más general llamada mapa .
Para obtener más información sobre los otros tipos de colecciones proporcionadas por la biblioteca estándar, consulte la documentación .

Discutiremos cómo crear y actualizar vectores, cadenas y mapas hash, así como también qué hace que cada uno sea especial.
 */
pub mod vectores;

fn main() {
    vectores::vector();
}
