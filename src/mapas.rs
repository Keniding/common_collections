/*
Almacenamiento de claves con valores asociados en mapas hash
La última de nuestras colecciones comunes es el mapa hash. El tipo HashMap<K, V> almacena una asignación de claves de tipo Ka valores de tipo Vmediante una función hash , que determina cómo se colocan estas claves y valores en la memoria. Muchos lenguajes de programación admiten este tipo de estructura de datos, pero suelen usar un nombre diferente, como hash , mapa , objeto , tabla hash , diccionario o matriz asociativa , por nombrar solo algunos.

Los mapas hash son útiles cuando se buscan datos no mediante un índice, como ocurre con los vectores, sino mediante una clave de cualquier tipo. Por ejemplo, en un partido, se podría registrar la puntuación de cada equipo en un mapa hash donde cada clave es el nombre del equipo y los valores son la puntuación de cada equipo. Dado el nombre de un equipo, se puede recuperar su puntuación.

En esta sección, repasaremos la API básica de los mapas hash, pero las funciones definidas HashMap<K, V>por la biblioteca estándar ofrecen muchas más ventajas. Como siempre, consulta la documentación de la biblioteca estándar para obtener más información.

Creación de un nuevo mapa hash
Una forma de crear un mapa hash vacío es usar newy para agregar elementos con insert. En el Listado 8-20, registramos las puntuaciones de dos equipos llamados Azul y Amarillo . El equipo Azul empieza con 10 puntos y el equipo Amarillo con 50.

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
Listado 8-20 : Creación de un nuevo mapa hash e inserción de algunas claves y valores
Tenga en cuenta que primero debemos obtener usela HashMapsección de colecciones de la biblioteca estándar. De nuestras tres colecciones comunes, esta es la que menos se usa, por lo que no se incluye en las funciones que se incluyen automáticamente en el preludio. Los mapas hash también tienen menos compatibilidad con la biblioteca estándar; por ejemplo, no hay una macro integrada para construirlos.

Al igual que los vectores, los mapas hash almacenan sus datos en el montón. Este HashMapcontiene claves de tipo Stringy valores de tipo i32. Al igual que los vectores, los mapas hash son homogéneos: todas las claves y todos los valores deben ser del mismo tipo.

Acceder a valores en un mapa hash
Podemos obtener un valor del mapa hash proporcionando su clave al get método, como se muestra en el Listado 8-21.

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
Listado 8-21 : Acceso a la puntuación del equipo Azul almacenada en el mapa hash
Aquí, scorese tendrá el valor asociado con el equipo Azul y el resultado será 10. El getmétodo devuelve un Option<&V>; si no hay ningún valor para esa clave en el mapa hash, getdevolverá None. Este programa gestiona esto Optionllamando copieda para obtener un , Option<i32>en lugar de un Option<&i32>, y luego unwrap_orestableciéndolo scorea cero si scoresno hay una entrada para la clave.

Podemos iterar sobre cada par clave-valor en un mapa hash de manera similar a como lo hacemos con los vectores, usando un forbucle:

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
Este código imprimirá cada par en un orden arbitrario:

Yellow: 50
Blue: 10

Gestión de la propiedad en mapas hash
Para los tipos que implementan el Copyatributo, como i32, los valores se copian en el mapa hash. Para los valores propios, como String, se moverán y el mapa hash será el propietario de dichos valores, como se muestra en el Listado 8-22.

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
Listado 8-22 : Muestra que las claves y los valores son propiedad del mapa hash una vez que se insertan
No podemos utilizar las variables field_namey field_valuedespués de que se hayan movido al mapa hash con la llamada a insert.

Si insertamos referencias a valores en el mapa hash, estos no se moverán a él. Los valores a los que apuntan las referencias deben ser válidos al menos mientras el mapa hash sea válido. Hablaremos más sobre estos temas en "Validación de referencias con tiempos de vida" en el capítulo 10.

Actualización de un mapa hash
Aunque la cantidad de pares de clave y valor se puede aumentar, cada clave única solo puede tener un valor asociado a la vez (pero no al revés: por ejemplo, tanto el equipo Azul como el equipo Amarillo podrían tener el valor 10 almacenado en el scoresmapa hash).

Al cambiar los datos de un mapa hash, debe decidir cómo gestionar el caso en que una clave ya tenga un valor asignado. Puede reemplazar el valor anterior por el nuevo, ignorándolo por completo. Puede conservar el valor anterior e ignorar el nuevo, añadiendo este solo si la clave aún no tiene un valor. O bien, puede combinar el valor anterior con el nuevo. ¡Veamos cómo hacerlo!

Sobrescribir un valor
Si insertamos una clave y un valor en un mapa hash y luego insertamos esa misma clave con un valor diferente, se reemplazará el valor asociado a esa clave. Aunque el código del Listado 8-23 se invoca insertdos veces, el mapa hash solo contendrá un par clave-valor, ya que estamos insertando el valor de la clave del equipo Azul en ambas ocasiones.

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
Listado 8-23 : Reemplazo de un valor almacenado con una clave particular
Este código imprimirá {"Blue": 25}. El valor original de 10ha sido sobrescrito.


Agregar una clave y un valor solo si no hay una clave presente
Es común verificar si una clave particular ya existe en el mapa hash con un valor y luego tomar las siguientes acciones: si la clave existe en el mapa hash, el valor existente debe permanecer como está; si la clave no existe, insertarla y un valor para ella.

Los mapas hash tienen una API especial para esto, llamada ``` entry, que toma la clave que se desea verificar como parámetro. El valor de retorno del entrymétodo es una enumeración llamada ``` Entryque representa un valor que podría existir o no. Supongamos que queremos verificar si la clave del equipo amarillo tiene un valor asociado. Si no lo tiene, insertamos el valor ``` 50, y lo mismo para el equipo azul. Usando la entryAPI, el código se parece al Listado 8-24.

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
Listado 8-24 : Uso del entrymétodo para insertar solo si la clave aún no tiene un valor
El or_insertmétodo on Entryestá definido para devolver una referencia mutable al valor de la Entryclave correspondiente si dicha clave existe. De no existir, inserta el parámetro como el nuevo valor de dicha clave y devuelve una referencia mutable al nuevo valor. Esta técnica es mucho más sencilla que escribir la lógica nosotros mismos y, además, se integra mejor con el verificador de préstamos.

Al ejecutar el código del Listado 8-24, se imprimirá {"Yellow": 50, "Blue": 10}. La primera llamada a entryinsertará la clave del equipo Amarillo con el valor, 50ya que este no tiene un valor. La segunda llamada a entryno modificará el mapa hash, ya que el equipo Azul ya tiene el valor 10.

Actualización de un valor basado en el valor anterior
Otro uso común de los mapas hash es consultar el valor de una clave y actualizarlo según el valor anterior. Por ejemplo, el Listado 8-25 muestra código que cuenta cuántas veces aparece cada palabra en un texto. Usamos un mapa hash con las palabras como claves e incrementamos el valor para registrar cuántas veces la hemos visto. Si es la primera vez que vemos una palabra, primero insertamos el valor 0.

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
Listado 8-25 : Conteo de ocurrencias de palabras usando un mapa hash que almacena palabras y las cuenta
Este código imprimirá {"world": 2, "hello": 1, "wonderful": 1}. Es posible que veas los mismos pares clave-valor impresos en un orden diferente: Recuerda, en "Acceso a valores en un mapa hash" , que la iteración sobre un mapa hash se realiza en un orden arbitrario.

El split_whitespacemétodo devuelve un iterador sobre subsecciones, separadas por espacios, del valor en text. El or_insertmétodo devuelve una referencia mutable ( &mut V) al valor de la clave especificada. Aquí, almacenamos esa referencia mutable en la countvariable, por lo que para asignarla a ese valor, primero debemos desreferenciarla countmediante el asterisco ( *). La referencia mutable queda fuera del ámbito al final del forbucle, por lo que todos estos cambios son seguros y permitidos por las reglas de préstamo.

Funciones hash
De forma predeterminada, HashMaputiliza una función hash llamada SipHash que puede proporcionar resistencia a ataques de denegación de servicio (DoS) que involucran tablas hash 1 . Este no es el algoritmo hash más rápido disponible, pero la compensación por una mejor seguridad que viene con la caída en el rendimiento vale la pena. Si perfila su código y encuentra que la función hash predeterminada es demasiado lenta para sus propósitos, puede cambiar a otra función especificando un hasher diferente. Un hasher es un tipo que implementa el BuildHashertrait. Hablaremos sobre traits y cómo implementarlos en el Capítulo 10 . No necesariamente tiene que implementar su propio hasher desde cero; crates.io tiene bibliotecas compartidas por otros usuarios de Rust que proporcionan hashers que implementan muchos algoritmos hash comunes.

Resumen
Los vectores, cadenas y mapas hash proporcionan una gran cantidad de funcionalidad necesaria en los programas para almacenar, acceder y modificar datos. Aquí tienes algunos ejercicios que ya deberías poder resolver:

Dada una lista de números enteros, utilice un vector y devuelva la mediana (cuando se ordena, el valor en la posición media) y la moda (el valor que aparece con mayor frecuencia; un mapa hash será útil aquí) de la lista.
Convierte cadenas a latín pig. La primera consonante de cada palabra se mueve al final y se añade ay , por lo que first se convierte en irst-fay . A las palabras que empiezan por vocal se les añade hay al final ( apple se convierte en apple-hay ). ¡Ten en cuenta los detalles sobre la codificación UTF-8!
Usando un mapa hash y vectores, cree una interfaz de texto que permita al usuario agregar nombres de empleados a un departamento de una empresa; por ejemplo, "Agregar a Sally a Ingeniería" o "Agregar a Amir a Ventas". Luego, permita al usuario recuperar una lista de todas las personas de un departamento o de todas las personas de la empresa por departamento, ordenadas alfabéticamente.
La documentación de la API de la biblioteca estándar describe métodos que tienen los vectores, cadenas y mapas hash que serán útiles para estos ejercicios.

Nos adentramos en programas más complejos en los que las operaciones pueden fallar, así que es el momento perfecto para hablar sobre la gestión de errores. ¡Lo haremos a continuación!
 */
use std::collections::HashMap;

pub fn mapas() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{scores:?}");

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    // Iteración
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{map:?}");
    println!(
        "{:?}",
        map.entry(String::from("Favorite language")).or_insert(String::from("Rust"))
    );

    // Sobrescribir
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25); // Duplicate hash key
    println!("{scores:?}");

    // entry representa un valor que puede ser insertado en el mapa o actualizado.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    // Actualización basado en un valor anterior
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}