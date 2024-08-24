/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main(){

    let dividendo: u16 = 216;
    let divisor: u16 = 0;

    match division_entera(dividendo, divisor){
        Ok((cociente, resto)) => println!("Cociente: {}, Resto: {}", cociente, resto),
        Err(mensaje) => println!("{}", mensaje),
    }
}

fn division_entera(dividendo: u16, divisor: u16) -> Result<(u16, u16), String> {
    
    let cociente = calcular_cociente(dividendo, divisor)?;
    let resto = dividendo % divisor;

    Ok((cociente, resto))
}

fn calcular_cociente(dividendo: u16, divisor: u16) -> Result<u16, String> {
    
    if divisor == 0 {
        Err(String::from("Error: división entre cero."))
    }
    else {
        Ok(dividendo / divisor)
    }
}