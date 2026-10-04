fn conditional(n: u8) -> u8 {
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

fn main() {
    // Macro higienica, con ! especifico que es una macro
    println!("Hello, world!");

    // Enteros
    println!("Enteros");
    let a: u32;
    let b: u8;

    a = 238999;
    b = 255;

    println!("a: {a}");

    // Para imprimir funciones se hace asi
    println!("conditional(): {}", conditional(b));

    // Esto es como un ternario
    // Debe tener ; para separla de la siguiente instruccion
    let size = if a < 20 { "pequeño" } else { "grande" };
    println!("size: {size}");
    
    // bucle
    let paco: u8 = 0;
    bucle(paco);

    // Macro que indica con un error que no esta implementado
    todo!("En proceso");
}

fn bucle(mut x: u8) {
    while x < 2 {
        x += 1;
    }

    for x in 1..5 {
        println!("x: {x}");
    }
    
    // Aqui vale 2
    println!("{}", x);

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }

    // Bucle que siempre entra una vez
    loop {
        x += 1;
        if x == 4 {
            break
        }
    }
    
    println!("{}", x);
}
