/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */
 
fn main(){

    let dividendo: u16 = 216;
    let divisor: u16 = 2;

    let cociente = calcular_cociente(dividendo, divisor);
}

fn calcular_cociente(dividendo: u16, divisor: u16) -> u16 {
    dividendo / divisor
}