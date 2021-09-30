use std::f32::consts::PI;


enum Figure {
    Cuadrado{lado:f32},
    Circulo{radio:f32},
    Triangulo{a:f32, b:f32}
}

fn area(figura: Figure) -> f32{
    match figura {
        Figure::Circulo {radio}=>radio.powf(2f32)*PI,
        Figure::Cuadrado {lado}=> lado*lado,
        Figure::Triangulo {a, b}=>a*b/2.0f32,
    }
}


fn main() {
    let circulo = Figure::Circulo {radio:4.12f32};
    let cuadrado = Figure::Cuadrado {lado:2.5f32};
    let triangulo = Figure::Triangulo {a:5f32, b:7f32};
    println!("Área círculo: {} U²", area(circulo));
    println!("Área cuadrado: {} U²", area(cuadrado));
    println!("Área triángulo: {} U²", area(triangulo));
}
