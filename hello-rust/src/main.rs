// 学习1
// use std::fs;

// fn main() {
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

    // 学习4  字符串

    // let mut z = String::new();
    // z.push_str("简单教程 简单编程");
    // println!("{}", z);

    // let name1 = "你好，简单教程
    // 简单编程3
    // 简单编程2
    // 简单编程1".to_string();
    // println!("{}",name1);

    // let name1 = "简单教程 简单编
    // 程3
    // 程2
    // 程1".to_string(); //原字符串对象
    // let name2 = name1.replace("1","滚");    // 查找并替换
    // println!("{}",name2);

    // let mut company = "简单教程".to_string();
    // company.push_str("t简单编程2");
    // println!("{}",company);

    // let example_string = String::from("简单教程1");
    // print_literal(example_string.as_str());
    //     let msg = "简单教程 简 单编程 www.twle.cn
    //    https://www.twle.cn"
    //         .to_string();
    //     let mut i = 1;

    //     for token in msg.split_whitespace() {
    //         println!("token {} {}", i, token);
    //         i += 1;
    //     }

    // let fullname = "李白，诗仙，唐朝";

    // for token in fullname.split("，") {
    //     println!("token is {}", token);
    // }

    // // 存储在一个向量中
    // println!("\n");
    // let tokens: Vec<&str> = fullname.split("，").collect();
    // println!("姓名 is {}", tokens[0]);
    // println!("称号 {}", tokens[1]);
    // println!("朝代 {}", tokens[2]);
    // let n1 = "简单教程 www.twle.cn".to_string();

    // for n in n1.chars() {
    //     println!("{}", n);
    // }

    // let n1 = "哈哈哈".to_string();
    // let n2 = "简单编程2".to_string();

    // let n3 = n1 + &n2; // 需要传递 n2 的引用
    // println!("{}",n3);

    // let number = 2020;
    // let number_as_string = number.to_string();

    // // 转换数字为字符串类型
    // println!("{}", number_as_string);
    // println!("{}", number_as_string != "2020");

    // let n1 =32;
    // let n2 = "简单编程".to_string();
    // let n3 = format!("{} {}",n1,n2);
    // println!("{}", n3);

    // Rust 位运算符范例 有点不是狠理解
    // let a: i32 = 2; // 二进制表示为 0 0 0 0 0 0 1 0
    // let b: i32 = 4; // 二进制表示为 0 0 0 0 0 0 1 1

    // let mut result: i32;

    // result = a & b;
    // println!("(a & b) => {} ", result);

    // result = a | b;
    // println!("(a | b) => {} ", result);

    // result = a ^ b;
    // println!("(a ^ b) => {} ", result);

    // result = !b;
    // println!("(!b) => {} ", result);

    // result = a << b;
    // println!("(a << b) => {}", result);

    // result = a >> b;
    // println!("(a >> b) => {}", result);
    // let state_code = "KA";
    // let state = match state_code {
    //     "MH" => {
    //         println!("Found match for MH");
    //         "Maharashtra"
    //     }
    //     "KL" => "Kerala",
    //     "KA" => "Karnadaka",
    //     "GA" => "Goa",
    //     _ => "Unknown",
    // };
    // println!("State name is {}", state);

    // let mut x = 1;
    // while x <= 6 {
    //     println!("inside loop x value is {}", x);
    //     x += 1;
    // }
    // println!("outside loop x value is {}", x);

    // let mut x = 0;
    // loop {
    //    x+=1;
    //    println!("x={}",x);
    // }

    // for x in 4..21 {
    //     if 5 == x {
    //         continue;
    //     }
    //     println!("x is {}", x);
    // }

    // fn get_pi()->f64 {
    //     22.0/7.0
    //  }
    // let b: (i32, bool, f64) = (30, true, 7.9);
    // print(b);
    // let mut arr= [23,3,4,5,6,7,8,9,10,11,12];
    // arr[2] = 666;
    // println!("{:?}",arr);
    // for index in 0..11 {
    //     println!("index is: {} & value is : {}",index,arr[index]);
    //  }
    // let mut arr = [10, 20, 30];
    // update(&mut arr);
    // println!("Inside main {:?}", arr);

    // let v = vec![1, 2, 3];

    // // 赋值会导致两个变量都对同一个数据拥有所有权
    // // 因为两个变量指向了相同的内存块
    // let v2 = v;

    // // Rust 会检查两个变量是否同时拥有堆上内存块的所有权。
    // // 如果发生所有权竞争，它会自动将所有权判给给新的变量
    // // 运行出错，因为 v 不再拥有数据的所有权
    // println!("{:?}", v2);
    // let v = vec![1, 2, 3]; // 向量 v 拥有堆上数据的所有权
    // let v2 = v; // 向量 v 将所有权转让给 v2
    // let v2_return = display(v2);
    // println!("In main {:?}", v2_return);
    // let mut name:String = String::from("TutorialsPoint");
    // display(&mut name);  // 传递一个可变引用
    // println!("The value of name after modification is:{}",name);
// }
// fn display(param_name:&mut String){
//     println!("param_name value is :{}",param_name);
//     param_name.push_str(" Rocks"); // 修改字符串，追加一些字符
//  }
// fn update(arr: &mut [i32; 3]) {
//     for i in 0..3 {
//         arr[i] = 0;
//     }
//     println!("Inside update {:?}", arr);
// }
// fn print(x: (i32, bool, f64)) {
//     let (age, is_male, cgpa) = x;
//     println!(
//         "Age is {} , isMale? {},cgpa is
//     {}",
//         age, is_male, cgpa
//     );
// }
// fn print_literal(data: &str) {
//     println!("显示的字符串字面量是: {}", data);
// }
// 函数返回值语法格式
// Rust 语言的返回值定义语法与其它语言有所不同，它是通过在 小括号后面使用 箭头 ( -> ) 加上数据类型 来定义的。
// fn display(v: Vec<i32>) -> Vec<i32> {
//     // 返回同一个向量
//     println!("inside display {:?}", v);
//     return v;
// }
//定义一个结构体
struct Employee {
    name:String,
    company:String,
    age:u32
 }
 fn main() {
    //初始化结构体
    let emp1 = Employee {
       company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
    };
    let emp2 = Employee{
       company:String::from("TutorialsPoint"),
       name:String::from("Kannan"),
       age:32
    };
    //将结构体作为参数传递给 display
    display(emp1);
    display(emp2);
 }
 
 // 使用点号(.) 访问符访问结构体的元素并输出它么的值
 fn display( emp:Employee){
    println!("Name is :{} company is {} age is 
    {}",emp.name,emp.company,emp.age);
 }