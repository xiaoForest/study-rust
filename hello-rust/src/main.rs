use std::fs;

fn main() {
    let mut line = String::new();
    println!("请粘贴你的目标网址:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("目标网址： , {}", line);
    println!("读取的字节数为：{}", b1);

    let url = line;
    let output = "rust.text";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}
