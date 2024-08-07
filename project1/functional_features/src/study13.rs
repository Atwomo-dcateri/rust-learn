//chapter13 functional-features

//函数式语言功能：迭代器和闭包

//函数式编程

//函数式编程（functional programming）。函数式编程风格通常包含将函数作为
//参数值或其他函数的返回值、将函数赋值给变量以供之后执行。。。


//rust语言中具备函数式编程的内容：


/*

闭包（Closures），一个可以储存在变量里的类似函数的结构 
迭代器（Iterators），一种处理元素序列的方式 
何使用这些功能来改进第十二章的 I/O 项目。
注意：这两个功能的性能。（他们的速度超乎你的想象！）

*/


//13-01 closures 闭包

//闭包：可以捕获环境的匿名函数

//通过前端输入，用户调用rust实现的后端，制定锻炼计划
/*
Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。
可以在一个地方 创建闭包，然后在不同的上下文中执行闭包运算。
不同于函数，闭包允许捕获调用者作用域中的值。
可以进行函数的复用

*/


//使用闭包创建行为的抽象

//一个代替假定计算的函数

use std::thread; 
use std::time::Duration;
fn simulated_expensive_calculation(intensity: u32) -> u32 { 
    println!("calculating slowly..."); 
    thread::sleep(Duration::from_secs(2)); 
    intensity
}

//获取用户的输入
fn main(){

    let simulated_user_specified_value = 10;
    let simulated_random_vnumber = 7;


}

//使用函数重构

