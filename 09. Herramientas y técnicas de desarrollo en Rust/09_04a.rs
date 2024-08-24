/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

 /*
  * Este archivo debe encontrarse en la carpeta src de un proyecto
  * creado con cargo. El motivo es que utilizaremos cargo
  * para ejecución de pruebas de unidad.
  */
fn main(){

}

fn calcular_cociente(dividendo: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("El divisor no puede ser cero.");
    }
    else {
        dividendo / divisor
    }
}