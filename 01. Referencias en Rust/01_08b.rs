/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main() {

    let palabra = "Hola";
    // let primera_letra = palabra.chars().next().unwrap();
    // let _primera_letra_ref = &primera_letra;

    let primera_letra = devuelve_primera_letra(palabra);
    println!("La primera letra es: {}", primera_letra);
}

fn devuelve_primera_letra<'a>(palabra: &'a str) -> &'a str {
    palabra.get(0..palabra.chars().next().unwrap().len_utf8()).unwrap()
}