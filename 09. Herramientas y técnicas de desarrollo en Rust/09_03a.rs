/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main(){
    let mut numeros_lost = [4, 8, 15, 16, 23, 42];
    incrementa_en_una_unidad(&mut numeros_lost);
}

fn incrementa_en_una_unidad(numeros_lost: &mut [i32]){
    for numero in numeros_lost.iter_mut() {
        *numero += 1;
    }
}