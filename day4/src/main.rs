use std::io;

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();

    println!("Ingresa un primer valor");
    io::stdin()
        .read_line(& mut first_number)
        .expect("Failed to read users input");
    println!("Ingresa un segundo valor");
    io::stdin()
        .read_line(& mut second_number)
        .expect("Failed to read users input");
    //Utilizamos trim() ya que la consola retorna "valor\n" para eliminar el salto de linea al final del input.
    let mut number_a : f32 = first_number.trim().parse().unwrap();
    let mut number_b : f32 = second_number.trim().parse().unwrap();

    swap(&mut number_a, &mut number_b);

    println!("El primer valor es {number_a} y el segundo valor es {number_b}");
    println!("El primer valor es {number_a} y el segundo valor es {number_b}");
}

//............... Value of type f32
//fn swap(value_a: f32, ...
//............... Inmutable Reference of type f32
//fn swap(value_a: &f32, ...
//............... Mutable Reference of type f32
//fn swap(value_a: &mut f32, ...

fn swap(value_a: &mut f32, value_b: &mut f32){
    let aux = *value_b;
    *value_b = *value_a;
    *value_a = aux;
}
