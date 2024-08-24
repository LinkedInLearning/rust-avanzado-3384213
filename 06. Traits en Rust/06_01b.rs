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

impl From<(String, u32, f64)> for Paciente {

    fn from(datos_paciente: (String, u32, f64)) -> Self {
        Paciente {
            nombre: datos_paciente.0,
            edad: datos_paciente.1,
            altura: datos_paciente.2,
        }
    }
}

fn main(){
    let datos_paciente = ("Anónimo".to_string(), 64, 1.89);
    let paciente = Paciente::from(datos_paciente); // into()
}