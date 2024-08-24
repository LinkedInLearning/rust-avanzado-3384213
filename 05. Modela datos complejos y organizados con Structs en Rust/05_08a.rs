/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

struct Paciente(String, u16, f64);

fn main(){

    // let tupla: (i32, f64, u8) = (500, 6.4, 1);

    /*
    struct Paciente {
        nombre: String,
        edad: u16,
        altura: f64,
    }
    */
    let paciente = Paciente("Anónimo".to_string(), 28, 1.75);
    println!("{}", paciente.2);
}