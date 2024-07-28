pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
mod front_of_house { //mod定义一个模块，名字为front_of_house
//这样就形成了Crate根为主的模块树
    mod hosting { //模块内同样的可以定义模块
        fn add_to_waitlist() {}//树中互为兄弟的结点被定义在同一模块中
        fn seat_at_table() {} //
    }
   /* 如果一 个模块 A 被包含在模块 B 中，我们将模块 A 称为模块 B 的 子（child），模块 B 则是模块 A 的 父
（parent）*/ 
//整个模块树都植根于名为 crate 的隐式模块下。没有通过显式的定义去表现模块树
    mod serving { 
        fn take_order() {} 
        fn server_order() {} 
        fn take_payment() {}
        }
}

//路径用于引用模块树中的顶（用于调用模块中的函数）

//路径的形式

/*
绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。 
相对路径（relative path）从当前模块开始，以 self 、 super 或当前模块的标识符开头。

注意：绝对路径和相对路径都后跟一个或多个由双冒号（ :: ）分割的标识符。
*/

//eg：

mod front_of_house1 { 
    pub mod hosting { 
        pub fn add_to_waitlist() {
            println!("hello world!");
        }
    } 
}

pub fn eat_at_restaurant() { 
    // Absolute path 
    crate::front_of_house1::hosting::add_to_waitlist();
    // Relative path 
    front_of_house1::hosting::add_to_waitlist();
}

fn main(){

    eat_at_restaurant();

}

//模块便于组织代码
//定义了 Rust 的 私有性边界（privacy boundary）：这条界 线不允许外
//部代码了解、调用和依赖被封装的实现细节
//子模块和父模块的关系类似于子类和父类，一个Crate就是一棵树，而节点之间的关系类似于类之间的关系

//Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的
//父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项

//私有就是餐馆后台（服务实现的细节），公共就是餐馆前为你提供的服务

//pub用于创造公有项，使上级模块可以引用下级模块，而不能访问模块的具体内容

//super作用类似于文件系统中../
//便于从当前路径根路径中调用其他模块，移动其他模块到另一模块中，只需要更新少量的代码


fn serve_order() {

    
} 

mod back_of_house {
    fn fix_incorrect_order() { 
        cook_order(); 
        super::serve_order();
    } 
    fn cook_order() {}
}
//模块中可以定义公有的结构体和枚举类型，但是其内容还是私有的，得根据需求进行内容的公有化


mod back_of_house1 {
    #[derive(Debug)]
    pub struct Breakfast{

        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{

            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

use back_of_house1::Breakfast;

pub fn eat_at_restaurant1(){

    let mut meal = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I `d like {} toast please", meal.toast,);
    println!("{:?}", meal);
}


//设计公有枚举
mod back_of_house2{

    pub enum Appitzer{
f
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2(){

    let order1 = back_of_house2::Appitzer::Soup;

    let order2 = back_of_house2::Appitzer::Soup;


}

//use将名称引入作用域
// 和路径类似于在文件系统中创建软连接
//简化了引用模块的复杂度，简化了代码

//不直接引用函数，直接引用不符合习惯，调用函数时指定父模块，这样可
//以清晰 地表明函数不是在本地定义的，同时使完整路径的重复度最小化

//习惯法引用函数

use std::collections::HashMap; //没有直接引用函数
fn main() { 
    let mut map = HashMap::new(); 
    map.insert(1, 2);
}

//同名类型引入同一作用域可以使用as对名称重新命名

use std::fmt::Result; 
use std::io::Result as IoResult;
fn function1() -> Result { // --snip--
}
fn function2() -> IoResult<()> {
    //不影响正常使用

// --snip--
}    

//pub use公有调用（重导出技术），引用模块的技术
//使得不同作用域的代码都可以使用这个模块
//与use相比使用 pub use ，我们可以使用一种结构编写代码，却将不同的结构形式暴露出来
//使库井井有条，方便开发这个库的程序员和调用这个库的程序员之间组织起来
//理解库代码的本意
mod front_of_house { 
    pub mod hosting { 
        pub fn add_to_waitlist() {}
    }
} 

pub use crate::front_of_house::hosting;//扩张了作用域
pub fn eat_at_restaurant() { 
    hosting::add_to_waitlist(); 
    hosting::add_to_waitlist(); 
    hosting::add_to_waitlist();
}

//引用外部包

/*
在cargo。toml添加依赖，后使用use 去引用模块到作用域中

除标准库外的外部依赖都得在cargo.toml中备案后
才能使用use引用到项目包中的作用域
[dependencies] 

rand = "0.5.5"

*/

//使用嵌套路径简化use语句
/*


use std::cmp::Ordering; 
use std::io;


use std::{cmp::Ordering, io};//注意，相同部分联合，不同部分嵌入{}中


*/

//使用glob运算符将所有共有定义引入作用域

/*

Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的。
glob 运算符有时也用于 prelude 模式
glob 运算符经常用于测试模块 tests 中
eg：

use std::collections::*;

*/


//将模块分割成不同的文件
//当模块变得更大时，你可能想要将它们 的定义移动到单独的文件中，从而使代码更容易阅读。

//模块分割后不会影响use语句，还能像之前那样调用模块



//eg：


//在 mod front_of_house 后使用分号，而不是代码块，这将告诉 Rust 在另一个与模块同名的文件中 加载模块的内容。
mod front_of_house;//表示模块已经被放入名为：front_of_house.rs/中
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() { 
    hosting::add_to_waitlist(); 
    hosting::add_to_waitlist(); 
    hosting::add_to_waitlist();
}
//进一步模块分割
// src/front_of_house 目录和一个包含 hosting 模块定义的。即分割到一个可以调用的最小项（一个模块只有一个函数）
//对于模块树没有影响，之前的代码调用也没有影响
//use语句不必修改



