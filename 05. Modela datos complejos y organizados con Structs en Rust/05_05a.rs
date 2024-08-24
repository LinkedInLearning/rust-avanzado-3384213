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

impl Empleado {

    fn obtener_nombre(&self) -> &String {
        &self.nombre
    }

    fn obtener_sueldo_bruto_anual(&self) -> f64 {
        self.sueldo_bruto_anual
    }

    fn obtener_departamento(&self) -> &String {
        &self.departamento
    }

    fn calcular_sueldo_mensual(&self) -> f64{
        self.sueldo_bruto_anual / 12.0
    }

    fn nuevo(nombre: &str, sueldo_bruto_anual: f64, departamento: &str) -> Empleado{
        Empleado {
            nombre: nombre.to_string(),
            sueldo_bruto_anual,
            departamento: departamento.to_string(),
        }
    }
}

fn main(){
    
    let empleado = Empleado::nuevo("John", 80_000.00,"Tecnología");

    println!("El empleado {} pertenece al departamento {},
    y recibe unos ingresos brutos anuales de {}€.",
    empleado.obtener_nombre(), empleado.obtener_departamento(), empleado.obtener_sueldo_bruto_anual());

    let sueldo_mensual = empleado.calcular_sueldo_mensual();
}