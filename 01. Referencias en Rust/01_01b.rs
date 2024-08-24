/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */
 
fn main(){

    let numero: i32 = 7;
    let numero_ref: &i32 = &numero;
    let direccion = &numero as *const i32;

    println!("Valor de numero: {}", numero);
    println!("Valor de numero: {}", numero_ref);
    println!("Direccion de memoria de numero: {:?}", direccion);
}