/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

struct Paciente {
    nombre: String,
    edad: u32,
    altura: f64,
}

fn main(){
    let nombre = String::from("Anónimo");
    let edad = 64;
    let altura = 1.89;

    let paciente: Paciente = Paciente {
        nombre: nombre,
        edad: edad,
        altura: altura,
    };
}