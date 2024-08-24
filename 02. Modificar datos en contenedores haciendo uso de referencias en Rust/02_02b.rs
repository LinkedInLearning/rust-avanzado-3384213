/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main(){

    let mut titulo = String::from("Cursos sobre C++ en LinkedIn Learning");

    cambiar_lenguaje(&mut titulo);
    println!("{}", titulo);
}

fn cambiar_lenguaje(mensaje: &mut String){
    if let Some(posicion_inicial) = mensaje.find("C++"){
        let posicion_final = posicion_inicial + "C++".len();
        mensaje.replace_range(posicion_inicial..posicion_final, "Rust");
    }
}