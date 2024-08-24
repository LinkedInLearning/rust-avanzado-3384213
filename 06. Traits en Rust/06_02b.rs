/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

trait SobreLaMedia {
    fn sobre_la_media(&self) -> bool;
}

impl SobreLaMedia for f64 {
    fn sobre_la_media(&self) -> bool {
        *self > 25_750.00
    }
}

fn main(){
    let sueldo_bruto_anual: f64 = 75_350.37;
    let comprprobacion_sueldo = sueldo_bruto_anual.sobre_la_media();
}