/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

struct Empleado {
    nombre: String,
    sueldo_bruto_anual: f64,
    departamento: String,
}

fn main(){
    
    let empleado = Empleado {
        nombre: String::from("John"),
        sueldo_bruto_anual: 80_000.00,
        departamento: String::from("Tecnología"),
    };

    println!("El empleado {} pertenece al departamento {},
    y recibe unos ingresos brutos anuales de {}€.",
    empleado.nombre, empleado.departamento, empleado.sueldo_bruto_anual);
}