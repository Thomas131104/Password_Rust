use std::io;
use rand::Rng;
use rand::seq::SliceRandom; 
use libc::{signal, SIGINT, SIG_IGN};



// Hàm nhập dữ liệu từ bàn phím
fn input(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}



// Hằng số chứa các ký tự mật khẩu hợp lệ
const POPULATION_PASSWORD_NUMBER: &str = "0123456789";




// Hàm tạo mật khẩu ngẫu nhiên
fn generate_password(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let population: Vec<char> = POPULATION_PASSWORD_NUMBER.chars().collect(); // Chuyển thành Vec<char>

    let mut password = String::new();
    for _ in 0..len {
        // Chọn ngẫu nhiên một ký tự từ Vec<char>
        let random_char = population.choose(&mut rng).unwrap(); // unwrap an toàn vì luôn có phần tử
        password.push(*random_char); // Thêm ký tự vào mật khẩu
    }
    password
}



// Chương trình chính
fn program() {
    let mut rng = rand::thread_rng();
    let min = 5;
    let max = 7;
    let len = rng.gen_range(min..=max);
    let password = generate_password(len);
    println!("Mật khẩu này có từ {} đến {} ký tự.", min, max);

    loop {
        let x = input("Nhập số: ");

        if x.len() != password.len() {
            println!("Bạn nhập sai số lượng kí tự. Yêu cầu {} kí tự.", password.len());
            continue;
        }

        let password_chars: Vec<char> = password.chars().collect();
        let mut is_correct = true;

        for (i, c) in x.chars().enumerate() {
            // Chuyển đổi ký tự nhập và mật khẩu thành số
            let input_digit = c.to_digit(10);
            let password_digit = password_chars[i].to_digit(10).expect("Mật khẩu không hợp lệ!");

            match input_digit {
                Some(digit) => {
                    if digit == password_digit {
                        println!("Index {} : {} : T", i + 1, digit);
                    } else {
                        println!("Index {} : {} : F", i + 1, digit);
                        is_correct = false;
                    }
                }
                None => {
                    println!("Ký tự '{}' không phải là số hợp lệ.", c);
                    is_correct = false;
                }
            }
        }

        if is_correct {
            println!("Mật khẩu chính xác!");
            break;
        } else {
            println!("Mật khẩu sai. Thử lại!");
        }
    }
}




// Hàm main
fn main() {
    unsafe {
        signal(SIGINT, SIG_IGN);
    }
    program();
}
