use std::io;
const WE_1: u32 = 123 * 123;
#[allow(dead_code)]//得cargo build后才起作用
/*fn main() {
    
    //let x = 5;//immutable variable
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //常量表达式和C语言无二致,但是表达
    //式形式方便阅读和理解，同时要求大写和下划线的形式给出
    //关键字是const
    
    println!("The immutable of WE_1 is: {}", WE_1);
}

fn main(){
    let x = 5;
    let x = x + 1;//没用mut的情况下二次定义同名变量会隐藏前一个变量的值

    {
        let x = x *2;
        let mut x = 2;//使用mut二次定义同名变量相当于定义一个全新的变量
        println!("The value of x is: {}", x);
    }
    

    println!("The value of x is: {}", x);
} 

fn main(){
    let spaces = "    ";
    println!("sapces: {}", spaces);
    let spaces = spaces.len();

    println!("sapces: {}", spaces);
}

fn main(){
    let x: u32 = 32;//相对于C语言rust对于整形变量溢出，有相对完善的解决方法
    let y: i32 = -32;//，eg：wrapping_*允许显式的定义溢出，即溢出后不报错
    let z: f64 = 32.0;
    let a = true;//布尔值可以显示和隐式的给出
    let b: bool = false;

    let c = 'y';//表示传递字符串

    println!("x: {} y: {} z: {}",x ,y ,z);//rust中char类型变量为四个字节，编码是按unicode。
    let c: char = '@';//相比ascii码可以表示更多字符

    //复合类型(元组和数组)
    //元组
    let tup: (i32, u32, f64, bool) = (-32, 43, 456.89, true);//元组是将一个或多值绑定在变量上
    let (h, i, j, k) = tup;//tup变量通过模式匹配的方式被解构到变量hijk上
    let go = tup.3;//元组可以通过索引来访问

    println!("The value of h is {}", h);
    println!("The value of go is {}", go);

    //数组
    let m = [1, 2, 3, 4, 5];//rust中数组的长度是固定的，数组中每一个元素的类型是相同的
                            //vector是向量类型的数组定义，长度是可变的同时是数组是上位替换
                            //对于不确定长度的数据最好用Vector，确定的用数组
    let m: [i32; 5] = [1, 2, 3, 4, 5];//定义一个i32长度为5的数组

    let m = [3;5];//创建所有元素为3，长度为5的数组
    let first = m[0];
    println!("The value of m[0] is: {}", first);
}*/

//从数组中读取值
fn main(){
    let a: [i32;5] = [1, 2, 3, 4, 5];
    let mut index = String::new();

    println!("Please enter an array index.");
    io::stdin().read_line(&mut index)
            .expect("Failed to read line!");
    
    let index: usize = index//rust中数组和切片的索引都是usize类型的
        .trim()
        .parse()
        .expect("Type a number!");

    let temp = a[index];

    println!("The value of temp is {}", temp);
}

