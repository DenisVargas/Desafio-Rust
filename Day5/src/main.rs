use std::io;

fn main() {
    //Los vectores son arrays expandibles. Pertenecen a std::vec
    //https://doc.rust-lang.org/rust-by-example/std/vec.html
    //https://doc.rust-lang.org/std/vec/
    //https://doc.rust-lang.org/book/ch08-00-common-collections.html
    let mut numbers : Vec<u8> = Vec::new();

    loop {
        println!("Quieres ingresar un numero? y/n");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to load users input.");
        if input.trim().eq_ignore_ascii_case("y") {
            println!("ingresa un numero");
            let mut value: String = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read the value");

            let parsed_value : u8 = value.trim().parse::<u8>().expect("Error al realizar la conversion");
            numbers.push(parsed_value);
        }
        if input.trim().eq_ignore_ascii_case("n") {
            break;
        }
    }

    numbers.sort();
    for (i, element) in numbers.iter().enumerate() {
        println!("En la posicion {i} tenemos el valor {element}.");
    }

    //Usa {:?} para imprimir colecciones.
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);
}
