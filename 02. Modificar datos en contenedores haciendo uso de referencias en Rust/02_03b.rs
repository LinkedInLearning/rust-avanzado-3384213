/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main(){

    let mut precios = vec![1.50, 2.35, 5.20, 9.99];

    incrementar_precios(&mut precios);

    println!("{:?}", precios);
}

fn incrementar_precios(precios: &mut Vec<f64>){
    for i in 0..precios.len(){
        precios[i] += 1.0;
    }
}