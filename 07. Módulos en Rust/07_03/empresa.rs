/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

pub struct Empleado {
    nombre: String,
    sueldo_bruto_anual: f64,
    departamento: String,
}

impl Empleado {

    pub fn actualizar_sueldo(&mut self, nuevo_sueldo: f64){
        self.sueldo_bruto_anual = nuevo_sueldo;
    }

    pub fn obtener_nombre(&self) -> &String {
        &self.nombre
    }

    pub fn obtener_sueldo_bruto_anual(&self) -> f64 {
        self.sueldo_bruto_anual
    }

    pub fn obtener_departamento(&self) -> &String {
        &self.departamento
    }

    pub fn calcular_sueldo_mensual(&self) -> f64{
        self.sueldo_bruto_anual / 12.0
    }

    pub fn nuevo(nombre: &str, sueldo_bruto_anual: f64, departamento: &str) -> Empleado{
        Empleado {
            nombre: nombre.to_string(),
            sueldo_bruto_anual,
            departamento: departamento.to_string(),
        }
    }
}
