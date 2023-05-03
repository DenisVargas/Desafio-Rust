use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let mut trows: u32 = 0;
    let mut numbers: [u8;6] = [0,0,0,0,0,0];
    let mut rng = thread_rng();

    loop {
        println!("Haz tirado el dado {trows} veces, quieres tirar un dado? y/n");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read users input.");

        if input.trim().eq_ignore_ascii_case("y") {    //Investigar mas sobre trabajo con strings, comparacion.
            let result: u8 = rng.gen_range(1u8..=6u8);
            println!("Ha salido el numero {result}!\n");
            trows += 1;
            let index = (result - 1u8) as usize; //Los indices usan usize, tenemos que usar conversiones.
            numbers[index] += 1u8;
        }
        //Si el dato es n, salgo del loop.
        if input.trim().eq_ignore_ascii_case("n") {
            print_resume(numbers, trows);
            println!("Hasta luego!");
            break;
        }
    }

    // let result = loop {
    //     5 //Ultima linea es considerada una expresion de retorno a menos que utilizemos ; (statement) y el retorno es unit ()
    // }
    
    // while bool {
    //     //no puede retornar valores directamente, tenemos que usar una variable externa.
    // }

    println!("Fin del programa");
}

fn print_resume(numbers: [u8;6], trows: u32) {
    //println!("Hasta ahora haz tirado el dado {trows} veces y han salido:\nnumero 1 {} veces\nnumero 2 {} veces\nnumero 3 {} veces\nnumero 4 {} veces\nnumero 5 {} veces,\nnumero 6 {} veces", numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5]);
    println!("Hasta ahora haz tirado el dado {trows} veces.");
    let mut element = 0;
    for times in numbers{
        element += 1;
        println!("El numero {element} ha salido {times} veces.");
    }
}
