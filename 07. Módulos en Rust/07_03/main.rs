/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

mod empresa;

fn main(){
    
    let mut empleado: empresa::Empleado = empresa::Empleado::nuevo("John", 80_000.00,"Tecnología");

    empleado.actualizar_sueldo(90_000.00);

    println!("El empleado {} pertenece al departamento {},
    y recibe unos ingresos brutos anuales de {}€.",
    empleado.obtener_nombre(), empleado.obtener_departamento(), empleado.obtener_sueldo_bruto_anual());

    let _sueldo_mensual = empleado.calcular_sueldo_mensual();
}