// Задача: написать алгоритм, принимающий на вход строку, разделенную пробелами, и длину строки в символах.
// Необходимо разбить исходный текст на строки и выравнять по указанной длине строки с помощью пробелов.
// Расстояние между словами нужно заполнять равным количеством пробелов, если же это не возможно, то добавляем
// еще по пробелу между словами слева направо. Если в строке помещается только 1 слово, то дополнить строку 
// пробелами справа Результат вернуть в виде единой строки, где полученный список равных по ширине строк склеен 
// с помощью символа перевода строки. 

use line_adjustment::line_adj::get_formated_str;

fn main() {
    let mut buffer = String::new();
    let length: i32 = 26;

    buffer = "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua".to_string();
    let result = get_formated_str(&buffer, length).to_string();
    print!("{}", result);
}