fn generate_workout(intensity: u32, random_number: u32) {
    

    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25{
        println!(
            "Today, do {} pushps!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result);
    }else {
        if random_number == 3 {

            println!("Take a break today! Remember to stay hybrated!");
        }else {
            
            println!(
                "Today, run for {} mintues!",
                expensive_result
            );
        }
    }
}//所用代码块中都会调用expensive_result);包括没有显式使用的if块

//重构使用闭包存储函数

//定义一个闭包并存储到变量

//闭包就是允许我们在特定的位置存储代码到变量，同时在指定的位置调用他，不会使其到处被调用
//定义一个闭包

//首先定义

/*

一种是标注类型，另一种是不用标注类型
只有一行时闭包可以省略大括号
fn add_one_v1 (x: u32) -> u32 { x + 1 } 
let add_one_v2 = |x: u32| -> u32 { x + 1 }; 
let add_one_v3 = |x|{ x + 1 }; 
let add_one_v4 = |x|x + 1 ;

*/fn mian1(){

    let expensive_closure = |num: u32|{//给闭包增加可选的类型注解

        println!("calculating slow ...");
    
        thread::sleep(Duration::from_secs(2));
    
        intensity
    };
}
    


//闭包的类型推断和注解
/*
由于闭包出现的地方大都是特定的上下文，而非通用环境，所
以rust基本可以推断出闭包的类型，用户再注解反而和rust中的内容重合了

注意不能两次调用一个闭包传递值的类型不同
*/

fn two_closure(){

    let results = |x| x;
    let s = results(5);
    let n = results(String::from("123"));//闭包默认第一次使用赋予的变量类型为作用域类型

}

//使用带有泛型和Fn trait的闭包

//就之前的闭包处理的代码，仍然存在重复调用，部分代码所需的值是相同的但是，没有每次调用之后就存储结果，
//每次需要又重新计算

//使用实现闭包和泛型的结构体可以计算的同时，存储结果，并在需要时进行调用

//怎么将闭包转化为一个类型

//每个闭包有自己独特的匿名类型，与闭包函数签名无关

//每个闭包都实现了trait Fn 、 FnMut 或 FnOnce 中的一个
//能够当作类型使用

//实现Fn trait bound的闭包，必须指定参数和返回值的类型

//注意：函数也都实现了这三个 Fn trait。如果不需要捕获环境中的值，则可以使用实现了 Fn trait 的函数而不是闭包。

//可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会 在需要结果时执行闭包，
//并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。
//你可能见过这种模式被称 memoization 或 lazy evaluation （惰性求值）。
// Cacher 的缓存逻辑

struct Cacher<T>
    where T: Fn(u32) -> (u32)
    {
        calculation: T,
        value: Option<u32>
    }


impl<T> Cacher<T>
    where T : Fn(u32) -> u32
    {

        fn new(calculation: T) -> Cacher<T>{
            
            Cacher{

                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32{

            match self.value{

                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }
//引入更多参数提高必报的灵活性目标


//闭包会捕获其环境


//闭包访问其定义作用域内的变量
//访问变量和闭包匿名参数类型不同,则代码不能编译,类型推断不能通过
fn main2(){

    let x = 4;
    
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}


//闭包有三种方式捕获其环境
/*

闭包周围的作用域被称为其 环境，environment(及周围变量的作用域)

三种获取参数的方式：获取所有权，可变借 用和不可变借用。
这三种捕获值的方式被编码为如下三个 Fn trait：
1 FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。
1 为了 消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的
1 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。 
2 FnMut 获取可变的借用值所以可以改变其环境
3 Fn 从其环境获取不可变的借用值

同时

由于所有闭包都 可以被调用至少一次，所以所有闭包都实现了 FnOnce 。
那些并没有移动被捕获变量的所有权到闭包
内的闭包也实现了 FnMut ，而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn 。 
*/

//使用move关键字使闭包强制获取环境值的所有权

fn move_own(){

    let x = vec![1, 2, 3];

    let equal_to_x = move |z: Vec<i32>| z == x;

    println!("can`t used x {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));//比较传递值是否为true
    }


//使用迭代器处理序列


//迭代器:

//迭代器模式允许你对一个序列的项进行某些处理。迭代器（iterator）负责遍历序列中的每
//一项和决定 序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑

//迭代器是惰性的.为一个序列变量创建一个迭代器而不调用方法是没有用的

//rust创建一个迭代器

fn make_iterator(){

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
}

//迭代器对序列操作的优势:

/*
相对于使用索引,和其他方法去获取序列的值,迭代器提供了统一的方法操纵序列,减少了相同逻辑的代码,消除了潜在的混乱
提供了对多种不同的序列使用相同逻辑的灵活性，而不仅仅是像 vector 这样可索引的数据结构

*/

//Iterator trait和next方法

pub trait Iterator{

    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

//除去实现一个trait的

//新增内容是关联类型:  type Item和Self::Item
//Iterator trait 要求同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型


fn iterator_demonstartion(){

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

}

//使用iter生成的是不可变引用的迭代器,而使用next则要求迭代器本身的变的这样才能改变迭代器中用于记录序列位置的状态

//代码 消费（consume）了，或使用了迭代.
/*
1 iter 方法生成一个不可变引用 的迭代器。
2 获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter
3 迭代可变引用，则可以调用 iter_mut 而不是 iter 


*/

//消费迭代器的方法(消费适配器)

//Iterator trait 有一系列不同的由标准库提供默认实现的方法

//这些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 next 方法的原因。

//这些调用 next 方法的方法被称为 消费适配器（consuming adaptors）

//eg
fn itemator_sum() {
        
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(6, total);
}
//产生其他迭代器的方法

// 迭代器适配器（iterator adaptors），他们允许我们将 当前迭代器变为不同类型的迭代器。

//注意:
//可以链式调用多个迭代器适配器。
//不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

//迭代适配器得配合消费适配器使用

//eg

fn conmuse_iter(){

    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}


//使用闭包获取环境

#[derive(Debug, PartialEq)]//PartialEq: 这个特征允许你比较两个结构体是否相等（使用 == 和 != 操作符）。


struct Shoe{

    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()//filter迭代适配器和collect()消费适配器同时作用,filter使得闭包返回值为true的才能被collect使用转化为集合
    
}


#[test]

fn filters_by_size() {
    
    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker")},
        Shoe {size: 13, style: String::from("sandal")},
        Shoe {size: 10, style: String::from("boot")}

    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {size: 10, style: String::from("sneaker")},
            Shoe { size: 10, style: String::from("boot")},
        ]
    );
}


struct Shoe{

    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()//filter迭代适配器和collect()消费适配器同时作用,filter使得闭包返回值为true的才能被collect使用转化为集合
    
}


#[test]

fn filters_by_size() {
    
    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker")},
        Shoe {size: 13, style: String::from("sandal")},
        Shoe {size: 10, style: String::from("boot")}

    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {size: 10, style: String::from("sneaker")},
            Shoe { size: 10, style: String::from("boot")},
        ]
    );
}
//实现 Iterator trait 来创建自定义迭代器


struct Counter{

    count: u32,
}

impl Counter {
    
    fn new() -> Counter {
        
        Counter{

            count: 0
        }
    }
}

impl Iterator for Counter{

    type Item = u32;//首先实现关联类型

    fn next(&mut self) -> Option<Self::Item> {//指定next方法来消费迭代器.

        self.count += 1;
        if self.count < 6 {
            
            Some(self.count)
        }else {
            None
        }
    }
}

//实现 Iterator trait 来创建自定义迭代器
#[test]

fn using_other_iterstor_trait_methods(){

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    print!("{:?}", sum);
    assert_eq!(18, sum);
}


//改进12章I/O项目


