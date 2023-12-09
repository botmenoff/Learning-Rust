use std::io;

fn main() {
    let parameter: i32;
    let mut input = String::new();
    println!("Ingrese un numero para mostrar su tabla de multiplicar: ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    parameter = input.trim().parse().expect("Error al convertir el valor");
    tabla_multiplicar(parameter); 

}

fn tabla_multiplicar(x: i32) {
    for i in 1..11 {
        println!("{} x {} = {}", x, i, x * i);
    }
}