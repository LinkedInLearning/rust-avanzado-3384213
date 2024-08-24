/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

/*
 * Definición del tipo enum. Esta definición no necesita incluirse.
 * enum Result<T, E> {
 *    Ok(T),
 *    Err(E),
 * }
 */

fn main(){

    let dividendo: u16 = 216;
    let divisor: u16 = 2;

    match calcular_cociente(dividendo, divisor){
        Ok(resultado) => println!("El resultado es: {}", resultado),
        Err(mensaje) => println!("{}", mensaje),
    }
}

fn calcular_cociente(dividendo: u16, divisor: u16) -> Result<u16, String> {
    
    if divisor == 0 {
        Err(String::from("Error: división entre cero."))
    }
    else {
        Ok(dividendo / divisor)
    }
}