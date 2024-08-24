/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main() {

    let palabra = "Hola";
    let primera_letra = palabra.chars().next().unwrap();
    let _primera_letra_ref = &primera_letra;

    println!("La primera letra es: {}", primera_letra);
}