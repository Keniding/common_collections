/*
Almacenamiento de listas de valores con vectores
El primer tipo de colección que analizaremos es Vec<T>, también conocido como vector. Los vectores permiten almacenar más de un valor en una única estructura de datos que los coloca uno junto al otro en la memoria. Los vectores solo pueden almacenar valores del mismo tipo. Son útiles cuando se tiene una lista de artículos, como las líneas de texto de un archivo o los precios de los artículos de un carrito de compra.

Creando un nuevo vector
Para crear un nuevo vector vacío, llamamos a la Vec::newfunción, como se muestra en el Listado 8-1.

    let v: Vec<i32> = Vec::new();
Listado 8-1 : Creación de un nuevo vector vacío para contener valores de tipoi32
Tenga en cuenta que añadimos una anotación de tipo. Dado que no insertamos ningún valor en este vector, Rust desconoce qué tipo de elementos queremos almacenar. Este es un punto importante. Los vectores se implementan mediante genéricos; explicaremos cómo usarlos con sus propios tipos en el Capítulo 10. Por ahora, tenga en cuenta que el Vec<T>tipo proporcionado por la biblioteca estándar puede contener cualquier tipo. Al crear un vector para un tipo específico, podemos especificarlo entre corchetes angulares. En el Listado 8-1, le indicamos a Rust que la Vec<T>instrucción in vcontendrá elementos de ese i32tipo.

Con mayor frecuencia, crearás un vector Vec<T>con valores iniciales, y Rust inferirá el tipo de valor que quieres almacenar, por lo que rara vez necesitarás esta anotación de tipo. Rust proporciona la vec!macro, que crea un nuevo vector que contiene los valores que le asignaste. El Listado 8-2 crea un nuevo vector Vec<i32>que contiene los valores 1, 2y 3. El tipo entero se i32 debe a que es el tipo entero predeterminado, como explicamos en la sección "Tipos de datos" del Capítulo 3.

    let v = vec![1, 2, 3];
Listado 8-2 : Creación de un nuevo vector que contiene valores
Dado que hemos proporcionado i32valores iniciales, Rust puede inferir que el tipo de v es Vec<i32>, y la anotación de tipo no es necesaria. A continuación, veremos cómo modificar un vector.

Actualizar un vector
Para crear un vector y luego agregarle elementos, podemos usar el pushmétodo, como se muestra en el Listado 8-3.

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
Listado 8-3 : Uso del pushmétodo para agregar valores a un vector
Al igual que con cualquier variable, si queremos poder cambiar su valor, debemos hacerla mutable usando la mutpalabra clave, como se explica en el Capítulo 3. Los números que colocamos dentro son todos de tipo i32, y Rust lo infiere de los datos, por lo que no necesitamos la Vec<i32>anotación.

Elementos de lectura de vectores
Hay dos maneras de referenciar un valor almacenado en un vector: mediante indexación o usando el getmétodo. En los siguientes ejemplos, hemos anotado los tipos de valores que devuelven estas funciones para mayor claridad.

El listado 8-4 muestra ambos métodos para acceder a un valor en un vector, con la sintaxis de indexación y el getmétodo.

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
Listado 8-4 : Uso de la sintaxis de indexación y uso del getmétodo para acceder a un elemento en un vector
Tenga en cuenta algunos detalles. Usamos el valor del índice de 2para obtener el tercer elemento, ya que los vectores se indexan por número, comenzando desde cero. Usar &y [] nos da una referencia al elemento en el valor del índice. Al usar el get método con el índice como argumento, obtenemos un Option<&T>que podemos usar con match.

Rust ofrece estas dos maneras de referenciar un elemento para que puedas elegir cómo se comporta el programa al intentar usar un valor de índice fuera del rango de elementos existentes. A modo de ejemplo, veamos qué sucede cuando tenemos un vector de cinco elementos e intentamos acceder a un elemento en el índice 100 con cada técnica, como se muestra en el Listado 8-5.

¡Este código entra en pánico!
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
Listado 8-5 : Intento de acceder al elemento en el índice 100 en un vector que contiene cinco elementos
Al ejecutar este código, el primer []método provocará un pánico en el programa porque hace referencia a un elemento inexistente. Este método es ideal cuando se desea que el programa se bloquee si se intenta acceder a un elemento más allá del final del vector.

Cuando getse pasa al método un índice fuera del vector, este retorna Nonesin entrar en pánico. Este método se usaría si, en circunstancias normales, se pudiera acceder a un elemento fuera del rango del vector. El código tendría entonces lógica para gestionar la presencia de Some(&element)o None, como se explicó en el Capítulo 6. Por ejemplo, el índice podría provenir de una persona que introduce un número. Si accidentalmente introduce un número demasiado grande y el programa obtiene un Nonevalor, se podría indicar al usuario cuántos elementos hay en el vector actual y darle otra oportunidad para introducir un valor válido. Esto sería más intuitivo que bloquear el programa por un error tipográfico.

Cuando el programa tiene una referencia válida, el verificador de préstamos aplica las reglas de propiedad y préstamo (descritas en el Capítulo 4) para garantizar que esta referencia y cualquier otra referencia al contenido del vector sigan siendo válidas. Recuerde la regla que establece que no se pueden tener referencias mutables e inmutables en el mismo ámbito. Esta regla se aplica en el Listado 8-6, donde mantenemos una referencia inmutable al primer elemento de un vector e intentamos añadir un elemento al final. Este programa no funcionará si también intentamos hacer referencia a ese elemento más adelante en la función.

¡Este código no se compila!
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
Listado 8-6 : Intento de agregar un elemento a un vector mientras se mantiene una referencia a un elemento
Compilar este código generará este error:

$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {first}");
  |                                     ------- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` (bin "collections") due to 1 previous error
El código del Listado 8-6 podría parecer que debería funcionar: ¿Por qué una referencia al primer elemento debería preocuparse por los cambios al final del vector? Este error se debe al funcionamiento de los vectores: dado que los vectores colocan los valores uno junto al otro en memoria, añadir un nuevo elemento al final del vector podría requerir asignar nueva memoria y copiar los elementos antiguos al nuevo espacio si no hay suficiente espacio para colocar todos los elementos uno junto al otro donde se almacena actualmente el vector. En ese caso, la referencia al primer elemento apuntaría a memoria desasignada. Las reglas de préstamo evitan que los programas se encuentren en esa situación.

Nota: Para obtener más información sobre los detalles de implementación del Vec<T>tipo, consulte “El Rustonomicon” .

Iterando sobre los valores de un vector
Para acceder a cada elemento de un vector, iteraremos por todos los elementos en lugar de usar índices para acceder a cada uno. El listado 8-7 muestra cómo usar un forbucle para obtener referencias inmutables a cada elemento de un vector de i32valores e imprimirlas.

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
Listado 8-7 : Impresión de cada elemento en un vector iterando sobre los elementos usando un forbucle
También podemos iterar sobre las referencias mutables a cada elemento de un vector mutable para realizar cambios en todos los elementos. El forbucle del Listado 8-8 añadirá valores 50a cada elemento.

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
Listado 8-8 : Iteración sobre referencias mutables a elementos en un vector
Para cambiar el valor al que se refiere la referencia mutable, debemos usar el *operador de desreferencia para obtener el valor iantes de poder usar el += operador. Hablaremos más sobre el operador de desreferencia en la sección "Siguiendo la referencia al valor" del Capítulo 15.

Iterar sobre un vector, ya sea de forma inmutable o mutable, es seguro gracias a las reglas del verificador de préstamos. Si intentáramos insertar o eliminar elementos en los for cuerpos de los bucles de los Listados 8-7 y 8-8, obtendríamos un error de compilación similar al del código del Listado 8-6. La referencia al vector que forcontiene el bucle impide la modificación simultánea de todo el vector.

Uso de una enumeración para almacenar múltiples tipos
Los vectores solo pueden almacenar valores del mismo tipo. Esto puede ser un inconveniente; sin duda, existen casos prácticos en los que es necesario almacenar una lista de elementos de diferentes tipos. Afortunadamente, las variantes de una enumeración se definen bajo el mismo tipo de enumeración, por lo que cuando necesitamos que un tipo represente elementos de diferentes tipos, podemos definir y usar una enumeración.

Por ejemplo, supongamos que queremos obtener valores de una fila en una hoja de cálculo cuyas columnas contienen enteros, números de punto flotante y cadenas. Podemos definir una enumeración cuyas variantes contendrán los diferentes tipos de valor, y todas las variantes de la enumeración se considerarán del mismo tipo: el de la enumeración. Luego, podemos crear un vector que contenga esa enumeración y, por lo tanto, contenga diferentes tipos. Esto se demuestra en el Listado 8-9.

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
Listado 8-9 : Definición de una enumeración para almacenar valores de diferentes tipos en un vector
Rust necesita saber qué tipos estarán en el vector en tiempo de compilación para saber exactamente cuánta memoria en el montón se necesitará para almacenar cada elemento. También debemos ser explícitos sobre los tipos permitidos en este vector. Si Rust permitiera que un vector contuviera cualquier tipo, existiría la posibilidad de que uno o más de estos tipos causaran errores en las operaciones realizadas sobre los elementos del vector. Usar una enumeración más una matchexpresión significa que Rust garantizará que en tiempo de compilación se gestionen todos los casos posibles, como se explica en el Capítulo 6.

Si desconoce el conjunto completo de tipos que un programa obtendrá en tiempo de ejecución para almacenar en un vector, la técnica de enumeración no funcionará. En su lugar, puede usar un objeto de rasgo, que abordaremos en el Capítulo 18.

Ahora que hemos analizado algunas de las formas más comunes de usar vectores, asegúrese de revisar la documentación de la API para conocer todos los métodos útiles definidos en Vec<T>la biblioteca estándar. Por ejemplo, además de push, un popmétodo elimina y devuelve el último elemento.

Al eliminar un vector se eliminan sus elementos
Como cualquier otro struct, un vector se libera cuando queda fuera del alcance, como se indica en el Listado 8-10.

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
Listado 8-10 : Muestra dónde se colocan el vector y sus elementos
Al eliminar un vector, también se elimina todo su contenido, lo que significa que se borrarán los enteros que contiene. El verificador de préstamos garantiza que las referencias al contenido de un vector solo se utilicen mientras el vector sea válido.

¡Pasemos al siguiente tipo de colección: String!
 */
pub fn vector() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Panic porque son valores fuera de rango
    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = &v[4]; // &v[100];
    println!("{:?}", does_not_exist);
    let does_not_exist = v.get(4); // v.get(100);
    println!("{:?}", does_not_exist);

    let /* mut */ v = vec![1, 2, 3, 4, 5]; // quitar mut
    let first = &v[0];
    // v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable [E0502] mutable borrow occurs here
    println!("{:?}", first);

    // Iterar
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // mut iterando vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        match i {
            SpreadsheetCell::Int(x) => println!("Got an int: {}", x),
            SpreadsheetCell::Text(ref s) => println!("Got a string: {}", s),
            SpreadsheetCell::Float(f) => println!("Got a float: {}", f),
        }

    }

    {
        let v = vec![1, 2, 3];
        println!("{:?}", v);
        // do stuff with v
    } // <- v goes out of scope and is freed here
}