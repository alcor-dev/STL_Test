use byteorder::{self, LittleEndian, ReadBytesExt};
use std::{fs, env, io, str};
use std::io::{prelude::*, Error};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    //los args comienzan con el comando, no con los atributos introducidos, por eso pasamos al 1
    let filename = &args[1];
    
    execute_analysis(filename);

    //read_text(&filename);

    //esta prueba chorra confirma mis sospechas
    //println!("El segundo argumento es: {}", &args[2]);

    //args[0] es el comando mismo
    println!("\n{}", &args[0]);
}

//Esto ejecuta un sencillo análisis que devuelve un boolean y según el resultado busca si el archivo es binario o no
fn execute_analysis(filename: &String ) {
    let bool = check_asciiSTL((&mut &filename).to_string());
    if !bool {
        let mut data_content = File::open(filename).expect("No ha sido posible abrir el archivo");
        read_binarySTL(&mut data_content);
    }
}

//Podría borrar esto pero es un error interesante de observar
fn read_text(filename: &String) {
    
    let content = fs::read_to_string(filename).expect("error");
    println!("{}", content);
    let bool = check_asciiSTL(content);   
    
}

//Simple chequeo para ver si el archivo es ASCII o _booleano
fn check_asciiSTL(content: String) -> bool {
    let mut counter: u32 = 0;
    if content.starts_with("solid") {
        println!("Esto ES un archivo STL ASCII");
        if content.contains("facet normal") {
            //matches busca todas las veces que aparece lo que se le ponga como entrada
            //seguido de .count() para lectura de archivos
            counter = content.matches("facet normal").count() as u32;
            //como los quads son dos triángulos pues dividimos por 2 los triángulos para obtener los quads resultantes
            println!("Hay: {} triángulos, que serían {} quads", counter, (&counter/2));
            return true
        }
    }
    false
}

//Al leer el booleano y ver que no es un ascii se activa esto 
fn read_binarySTL(file:&mut File) {
    println!("Es un archivo binario");

    //Manda una comprobación del archivo (ya no manda como originalmente una referencia de &String)
    let header = check_binarySTL(file);
    //En caso de que salga un error, le damos un mensaje predefinido y con el 
    //expect podemos coger el Result sin el Error, pudiendo interaccionar mucho más facilmente
    let polygon = create_triangle_list(file, header).expect("Error creando polígono");
    println!("El número de polígonos es de {} triángulos y {} quads", polygon.header.num_triangles, polygon.header.num_triangles/2);
    //Comprobado que funciona, madre mía
    //println!("{:?}", polygon);    
}

//Devuelve un polígono usando el header antes creado y el ahora creado vector de triángulos
fn create_triangle_list<T: ReadBytesExt> (input: &mut T, header: Header) -> Result<Polygon, Error> {
    let mut triangles = Vec::new();

    //Recorre todos los triángulos individualmente primero, leyendo los vértices de cada triángulo y luego los triángulos creados con estos
    for _ in 0..header.num_triangles {
        triangles.push(read_triangle(input)?);
    }

    Ok(Polygon{ header, triangles})
}

fn check_binarySTL(file: &mut File) -> Header {    
    //leemos desde el byte 80, 4 bytes;
    //se puede usar además de [4;80] -> [08u; 80];
    let mut buffer = [4; 80];

    //ABRIMOS EL ARCHIVO
    //LO LEEMOS USANDO PARA REFERENCIA DE QUÉ BYTES COGER
    file.read(&mut buffer);    

    //Ahora Buffer se ha transformado en los datos que hemos cortado del archivo    
    //leemos y el file se convierte en la parte seccionada
    //convertimos la lectura de ese archivo a U32 con un crate externo usando LittleEndian
    let num_triangulos = file.read_u32::<LittleEndian>().expect("Error");

    //Mini comprobación de tipo de archivo por curiosidad
    println!("{:?}", file);
    //LO LEE

    //Ya no es necesario que sea tan redundante y así no confunde a nadie
    /*println!("Hay: {} triángulos en esta figura, que serían {} quads", num_triangulos, (&num_triangulos/2));*/

    //Devolvemos el header y el número de triángulos dentro de un struct
    Header{header: buffer, num_triangles: num_triangulos}
}

//Para devolver este sistema, hace falta indicar tanto el resultado, como el posible error
fn read_point <T: ReadBytesExt> (input: &mut T) -> Result <[f32; 3], Error> {
    //La ? del final indica que algo puede devolver un error
    let p1 = input.read_f32::<LittleEndian>()?;
    let p2 = input.read_f32::<LittleEndian>()?;
    let p3 = input.read_f32::<LittleEndian>()?;

    //En caso de que todo salga bien, que mande ese pequeño array de vuelta
    Ok([p1, p2, p3])
}

//Mismo proceso para sacar los triángulos y sus variables asociadas
fn read_triangle <T: ReadBytesExt> (input: &mut T) -> Result <Triangle, Error> {
    let normal = read_point(input)?;
    let v1 = read_point(input)?;
    let v2 = read_point(input)?;
    let v3 = read_point(input)?;
    let attribute = input.read_u16::<LittleEndian>()?;

    //En caso de que todo salga bien, que mande un objeto Triangle de vuelta
    Ok(Triangle { normal, v1, v2, v3, attribute})
}

//Creamos una estructura que luego guardaremos en un vector de triángulos
#[derive(Debug)]
struct Triangle {
    normal: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    v3: [f32; 3],
    attribute: u16,
}


//Creamos un header con los datos del header y el número de triángulos
#[derive(Debug)]
struct Header {
    header: [u8; 80],
    num_triangles: u32,
}

//Creamos un struct con otros structs sirviendo de base
#[derive(Debug)]
struct Polygon {
    header: Header,
    triangles: Vec::<Triangle>,
}

