/*use std::io;

fn main(){
    println!("Guess the number!");
    
    println!("Please input your guess.");

    let mut guess = String::new();//创建一个可变变量，mut表示可变

    //::是语法表示new()是一个关联函数。string表示rust字符型变量
    //，可以定义字符型变量。创建一个可变变量，并将其绑定在空的string实例上
    io::stdin().read_line(&mut guess)//&表示一个引用，默认不可变，使用mut使其可变
        .expect("Failed to read line");//.method_name()是调用函数的模式

    println!("You guessed: {}", guess);//io：result一个枚举类型值，包
                                       //含成功和错误的结果。结合其他函数对结果进行处理。
                                       //expect()就是对read))line()的io:result结果处理
                                       //println是格式化字符串的第一个参数，’{}‘预留的占位符
                                       //使用多个{}可以打印多个值
    let a = 3;
    let b = 5;
    println!("b = {} and a = {}",a,b);
}*/
/*use std::io;
use rand::Rng;

fn main(){
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //使用rand::thread_rng获取rng中trait中定义的方法

    

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess);
    
    println!("The secret number is: {}", secret_number);
}*/

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("secret number: {}", secret_number);
    loop{
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");
    
        let guess: u32 = match guess.trim().parse(){//match条件执行语句，可以对返回结果进行条件判断，修改判断结果
            Ok(num) => num,
            Err(_) => continue,
        };
            //.expect("Please type a number!");

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),//rust具有一定的推断
            //能力，但是没有提示信息的情况下无法分辨是数字类型还是字符型变量
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }    
}