/*
Almacenamiento de texto codificado en UTF-8 con cadenas
Hablamos de cadenas en el Capítulo 4, pero las analizaremos con más profundidad ahora. Los nuevos Rustaceans suelen quedarse atascados en cadenas por una combinación de tres razones: la propensión de Rust a exponer posibles errores, que las cadenas sean una estructura de datos más compleja de lo que muchos programadores creen, y UTF-8. Estos factores se combinan de una manera que puede parecer difícil si vienes de otros lenguajes de programación.

Analizamos las cadenas en el contexto de las colecciones, ya que se implementan como una colección de bytes, además de algunos métodos que proporcionan funcionalidad útil cuando estos bytes se interpretan como texto. En esta sección, abordaremos las operaciones que Stringrealiza cada tipo de colección, como la creación, la actualización y la lectura. También analizaremos sus String diferencias con respecto a otras colecciones, concretamente, cómo la indexación en un archivo Stringse complica debido a las diferencias en la interpretación Stringde los datos entre las personas y las computadoras.


Definición de cadenas
Primero definiremos qué entendemos por el término "string" . Rust solo tiene un tipo de cadena en el lenguaje principal: la porción de cadena, strque suele verse en su forma prestada &str. En el capítulo 4, hablamos sobre las porciones de cadena, que son referencias a datos de cadena codificados en UTF-8 almacenados en otro lugar. Los literales de cadena, por ejemplo, se almacenan en el binario del programa y, por lo tanto, son porciones de cadena.

El Stringtipo, proporcionado por la biblioteca estándar de Rust en lugar de estar codificado en el lenguaje principal, es un tipo de cadena ampliable, mutable, propio y codificado en UTF-8. Cuando los habitantes de Rust hablan de "cadenas" en Rust, podrían referirse a los tipos Stringde fragmentos de cadena &str, no solo a uno de ellos. Aunque esta sección trata principalmente sobre String, ambos tipos se usan ampliamente en la biblioteca estándar de Rust, y tanto Stringlos fragmentos de cadena como están codificados en UTF-8.

Creando una nueva cadena
Muchas de las mismas operaciones disponibles con también Vec<T>están disponibles con porque se implementa como un contenedor alrededor de un vector de bytes con garantías, restricciones y capacidades adicionales. Un ejemplo de una función que funciona de la misma manera con y es la función para crear una instancia, que se muestra en el Listado 8-11.StringStringVec<T>Stringnew

    let mut s = String::new();
Listado 8-11 : Creación de un nuevo espacio vacíoString
Esta línea crea una nueva cadena vacía llamada s, en la que podemos cargar datos. A menudo, tendremos datos iniciales con los que queremos iniciar la cadena. Para ello, usamos el to_stringmétodo , disponible en cualquier tipo que implemente el Displayatributo, como lo hacen los literales de cadena. El listado 8-12 muestra dos ejemplos.

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();
Listado 8-12 : Uso del to_stringmétodo para crear un Stringa partir de un literal de cadena
Este código crea una cadena que contiene initial contents.

También podemos usar la función String::frompara crear un Stringa partir de un literal de cadena. El código del Listado 8-13 es equivalente al del Listado 8-12, que usa to_string.

    let s = String::from("initial contents");
Listado 8-13 : Uso de la String::fromfunción para crear Stringa partir de un literal de cadena
Dado que las cadenas se usan para tantas cosas, podemos usar diversas API genéricas para cadenas, lo que nos ofrece una gran variedad de opciones. Algunas pueden parecer redundantes, ¡pero todas tienen su utilidad! En este caso, String::fromy to_stringhacen lo mismo, así que la elección es cuestión de estilo y legibilidad.

Recuerde que las cadenas están codificadas en UTF-8, por lo que podemos incluir en ellas cualquier dato codificado correctamente, como se muestra en el Listado 8-14.

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
Listado 8-14 : Almacenamiento de saludos en diferentes idiomas en cadenas
Todos estos son Stringvalores válidos.

Actualizar una cadena
Un objeto A Stringpuede aumentar de tamaño y su contenido puede cambiar, al igual que el de un objeto A Vec<T>, si se le introducen más datos. Además, se puede usar fácilmente el +operador o la format!macro para concatenar Stringvalores.


Añadiendo con push_stropush
Podemos hacer crecer un objeto Stringusando el push_strmétodo para agregar una porción de cadena, como se muestra en el Listado 8-15.

    let mut s = String::from("foo");
    s.push_str("bar");
Listado 8-15 : Anexar una porción de cadena a un Stringusando el push_strmétodo
Después de estas dos líneas, scontendrá foobar. El push_strmétodo toma una porción de cadena porque no queremos necesariamente tomar posesión del parámetro. Por ejemplo, en el código del Listado 8-16, queremos poder usar s2después de añadir su contenido a s1.

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
Listado 8-16 : Uso de una porción de cadena después de agregar su contenido a unaString
Si el push_strmétodo tomara posesión de s2, no podríamos imprimir su valor en la última línea. Sin embargo, este código funciona como esperábamos.

El pushmétodo toma un solo carácter como parámetro y lo agrega a String. El Listado 8-17 agrega la letra l a a Stringusando el push método.

    let mut s = String::from("lo");
    s.push('l');
Listado 8-17 : Agregar un carácter a un Stringvalor usandopush
Como resultado, scontendrá lol.


Concatenando con +oformat!
A menudo, querrá combinar dos cadenas existentes. Una forma de hacerlo es usar el +operador, como se muestra en el Listado 8-18.

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
Listado 8-18 : Uso del +operador para combinar dos Stringvalores en un nuevo Stringvalor
La cadena s3contendrá Hello, world!. El motivo s1deja de ser válido tras la adición, y la razón por la que usamos una referencia a s2, se relaciona con la firma del método que se llama al usar el +operador. El +operador usa el addmétodo, cuya firma se ve así:

fn add(self, s: &str) -> String {
En la biblioteca estándar, verás addque se definen mediante genéricos y tipos asociados. Aquí, hemos sustituido tipos concretos, que es lo que ocurre al llamar a este método con Stringvalores. Hablaremos de los genéricos en el Capítulo 10. Esta firma nos da las claves necesarias para comprender las partes complejas del +operador.

Primero, s2tiene un &, lo que significa que estamos agregando una referencia de la segunda cadena a la primera. Esto se debe al sparámetro de la add función: Solo podemos agregar una porción de cadena a un String; no podemos sumar dos Stringvalores. Pero espera: el tipo de &s2es &String, no &str, como se especifica en el segundo parámetro de add. Entonces, ¿por qué se compila el Listado 8-18?

La razón por la que podemos usar [nombre del &s2método] en la llamada a [nombre del método] addes que el compilador puede convertir el &Stringargumento en un [ nombre &strdel método]. Al llamar al addmétodo, Rust utiliza una conversión de desreferencia, que en este caso se convierte &s2en &s2[..][nombre del método]. Analizaremos la conversión de desreferencia con más detalle en el Capítulo 15. Dado que [nombre del método add] no toma posesión del sparámetro, s2seguirá siendo válido Stringdespués de esta operación.

En segundo lugar, podemos ver en la firma que addtoma posesión de self porque selfno tiene un &. Esto significa que, s1en el Listado 8-18, se moverá a la addllamada y dejará de ser válido después de eso. Por lo tanto, aunque let s3 = s1 + &s2;parezca que copiará ambas cadenas y creará una nueva, esta instrucción en realidad toma posesión de s1, añade una copia del contenido de s2y luego devuelve la posesión del resultado. En otras palabras, parece que está haciendo muchas copias, pero no es así; la implementación es más eficiente que copiar.

Si necesitamos concatenar varias cadenas, el comportamiento del +operador se vuelve complicado:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
En este punto, sserá tic-tac-toe. Con tantos caracteres +y " , es difícil ver qué sucede. Para combinar cadenas de forma más compleja, podemos usar la format!macro:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
Este código también se establece sen tic-tac-toe. La format!macro funciona como println!, pero en lugar de mostrar la salida en pantalla, devuelve un Stringcon el contenido. La versión del código que usa format!es mucho más fácil de leer, y el código generado por la format!macro usa referencias para que esta llamada no tome posesión de ninguno de sus parámetros.

Indexación en cadenas
En muchos otros lenguajes de programación, acceder a caracteres individuales de una cadena mediante referencias por índice es una operación válida y común. Sin embargo, si intenta acceder a partes de una Stringsintaxis de indexación en Rust, obtendrá un error. Considere el código no válido del Listado 8-19.

¡Este código no se compila!
    let s1 = String::from("hi");
    let h = s1[0];
Listado 8-19 : Intento de utilizar la sintaxis de indexación con unString
Este código dará como resultado el siguiente error:

$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:16
  |
3 |     let h = s1[0];
  |                ^ string indices are ranges of `usize`
  |
  = note: you can use `.chars().nth()` or `.bytes().nth()`
          for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
          but trait `SliceIndex<[_]>` is implemented for `usize`
  = help: for that trait implementation, expected `[_]`, found `str`
  = note: required for `String` to implement `Index<{integer}>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `collections` (bin "collections") due to 1 previous error
El error lo dice todo: las cadenas de Rust no admiten la indexación. ¿Pero por qué no? Para responder a esta pregunta, necesitamos analizar cómo Rust almacena las cadenas en memoria.

Representación interna
A Stringes un contenedor sobre a Vec<u8>. Veamos algunos ejemplos de cadenas UTF-8 correctamente codificadas del Listado 8-14. Primero, este:

    let hello = String::from("Hola");
En este caso, lenserá 4, lo que significa que el vector que almacena la cadena "Hola"tiene 4 bytes. Cada una de estas letras ocupa 1 byte cuando se codifica en UTF-8. Sin embargo, la siguiente línea puede sorprenderle (tenga en cuenta que esta cadena comienza con la letra cirílica mayúscula Ze , no con el número 3):

    let hello = String::from("Здравствуйте");
Si te preguntaran la longitud de la cadena, podrías decir 12. De hecho, la respuesta de Rust es 24: esa es la cantidad de bytes que se necesitan para codificar "Здравствуйте" en UTF-8, ya que cada valor escalar Unicode de esa cadena ocupa 2 bytes de almacenamiento. Por lo tanto, un índice en los bytes de la cadena no siempre se correlacionará con un valor escalar Unicode válido. Para demostrarlo, considera este código de Rust inválido:

¡Este código no se compila!
let hello = "Здравствуйте";
let answer = &hello[0];
Ya sabes que answerno será З, la primera letra. Al codificarse en UTF-8, el primer byte de Зes 208y el segundo es 151, por lo que parecería que answerdebería ser 208, pero 208no es un carácter válido por sí mismo. 208Es probable que un usuario no desee obtener la primera letra de esta cadena; sin embargo, ese es el único dato que Rust tiene en el índice de byte 0. Los usuarios generalmente no quieren que se devuelva el valor del byte, incluso si la cadena contiene solo letras latinas: si &"hi"[0]fuera un código válido que devolviera el valor del byte, devolvería 104, no h.

La respuesta, entonces, es que para evitar devolver un valor inesperado y causar errores que podrían no descubrirse inmediatamente, Rust no compila este código en absoluto y evita malentendidos al principio del proceso de desarrollo.


Bytes, valores escalares y grupos de grafemas
Otro punto sobre UTF-8 es que en realidad hay tres formas relevantes de ver las cadenas desde la perspectiva de Rust: como bytes, valores escalares y grupos de grafemas (lo más cercano a lo que llamaríamos letras ).

Si observamos la palabra hindi “नमस्ते” escrita en escritura devanagari, se almacena como un vector de u8valores que se ve así:

[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
Esos son 18 bytes, y así es como las computadoras almacenan estos datos. Si los consideramos como valores escalares Unicode, que son el chartipo de Rust, esos bytes se ven así:

['न', 'म', 'स', '्', 'त', 'े']
Aquí hay seis charvalores, pero el cuarto y el sexto no son letras: son diacríticos que no tienen sentido por sí solos. Finalmente, si los consideramos como grupos de grafemas, obtendríamos lo que una persona llamaría las cuatro letras que componen la palabra hindi:

["न", "म", "स्", "ते"]
Rust proporciona diferentes formas de interpretar los datos de cadenas sin procesar que almacenan las computadoras para que cada programa pueda elegir la interpretación que necesita, sin importar en qué lenguaje humano estén los datos.

Una última razón por la que Rust no permite indexar en a Stringpara obtener un carácter es que se espera que las operaciones de indexación siempre tomen un tiempo constante (O(1)). Sin embargo, no es posible garantizar ese rendimiento con a String, ya que Rust tendría que recorrer el contenido desde el principio hasta el índice para determinar cuántos caracteres válidos hay.

Cortar cuerdas
Indexar una cadena suele ser una mala idea, ya que no está claro cuál debería ser el tipo de retorno de la operación: un valor de byte, un carácter, un grupo de grafemas o un fragmento de cadena. Por lo tanto, si realmente necesita usar índices para crear fragmentos de cadena, Rust le pide que sea más específico.

En lugar de indexar []con un solo número, puede utilizar []un rango para crear una porción de cadena que contenga bytes específicos:

let hello = "Здравствуйте";

let s = &hello[0..4];
Aquí, shabrá un &strque contiene los primeros 4 bytes de la cadena. Anteriormente, mencionamos que cada uno de estos caracteres ocupaba 2 bytes, lo que significa que sserá Зд.

Si intentáramos cortar solo una parte de los bytes de un carácter con algo como &hello[0..1], Rust entraría en pánico en tiempo de ejecución de la misma manera que si se accediera a un índice no válido en un vector:

$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`

thread 'main' panicked at src/main.rs:4:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Debe tener cuidado al crear segmentos de cadenas con rangos, ya que hacerlo puede provocar que su programa se bloquee.


Iteración sobre cadenas
La mejor manera de operar con fragmentos de cadenas es especificar explícitamente si se desean caracteres o bytes. Para valores escalares Unicode individuales, utilice el charsmétodo. Llamar charsa "Зд" separa y devuelve dos valores de tipo char, y se puede iterar sobre el resultado para acceder a cada elemento:

for c in "Зд".chars() {
    println!("{c}");
}
Este código imprimirá lo siguiente:

З
д
Como alternativa, el bytesmétodo devuelve cada byte sin procesar, lo que podría ser apropiado para su dominio:

for b in "Зд".bytes() {
    println!("{b}");
}
Este código imprimirá los 4 bytes que componen esta cadena:

208
151
208
180
Pero asegúrese de recordar que los valores escalares Unicode válidos pueden estar compuestos de más de 1 byte.

Obtener clústeres de grafemas a partir de cadenas, como en el caso del script devanagari, es complejo, por lo que la biblioteca estándar no ofrece esta funcionalidad. Si necesita esta funcionalidad, puede encontrar crates en crates.io .


Manejo de las complejidades de las cadenas
En resumen, las cadenas son complejas. Cada lenguaje de programación toma decisiones diferentes sobre cómo presentar esta complejidad al programador. Rust ha optado por que el manejo correcto de Stringdatos sea el comportamiento predeterminado de todos sus programas, lo que implica que los programadores deben prestar más atención al manejo de datos UTF-8 desde el principio. Esta compensación expone una mayor complejidad de las cadenas que la que se aprecia en otros lenguajes de programación, pero evita tener que gestionar errores relacionados con caracteres no ASCII más adelante en el ciclo de desarrollo.

La buena noticia es que la biblioteca estándar ofrece una amplia funcionalidad basada en los tipos Stringy &strpara gestionar correctamente estas situaciones complejas. Asegúrese de consultar la documentación para encontrar métodos útiles, como containsbuscar en una cadena y replacesustituir partes de una cadena por otra.

Pasemos a algo un poco menos complejo: ¡mapas hash!
 */
