/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

enum Semaforo {
    Rojo,
    Amarillo,
    Verde,
}

trait EstadoSemaforo {
    fn accion(&self) -> String;
}

impl EstadoSemaforo for Semaforo {

    fn accion(&self) -> String {

        match self {
            Semaforo::Rojo => String::from("Detente"),
            Semaforo::Amarillo => String::from("Disminuye la velocidad"),
            Semaforo::Verde => String::from("Continúe"),
        }
    }
}

fn main(){
    let color_semaforo = Semaforo::Rojo;
    color_semaforo.accion();
}