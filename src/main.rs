use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};

use bomberman::punto::Punto;
mod bomberman;

/// Toma los argumentos de la consola y ejecuta el juego Bomberman-R, luego imprime el resultado o un error en el archivo destino.
///
/// # Ejemplos
///
/// ```
/// $ cargo run main.rs entrada.txt salidas 0 0
///
/// entrada:
/// 1  B2 R R _ F1 _ _
/// 2  _ W R W _ W _
/// 3  B5 _ _ _ B2 _ _
/// 4  _ W _ W _ W _
/// 5  _ _ _ _ _ _ _
/// 6  _ W _ W _ W _
/// 7  _ _ _ _ _ _ _
///
///
/// salida:
/// 1  _ R R _ _ _ _
/// 2  _ W R W _ W _
/// 3  _ _ _ _ _ _ _
/// 4  _ W _ W _ W _      o     1  Error: Descripcion del error
/// 5  _ _ _ _ _ _ _
/// 6  _ W _ W _ W _
/// 7  _ _ _ _ _ _ _
/// ```
///
/// # Argumentos
///
/// * 1: Archivo de input que contiene la tabla inicial.
/// * 2: Ruta hacia el output que tendra el mismo nombre que el archivo de input.
/// * 3: Coordenada X de alguna Bomba.
/// * 4: Coordenada Y de alguna Bomba.
///
/// # Devuelve
///
/// Escribe en el archivo de output el estado final del juego o una descripcion del Error que lo impidio.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        return print!("Error: faltan argumentos");
    }
    let ruta_entrada: String = args[1].clone();
    let ruta_salida: String = args[2].clone() + "/" + &extraer_archivo_destino(&ruta_entrada);

    let salida: File = match File::create(ruta_salida.clone()) {
        Ok(f) => f,
        _ => return print!("Error: ruta de salida invalida"),
    };

    let tablero: Vec<Vec<String>> = match tablero_desde_archivo(ruta_entrada) {
        Err(e) => return devolver_error(e, salida),
        Ok(tab) => tab,
    };
    let tamanio = tablero.len();
    let mut bomberman = bomberman::Bomberman {
        tablero,
        pila_bombas: Vec::new(),
    };

    let punto_bomba: Punto = match extraer_coord(args, tamanio, 3, 4) {
        Err(e) => return devolver_error(e, salida),
        Ok(pt) => pt,
    };

    if let Err(e) = bomberman::Bomberman::comenzar(&mut bomberman, punto_bomba.x, punto_bomba.y) {
        return devolver_error(e, salida);
    }
    escribir_tablero_final_en_archivo(bomberman.tablero, salida);
}

/// Toma la ruta a un archivo de texto en el que haya una matriz de Strings NxN separada por espacios y saltos de linea y devuelve un Result con la Matriz o un Err(String) con la descripcion del error.
///
/// # Ejemplos
///
/// ```
/// let  tablero: Vec<Vec<String>>;
/// match tablero_desde_archivo(ruta_entrada) {
///     Err(e) => {return devolver_error(e, salida)}
///     Ok(tab) => tablero = tab
/// }
/// ```
///
/// # Argumentos
///
///  * ruta_entrada: La ruta hasta el archivo de texto con la matriz.
///
/// # Devuelve
///
/// Un Result con la matriz de Strings o un Err con la descripcion del.
fn tablero_desde_archivo(ruta_entrada: String) -> Result<Vec<Vec<String>>, String> {
    let archivo_entrada_result = File::open(ruta_entrada);

    let file_input: File = match archivo_entrada_result {
        Ok(file) => file,
        _ => return Err("Error: no se pudo abrir correctamente el archivo".to_string()),
    };

    let reader = io::BufReader::new(file_input);

    // Creo el tablero de Juego
    let mut tablero: Vec<Vec<String>> = Vec::new();
    for linea in reader.lines() {
        let casillas: String = match linea {
            Ok(f) => f,
            _ => return Err("Error: no se pudo leer correctamente el archivo".to_string()),
        };
        let palabras: Vec<String> = casillas.split_whitespace().map(String::from).collect();
        tablero.push(palabras)
    }
    if es_tablero_valido(&mut tablero) {
        return Ok(tablero);
    }
    Err("Error: archivo de entrada invalido".to_string())
}

/// Toma una matriz de Strings y evalua todas las casillas para ver si son validas para un juego de Bomberman.
///
/// # Ejemplos
///
/// ```
/// let tablero = Vec![
///     Vec!["W".to_string(),"B1".to_string()],
///     Vec!["S3".to_string(),"R".to_string()]
///     ];
/// if es_tablero_valido(&mut tablero){
///     return Ok(tablero)
/// }
/// return Err("Error: tablero invalido".to_string())
/// ```
///
/// # Argumentos
///
/// * tablero: La matriz de Strings que se quiere evaluar si es valida.
///
/// # Devuelve
///
/// Un booleano indicando si la matriz es valida (true) o si no es valida (false).
fn es_tablero_valido(tablero: &mut Vec<Vec<String>>) -> bool {
    let validos_no_bomba = [
        "_".to_string(),
        "W".to_string(),
        "R".to_string(),
        "DU".to_string(),
        "DL".to_string(),
        "DR".to_string(),
        "DD".to_string(),
        "F1".to_string(),
        "F2".to_string(),
        "F3".to_string(),
    ];
    for fila in tablero {
        for elem in fila {
            if validos_no_bomba.contains(elem) {
                continue;
            }
            let mut iter = elem.chars();
            let prim = iter.next();
            let sec = iter.next().unwrap_or('X');
            if (prim == Some('S')) | (prim == Some('B')) && sec.is_ascii_digit() && sec != '0' {
                continue;
            }
            return false;
        }
    }
    true
}

