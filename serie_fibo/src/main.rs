#[warn(unused_variables)]
#[warn(unused_imports)]

use std::io;

fn fibonacci(n:i128) -> i128{
    if n<2{
        return n;
    }
    return fibonacci(n-1)+fibonacci(n-2);

}

fn main() {
    //let serie_fibo =[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765];
    let mut contador:i32 =0;
    let mut a = String::new();
    let mut b = String::new();


    io::stdin()
        .read_line(&mut a)
        .expect("@Fall. imposible leer la línea");
    let a:i128 = a
        .trim()
        .parse()
        .expect("entrada entered was not a number");

    io::stdin()
        .read_line(&mut b)
        .expect("@Fall. imposible leer la línea");
    let b:i128 = b
        .trim()
        .parse()
        .expect("entrada entered was not a number");



for i in 0..16{
    let num_fibo:i128 = fibonacci(i);
    if a==1 && b==2{
        if num_fibo>=a && num_fibo <= b{
            contador+=1;
        }
    }
    if num_fibo > a && num_fibo < b{
        contador+=1;
    }
}

    println!("{}", contador);
}
