fn main() {
    let mut counter : i32 = 0;

    let result = loop {
        counter += 1;
        println!("Hello, world!");
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut counter: i32 = 0;
    while counter < 5 {
        println!("Remains {counter}!");
        counter += 1;
    };
    println!("End of program, with result as {counter}!");

    let a = [10,20,30,40,50];

    for element in a {
        println!("The value of the element is {element}");
    }
}
