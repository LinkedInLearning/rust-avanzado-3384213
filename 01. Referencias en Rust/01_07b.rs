/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main(){

    let mut saldo_cuenta: f64 = 30_215.25;

    actualizar_saldo(&mut saldo_cuenta, 300.0);
    actualizar_saldo(&mut saldo_cuenta, -40.0);
}

fn actualizar_saldo(saldo: &mut f64, importe: f64){
    *saldo += importe;
}