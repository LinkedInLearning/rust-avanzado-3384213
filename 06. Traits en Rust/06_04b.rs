/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

struct Empleado {
    nombre: String,
    sueldo_bruto_anual: f64,
}

struct Responsable {
    nombre: String,
    sueldo_bruto_anual: f64,
}

trait CalculoBonificacion {
    fn calcula_bonificacion(&self) -> f64;
}

impl CalculoBonificacion for Empleado {

    fn calcula_bonificacion(&self) -> f64 {
        self.sueldo_bruto_anual / 12.00
    }
}

impl CalculoBonificacion for Responsable {

    fn calcula_bonificacion(&self) -> f64 {
        (self.sueldo_bruto_anual / 12.00) * 2.0
    }
}

fn main(){
    
}