/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

#[derive(Debug)]
struct Color {
    nombre: String,
}

fn main(){
    let mut numeros_lost = [4, 8, 15, 16, 23, 42];
    println!("Tipo de numeros_lost: {}", std::any::type_name_of_val(&numeros_lost));
    incrementa_en_una_unidad(&mut numeros_lost);
    println!("Valores incrementados: {:?}", numeros_lost);

    let color_favorito = Color {nombre: "Azul".to_string()};
    println!("\nColor favorito: {:#?}", color_favorito);
}

fn incrementa_en_una_unidad(numeros_lost: &mut [i32]){
    println!("--- Comienza la función en este punto ---");
    for numero in numeros_lost.iter_mut() {
        println!("Numero antes: {}", *numero);
        *numero += 1;
        println!("Numero después: {:X}", *numero);
    }
}