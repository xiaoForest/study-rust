// 学习1
// use std::fs;

fn main() {
    // 学习1
    // let mut line = String::new();
    // println!("请粘贴你的目标网址:");
    // let b1 = std::io::stdin().read_line(&mut line).unwrap();
    // println!("目标网址： , {}", line);
    // println!("读取的字节数为：{}", b1);

    // let url = line;
    // let output = "rust.text";

    // println!("Fetching url: {}", url);
    // let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    // println!("Converting html to markdown...");
    // let md = html2md::parse_html(&body);

    // fs::write(output, md.as_bytes()).unwrap();
    // println!("Converted markdown has been saved in {}.", output);

    // 学习2 数据类型
    // let result = 10.00; // 默认是 f64
    // let interest: f32 = 8.35;
    // let cost: f64 = 15000.600; // 双精度浮点型

    // println!("result value is {}", result);
    // println!("interest is {}", interest);
    // println!("cost is {}", cost);

    // let interest:f32 = 8.5;   // integer assigned to float variable
    // println!("interest is {}",interest);

    // let float_with_separator = 11_000.555_001;
    // println!("float value {}", float_with_separator);

    // let int_with_separator = 50___000;
    // println!("int value {}", int_with_separator);

    // let emoji = '😊'; // 笑脸的那个图
    // println!("int value2 {}", emoji);

    // 学习3 定义变量
    // let mut fees: i32 = 25_000;
    // println!("fees is {} ", fees);
    // fees = 35_000;
    // println!("fees changed is {}", fees);

    // // 学习4 Rust 常量
    // const USER_LIMIT:f32 = 3.14;
    // println!("fees changed is {}", USER_LIMIT);
    // let name: &str = "Mohtashim";
    // let name: usize = name.len();
    // //Error : `NAME` already defined
    // println!("改变 name 常量的类型: {}", name);
}
