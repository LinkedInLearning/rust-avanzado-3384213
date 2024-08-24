/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

struct Paciente(String, u16, f64);

fn main(){

    let paciente = Paciente("Anónimo".to_string(), 28, 1.75);
    println!("{}", paciente.2);
}