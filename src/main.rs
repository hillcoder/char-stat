use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn update_count(m_array: &mut[i32],m_index: usize){
    m_array[m_index]+=1;
}

fn print_statistic(m_array: &mut[i32]){
    for n in 0..m_array.len(){
        let m_char: u8= n as u8 +65;
        println!("{}:{}", m_char as char , m_array[n]);
    }
}



fn read_file_line_by_line(str: &String) -> String {
    let path = Path::new(&str);
    let mut file: File = File::open(path).expect("File not found");
    let mut data: String= String::new();

    file.read_to_string(&mut data).expect("Error while reading file");

    return data;
}
 
fn main() {
    let mut my_char = [0;26];
    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        println!("Usage:");
        println!("{} <filename>",&args[0]);
        return;
    }

    if !Path::new(&args[1]).exists(){
        println!("File {} doesn't exists",&args[1]);
        return;
    }

    println!("{}",&args[1]);
    let my_file = read_file_line_by_line(&args[1]);
    println!("{}",&my_file);
    
    for c in my_file.chars() { 
        if c.is_ascii_alphabetic(){
            let index: u32 = c.to_ascii_uppercase() as u32 - 65;
            update_count(&mut my_char,index as usize);
        }
    }

    print_statistic(&mut my_char);
}