/// Toma un String con una descripcion de un error y lo escribe en el archivo.
///
/// # Ejemplos
///
/// ```
/// let salida: File = File::create("ruta/de/salida.txt").unwrap();
/// match result_casual {
///     Ok() => {},
///     Err(e: String) => devolver_error(e,salida)
/// }
/// ```
///
/// # Argumentos
///
/// * error_string: La cadena que se quiere escribir en el archivo para informar el error.
/// * salida: El archivo donde se quiere escribir.
fn devolver_error(error_string: String, mut salida: File) {
    if let Err(_e) = salida.write((error_string).as_bytes()) { // Error en la escritura
    }
}

/// Toma un vector de strings y extrae de las posiciones de los indices i1 e i2, 2 strings para pasar a usize y devolverlos en un Result o devolver un Error con un String con su descripcion.
///
/// # Ejemplos
///
/// ```
/// let punto_bomba: Punto;
/// match extraer_coord(args, tamanio,indice_1,indice_2) {
///     Err(e) => {return devolver_error(e, salida)},
///     Ok(pt) => punto_bomba = pt
/// }
/// ```
///
/// # Argumentos
///
/// * args: Vector de Strings que incluye las coordenadas a transformar.
/// * tamanio: Valor que no pueden superar para que sean validas.
/// * i1: Indice de la coordenada X.
/// * i2: Indice de la coordenada Y.
///
/// # Devuelve
///
/// Un Result exitoso con un vector que contiene las coordenadas en usize o un Error con un String describiendo el mismo.
fn extraer_coord(args: Vec<String>, tamanio: usize, i1: usize, i2: usize) -> Result<Punto, String> {
    let mut punto_bomba: Punto = Punto { x: 0, y: 0 };

    match args[i1].parse() {
        Ok(x) => punto_bomba.x = x,
        _ => return Err("Error: coordenadas invalidas".to_string()),
    }
    match args[i2].parse() {
        Ok(y) => punto_bomba.y = y,
        _ => return Err("Error: coordenadas invalidas".to_string()),
    }
    if punto_bomba.x >= tamanio || punto_bomba.y >= tamanio {
        return Err("Error: coordenadas invalidas".to_string());
    }
    Ok(punto_bomba)
}

/// Toma una matriz de Strings y la escribe en el archivo pedido, separando las filas con saltos de linea y las columnas con espacios.
///
/// # Ejemplos
///
/// ```
/// let tablero: Vec<Vec<String>> = Vec![
///     Vec!['M','A','T','R'],
///     Vec!['_','_','_','I'],
///     Vec!['_','_','_','Z']
/// ];
/// escribir_tablero_final_en_archivo(tablero,archivo_a_escribir)
/// ```
/// y queda en el archivo_a_escribir:
/// ```
/// 1  M A T R
/// 2  _ _ _ I
/// 3  _ _ _ Z
/// ```
/// # Argumentos
///
/// * tablero: Matriz de strings que se quiere escribir en el archivo.
/// * salida: Archivo destino en el que se quiere escribir.
fn escribir_tablero_final_en_archivo(tablero: Vec<Vec<String>>, mut salida: File) {
    for (indice, fila) in tablero.iter().enumerate() {
        if indice == tablero.len() - 1 {
            if let Err(_e) = salida.write((fila.join(" ")).as_bytes()) {
                return; // Error en la escritura
            }
        } else if let Err(_e) = salida.write((fila.join(" ") + "\n").as_bytes()) {
            return; // Error en la escritura
        }
    }
}

/// Toma una ruta que puede contener directorios y extrae el nombre del archivo destino.
///
/// # Ejemplos
///
/// ```
/// let ruta_entrada: String = "entradas/de/ejemplo/primero.txt";
/// let ruta_salida: String = "directorio/de/salidas" + "/" + &extraer_archivo_destino(&ruta_entrada);
///
/// if ruta_salida == "directorio/de/salidas/primero.txt" {
///     let salida = File::create(ruta_salida.clone())
/// }
///
/// ```
///
/// # Argumentos
///
/// * ruta: String de la ruta de directorios hasta el archivo al que se le quiere saber el nombre.
///
/// # Devuelve
///
/// Un String con el nombre del archivo destino de la ruta provista o, en caso de error, la misma ruta provista.
fn extraer_archivo_destino(ruta: &String) -> String {
    let separado = ruta.split('/');
    match separado.last() {
        Some(r) => r.to_string(),
        None => ruta.to_string(),
    }
}
