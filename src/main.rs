use regex::Regex;

fn main() {
    /* lenguaje altamente tipado,  */
    /* Definicion de la variable, nombre, se coloca i8 de 8 bits
    i8 para - y + y u8 para solo +
    */

    /* Se coloca mut para poder editarla,
    i8: entero de 8 bits.
    u8: numeros sin signos
    */

    /*   let mut eda : u8 = 18;


     /*  */
     eda = eda + 1;

    // let temperatura: i8 = 5;

     /* Siempre se escribe con ampersand los strings */
     let nombr: &str = "Ervin";

     /* {} placeolder

     */
     println!("Soy {} y tengo {} años !", nombr, eda);

     println!("Por favor introduce tu nombr: ");


     //este se utiliza para que valla mas rapido
     //let mut str_prueba : &str = "str";

     //tiene más opciones, se utriliza para cosas complejas
     let mut nombre : String = String::new();

     /*
     std= libreria estandar de rust. la que se conecta al sistema operativo.
     io= entradas y salidas se utiliza para mostrar datos o recivir datos de consola
     stdin()= se trae de io
     read_line()= trae datos desde la consola y se guarda lo que esta dentro de ()
     unwrap() = manejar errores

      */
     std::io::stdin().read_line(&mut nombre).unwrap();
     nombre = nombre.trim().to_string();

     //vamos a obtener la edad desde la consola y convertir esa edad a un numero
     println!("Por favor introduce tu edad");
     let mut edad : String = String::new();
     std::io::stdin().read_line(&mut edad).unwrap();

     //convertir un string a numero
     // trim() = quita los espacios
     let edad_int : u8 = edad.trim().parse().unwrap();


     println!("Hola que mas {} tu edad es {} ", nombre, edad_int);



     println!("****************Creando condicionales********");
     if edad_int >= 18 {
         print!("Puedes entrar a la discoteca");
     } else {
         print!("Eres menor de edad no puedes entrar");
     } */

/*     let numero_1 = 2;
    let numero_2 = 4;

    let suma = numero_1 + numero_2;

    loop {
        println!(
            "los numeros de de la suma son {} y {}: ",
            numero_1, numero_2
        );

        //pedir nuemro
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int: i16 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("Lo has echo bien, el resultado {} es correcto. ", suma);
            break;
        } else {
            println!(
                "El resultado: {} no es correcto, intentalo de nuevo",
                suma_usuario_int
            )
        }
    } */


}
