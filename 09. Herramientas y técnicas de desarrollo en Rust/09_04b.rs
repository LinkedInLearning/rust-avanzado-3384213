/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calcular_cociente(){
        assert_eq!(calcular_cociente(10,3), 3)
    }

    #[test]
    #[should_panic(expected = "El divisor no puede ser cero.")]
    fn test_calcular_cociente_division_entre_cero(){
        calcular_cociente(10,0);
    }
}

 fn main(){

 }
 
 fn calcular_cociente(dividendo: i32, divisor: i32) -> i32 {
     if divisor == 0 {
         panic!("El divisor no puede ser cero.");
     }
     else {
         dividendo / divisor
     }
 }