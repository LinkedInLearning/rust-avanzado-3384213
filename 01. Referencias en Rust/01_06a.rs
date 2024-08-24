/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */
 
fn main() {

    let dividendo: u16 = 216;
    let divisor: u16 = 2;

    let cociente = calcular_cociente(dividendo, divisor);

    match cociente {
        Some(valor) => println!("El cociente es: {}", valor),
        None => println!("No se puede dividir entre cero."),
    }
}

fn calcular_cociente(dividendo: u16, divisor: u16) -> Option<u16> {
    if divisor == 0 {
        None
    } else {
        Some(dividendo / divisor)
    }
}