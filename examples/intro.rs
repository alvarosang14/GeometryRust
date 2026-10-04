fn conditional(n: u8) -> u8 {
    // Esto es como un ternario
    // Debe tener ; para separla de la siguiente instruccion
    let size = if n < 20 { "pequeño" } else { "grande" };
    println!("size: {size}");

    // if es un tipo de expresion y todas sus ramas deben de ser del mismo tipo
    if n < 100 {
        // Retorno un valor
        return 0;
    } else if n == 101 {
        // Declaro una expresión, siemrpoe sin pounto y coma
        // porque si no esa es la ultima expresion
        1
    } else {
        2
    }
}

fn bucle(mut x: u8) {
    while x < 2 {
        x += 1;
    }

    // si quieres incluir el ultimo 1..=5
    for x in 1..5 {
        println!("x: {x}");
    }
    
    // Aqui vale 2
    println!("{}", x);

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }

    // Bucle que siempre entra una vez.
    // Siempre devuelve un valor no trivial
    loop {
        x += 1;
        if x == 4 {
            break
        }
        if x == 3 {
            continue
        }
        println!("Hello world {}", x);
    }

    // Uso de etiquetas para salir del bucle externo
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    
    println!("{}", x);
}

fn ambito() {
    let x = {
        println!("=== Ambito ===");
        // si pongo ; devuleve el tipo unitario, ()
        2 - 3
    };

    let y = {
        if x < 0 {
            // no meter retunr proque se sale de la funcion entera
            1
        } else {
            2
        }
    };

    println!("Ambito: {x}, {y}");

    let a = 10;
    println!("antes: {a}");
    {
        // Creamos la variable a nivel interno
        let a = "hola";
        println!("ámbito interno: {a}");

        // La sombreamos
        let a = true;
        println!("sombreado en el ámbito interno: {a}");
    }

    println!("después: {a}");

}

fn tuplas_arrays() {
    // a y b son de distinto tipos
    let _a: [u8; 10];
    let b: [u8; 3] = [0; 3];

    println!("Tuplas array {}", b[0]);

    // Como las arrays no tine una forma estandar de imprimir
    // Su salida es por depuración
    println!("Tuplas array {:?}", b);
    // Con # damos formato al texto, en este caso salto de linea
    println!("Tuplas array {b:#?}");

    // Las tuplas me permiten crear un tipo combinado
    // La tupla vacía () es llamado el “tipo de unidad” y significa 
    // la ausencia de un valor de retorno, parecido a void en otros lenguajes.
    let c: (bool, i8) = (true, 5);
    println!("Tupla: {}", c.0);

    for x in b {
        let a = 1;
    }
}

// ==================================================================
// main
// ==================================================================
fn main() {
    // Macro higienica, con ! especifico que es una macro
    println!("Hello, world!");

    // Enteros
    println!("Enteros");
    let _a: u32;
    let a: u16; // esto en cpp no se puede hacer
    let b: u8;

    a = 23899;
    b = 255;

    println!("a: {a}");

    // Para imprimir funciones se hace asi
    println!("conditional(): {}", conditional(b));
    
    // bucle con inferencia de tipos para paco
    let paco = 0;
    bucle(paco);

    // Llamada a ambito
    ambito();

    // Llamada a macros
    macros();

    // Tuplas y arrays
    tuplas_arrays();

    // Macro que indica con un error que no esta implementado
    todo!("En proceso");
}

// rust hace varias pasadas y por eso a diferenci de c/cpp se puede declarar despues de su uso
// hace como una especie de parseo
fn macros() {
    println!("Macros");

    // Para evitar warnings, le indica al compilador que la ignore
    let _x = format!("Macro: x = {val}", val = 10);

    // Para evitar warnings
    let _i = dbg!("Imprime y devuelve");
    
    let y: u8 = 0;
    if y == 1 {
        // Si entra aqui falla entre terribles sufrimientos
        unreachable!("Error");
    }

}