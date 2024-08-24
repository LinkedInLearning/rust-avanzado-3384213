/*
 * Curso: Rust Avanzado
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust en la web: eliezerlopez.rs
 * */

fn main() {

    let web = String::from("https://eliezerlopez.rs");

    let recursos_sobre_rust = web;
    // 'web' ya no es válida a partir de esta línea
    
    // Préstamo de 'recursos_sobre_rust' a la función 'imprimir_mensaje'.
    imprimir_mensaje(&recursos_sobre_rust);

    // println!("{}", web); --> implica error de compilación
    
    println!("\nMás recursos sobre Rust en: {}\n", recursos_sobre_rust);
}

fn imprimir_mensaje(url: &String) {
    println!("\n\n> Suscríbete a mi boletín para estar al tanto\n> de las últimas novedades sobre lenguajes de programación\n> de alto rendimiento y tendencias digitales en: {}", url);
}