/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main() {

    let numeros_lost = [4, 8, 15, 16, 23, 42];

    let primera_mitad = &numeros_lost[..3];
    let segunda_mitad = &numeros_lost[3..];

    println!("Primera mitad: {:?}", primera_mitad);
    println!("Segunda mitad: {:?}", segunda_mitad);
}