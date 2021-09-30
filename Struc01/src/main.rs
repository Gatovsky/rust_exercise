#[derive(Debug)]


struct Persona {
    nombre: String,
    apellido: String,
    edad: u8
}

fn main() {
    let persona1 = Persona {nombre: "hector".parse().unwrap(), apellido: "hernandez".parse().unwrap(), edad: 26};

    println!("Persona 1 se llama {} {} y tiene {} años de edad", persona1.nombre, persona1.apellido, persona1.edad);
    println!("estructura {:?}", persona1);

    {
        let persona2 = Persona {nombre: "rosca".parse().unwrap(), apellido: "oropesa".parse().unwrap(), edad: 22};
        println!("estructura {:?}", persona2);
    }//alcance 2
}