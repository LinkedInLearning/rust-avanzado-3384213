/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main() {
    let numeros_lost = [4, 8, 15, 16, 23, 42];
    println!("El número más alto es: {}\n",
        localiza_el_valor_mas_alto(&numeros_lost));
}

fn localiza_el_valor_mas_alto(numeros_lost: &[i32]) -> &i32 {
    let mut valor_mas_alto = &numeros_lost[0];
    for numero in numeros_lost {
        if numero > valor_mas_alto {
            valor_mas_alto = numero;
        }
    }
    valor_mas_alto
}