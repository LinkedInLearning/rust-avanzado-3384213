/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

struct Paciente{
    nombre: String,
    edad: u32,
    altura: f64,
}

fn main(){

    let datos_paciente = ("Anónimo".to_string(), 64, 1.89);
    let paciente = Paciente::from(datos_paciente);
}