#[allow(dead_code)]
fn another_funtions(){
    println!("Another funtions");
}

fn funtions1(x: i32){
    println!("The value of x is {}", x);//参数部分rust和C语言无二致
}

fn print_label_measurement(value: i32,unit_label: char){
    println!("The measurement is {} {}", value, unit_label);//多个参数得显式的指出各个参数的类型
}

fn sta_exp(){
    let y1= 6;//语句执行一些操作没有返回值
            //rust赋值语句没有返回值因此不存在let x = (let y = 6);
    let x1 = 5;

    let z = {
        let x1 = 4;
        x1 + 1//语句计算出一个值，与表达式的形式区别是没有分号
    };

    println!("The value of z is {}", z);
}

fn five() -> i32{

    5//返回值和C语言一样但是，除了显式使用return
     //语句，大都是使用默认最后一个表达式的值为返回值

}

fn plus_one(x: i32) -> i32{

    let m = {
        x + 1//{}确定了作用域
    };

    return x + 1;//使用return语句可以和末尾表达式达到一样的效果，但是return可以在任意地方返回
} 


//控制流

fn if1(number: i32) -> i32{

    if number > 5 {
        println!("condition is true!");
        5
    }else{
        println!("condition is false!");//对于if-else rust的处理和python一样，条件为真即截止
        6//对于返回值的类型一开始就要知道所以if-else中不能允许不同条件代码块中的返回值类型不同。一个变量不能有两个类型
    }
}

fn loop1(){

    loop{
        println!("again!");//循环关键字loop，没有退出条件
    }
    
}


fn loop2(){

    let mut count = 0;
    'counting_up: loop{
        println!("count = {}", count);
        let mut remaining = 10;

        loop{
            println!("remaining = {}", remaining);
            
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;//最外层循环定义的变量作用域最大，作用域依次向内递减
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {}", count);
}

fn loop3(){

    //break和循环的联用
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10{

            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while1(x: i32){

    let mut number: i32 = x;

    while number != 0 {

        println!("number {}", number);
        number -= 1;

    }

    println!("LIFTOFF!!!");
}


fn for1(){

    //rust中最常用的循环结构，相比其他循环，for可以自动检测数组长度，
    //修改数组个数时不必不必修改循环里的代码。增强安全性，减少了bug

    //let mut number: i32 = 5;
    let a = [1, 2, 3, 4, 5];
    
    for element in a.iter(){

        println!("The value of a is {}", element);
    }

    for number in (1..4).rev(){

        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}

fn main() {
   
    for1();

}