pub fn cadena() {
    let mut s = String::new();
    s.push_str("hello");
    println!("s = {}", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("s = {}", s);

    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("s = {}", s);

    let s = String::from("initial contents");
    println!("s = {}", s);

    let hello = String::from("السلام عليكم");
    println!("hello = {}", hello);
    let hello = String::from("Dobrý den");
    println!("hello = {}", hello);
    let hello = String::from("Hello");
    println!("hello = {}", hello);
    let hello = String::from("שלום");
    println!("hello = {}", hello);
    let hello = String::from("नमस्ते");
    println!("hello = {}", hello);
    let hello = String::from("こんにちは");
    println!("hello = {}", hello);
    let hello = String::from("안녕하세요");
    println!("hello = {}", hello);
    let hello = String::from("你好");
    println!("hello = {}", hello);
    let hello = String::from("Olá");
    println!("hello = {}", hello);
    let hello = String::from("Здравствуйте");
    println!("hello = {}", hello);
    let hello = String::from("Hola");
    println!("hello = {}", hello);

    // Añadir cadena con push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s = {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 = {}, s2 = {}", s1, s2);

    // agregar carácter
    let mut s = String::from("lo");
    s.push('l');
    println!("s = {}", s);

    // Concatenar
    // esta instrucción en realidad toma posesión de s1, añade una copia del contenido de s2y luego devuelve la posesión del resultado
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 =String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s = {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);

    // Indexación en String no se puede
    // let s1 = String::from("hi");
    // let h = s1[0]; // the type `str` cannot be indexed by `{integer}` [E0277] string indices are ranges of `usize`
    // println!("h = {}", h);

    let hello = String::from("Hola");
    println!("{}", hello);

    let hello = String::from("Здравствуйте");
    println!("{}", hello);

    let hello = "Здравствуйте";
    // let answer = &hello[0]; // The type `str` cannot be indexed by `{integer}` [E0277]
    println!("{}", hello);

    // Indexación con []
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // Iteración sobre cadena con chart
    for c in "Зд".chars() {
        println!("{}", c);
    }

    // Iteración con bytes
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
