#[allow(unused)]

fn calcular(mut tupla: (f64, f64, f64)) -> f64{
    tupla.0*tupla.1*tupla.2
}

fn modificar(mut tupla: (f64, i32, char)) ->(f64, i32, char){
    tupla.0=tupla.0+0.3;
    tupla.1=tupla.1*2;
    tupla.2='b';

    tupla
}


fn main() {
    let tupla:(String, String, u32) = ("hector".parse().unwrap(), "hernandez".parse().unwrap(), 26);
    println!("{} {} {} años", tupla.0, tupla.1, tupla.2);

    //tupla mutable que se escribe con los datos de la primera
    let tupla2 = (tupla.0, tupla.1+" arieta", tupla.2-1);
    println!("{} {} {} años", tupla2.0, tupla2.1, tupla2.2);

    println!("{}", calcular((2.8, 3.3, 3.5)));

    let tupla3:(f64,i32, char) =(6.7, 3, 'a');
    println!("tupla3 modificada {:?}", modificar(tupla3));
    //println!("0: {} 1:{} 2:{}", tupla3.0, tupla3.1, tupla3.2);

}
