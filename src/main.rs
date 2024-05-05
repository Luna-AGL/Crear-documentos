use std::io::{self, BufRead};
use std::fs;
use std::path::Path;

fn main() {
    println!("Bienvenido al gestor de archivos y notas de texto!");

    loop {
        println!("\nSelecciona una opción:");
        println!("1. Crear carpeta");
        println!("2. Crear nota de texto");
        println!("3. Salir");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim() {
            "1" => crear_carpeta(),
            "2" => crear_nota(),
            "3" => break,
            _ => println!("Opción no válida"),
        }
    }

    println!("¡Hasta luego!");
}

fn crear_carpeta() {
    println!("Ingrese el nombre de la carpeta:");
    let mut nombre_carpeta = String::new();
    io::stdin().read_line(&mut nombre_carpeta).expect("Error al leer la entrada");

    let nombre_carpeta = nombre_carpeta.trim();
    let path = Path::new(nombre_carpeta);

    match fs::create_dir(path) {
        Ok(_) => println!("Carpeta '{}' creada con éxito", nombre_carpeta),
        Err(err) => eprintln!("Error al crear la carpeta: {}", err),
    }
}

fn crear_nota() {
    println!("Ingrese el nombre de la nota:");
    let mut nombre_nota = String::new();
    io::stdin().read_line(&mut nombre_nota).expect("Error al leer la entrada");

    println!("Ingrese el contenido de la nota (Agrega en una linea abajo <outP> para finalizar):");
    let mut contenido_nota = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Error al leer la entrada");
        if line.trim() == "<outP>" {
            break;
        }
        contenido_nota.push_str(&line);
        contenido_nota.push('\n');
    }

    let nombre_nota = nombre_nota.trim();
    let path = Path::new(nombre_nota);

    match fs::write(path, contenido_nota) {
        Ok(_) => println!("Nota '{}' creada con éxito", nombre_nota),
        Err(err) => eprintln!("Error al crear la nota: {}", err),
    }
}
