use byteorder::{self, LittleEndian, ReadBytesExt};
use std::{fs, env, io, str};
use std::io::{prelude::*, BufReader};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    //los args comienzan con el comando, no con los atributos introducidos, por eso pasamos al 1
    let filename = &args[1];
    
    check_asciiSTL(&filename);
    //read_text(&filename);

    //esta prueba chorra confirma mis sospechas
    //println!("El segundo argumento es: {}", &args[2]);

    //args[0] es el comando mismo
    println!("\n{}", &args[0]);
}

fn read_text(filename: &String) {
    
    let content = fs::read_to_string(filename).expect("error");
    println!("{}", content);
    check_asciiSTL(&content);
}

fn check_asciiSTL(content: &String) {
    let mut counter: u32 = 0;
    if content.starts_with("solid") {
        println!("Esto ES un archivo STL ASCII");
        if content.contains("facet normal") {
            //matches busca todas las veces que aparece lo que se le ponga como entrada
            //seguido de .count() para lectura de archivos
            counter = content.matches("facet normal").count() as u32;
            //como los quads son dos triángulos pues dividimos por 2 los triángulos para obtener los quads resultantes
            println!("Hay: {} triángulos, que serían {} quads", counter, (&counter/2));
        }
    } else {
        println!("Esto NO es un STL ASCII \nProbando si es un STL Binario...");
        check_binarySTL(&content);
    }
}

fn check_binarySTL(filename: &String) {    
    //leemos desde el byte 80, 4 bytes;
    //se puede usar además de [4;80] -> [08u; 80];
    let mut buffer = [4; 80];
    //ABRIMOS EL ARCHIVO
    let mut file = File::open(&filename).expect("Error");
    //LO LEEMOS USANDO PARA REFERENCIA DE QUÉ BYTES COGER
    file.read(&mut buffer);
    //leemos y el file se convierte en la parte seccionada
    //convertimos la lectura de ese archivo a U32 con un crate externo usando LittleEndian
    let num_triangulos = file.read_u32::<LittleEndian>().expect("Error");
    //LO LEE
    println!("Hay: {} triángulos en esta figura, que serían {} quads", num_triangulos, (&num_triangulos/2));
}
struct Triangle {
    normal: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    v3: [f32; 3],
    attribute: u16,
}

struct Header {
    header: [u8; 80],
    num_triangles: u32,
}

