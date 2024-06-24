use std::io;

fn main() {
    println!("Calculadora Matemática");

    loop {
        println!("Escolha a operação:");
        println!("1. Soma");
        println!("2. Subtração");
        println!("3. Multiplicação");
        println!("4. Divisão");
        println!("5. Sair");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Falha ao ler a linha");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, insira um número válido.");
                continue;
            }
        };

        match choice {
            1 => {
                let (num1, num2) = get_two_numbers();
                println!("Resultado da soma: {}", num1 + num2);
            },
            2 => {
                let (num1, num2) = get_two_numbers();
                println!("Resultado da subtração: {}", num1 - num2);
            },
            3 => {
                let (num1, num2) = get_two_numbers();
                println!("Resultado da multiplicação: {}", num1 * num2);
            },
            4 => {
                let (num1, num2) = get_two_numbers();
                if num2 != 0 {
                    println!("Resultado da divisão: {}", num1 / num2);
                } else {
                    println!("Erro: Divisão por zero.");
                }
            },
            5 => {
                println!("Encerrando o programa.");
                break;
            },
            _ => println!("Opção inválida. Escolha uma opção de 1 a 5."),
        }
    }
}

fn get_two_numbers() -> (i32, i32) {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Digite o primeiro número:");
    io::stdin().read_line(&mut num1)
        .expect("Falha ao ler a linha");
    let num1: i32 = num1.trim().parse()
        .expect("Por favor, insira um número válido");

    println!("Digite o segundo número:");
    io::stdin().read_line(&mut num2)
        .expect("Falha ao ler a linha");
    let num2: i32 = num2.trim().parse()
        .expect("Por favor, insira um número válido");

    (num1, num2)
}
