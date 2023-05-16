//Dia 6
//Objetivo: Crear un programa que represente una estructura de datos a mi eleccion utilizando structs y/o enums.

enum Node {
    Root,
    Rect,
}

struct Rect {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    children: Vec<Rect>,
}

impl Rect {
    //Clone necesita que entendamos que son los traits.
    // fn clone(&self) -> Rect{
    //     let newRect = Rect::new(0,0,0,0, Vec::<Rect>::new());
    //     return newRect
    // }
    // fn add_child(&self, child: Rect) -> Rect{ //Queremos tomar ownership del dato que se recibe.
    //     self.children.push(child.clone());
    //     return child
    // }
    fn get_children(&self) -> &Vec<Rect>{ //Retornamos una referencia inmutable
        return &self.children;
    }
    fn get_first_child(&self) -> &Rect { //Queremos retornar una referencia al primer elemento.
        //Acceder a un valor por medio de la indexacion retora una referencia (&)
        return &self.children[0];
    }
}

fn main() {
    println!("Hello, world!");
}
