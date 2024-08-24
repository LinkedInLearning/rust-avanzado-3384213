/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main(){

    let cadena = String::from("LinkedIn Learning");

    if let Some(posicion_inicial) = cadena.find("Learning"){
        let learning_slice = &cadena[posicion_inicial..cadena.len()];
        println!("{:?}", learning_slice);
    }
}