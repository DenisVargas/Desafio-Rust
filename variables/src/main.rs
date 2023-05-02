//Las variables son inmutables por defecto
//Para hacerlos mutables usa mut
//Si quieres valores estaticos utiliza constant, los nombres deben estar escritos en mayuscula.

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    another_function();
    let six = plus_one(5);
    println!("The result is: {six}");
}

fn another_function() {
    println!("Another function.");
}

fn plus_one(x:i32) -> i32 {
    //x + 1; //Produce un error, ya que es una statement.
    x + 1  //Funciona porque se evalua como expression.
}
