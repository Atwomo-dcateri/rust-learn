//chapter 11 Atuo testing

//测试用于能正常编译运行源程序，输出结果是否和预期的一致


pub mod front_of_auto{

    pub fn s1() {
        
        
    }
}
//属性（attribute）是关于 Rust 代码片段的元数据；第五章中结构体中用到的 derive 属性就是一个例子
//将一个函数转化为测试函数：在函数前加上#test就行。Cargo testRust 会构建 一个测试执行程序用
//来调用标记了 test 属性的函数，并报告每一个测试是通过还是失败。
pub fn add_two(value: i32) -> i32 {
    
    value + 2
}

#[cfg(test)]

mod tests {

    #[test]
    /*fn it_test() {
        
        assert_eq!(2 + 2, 4 );
    }*/

    fn exploration() {
        
        assert_eq!(2 + 2, 4 );
    }

    #[test]
    fn another(){//一个成功和一个失败的测试，

        panic!("Make this test fail");
    }


    //调用宏可以更好的帮助我们测试


    //使用assert！检查测试结果

    #[derive(Debug)]
    struct Rectangle{

        width: u32,
        height: u32,
    }

    impl Rectangle {
        
        fn can_hold(&self, other: &Rectangle) -> bool{

            self.width > other.width && self.height > other.height
        }
    }
    use std::result;

    use super::*;
    #[test]
    fn large_can_hold_smaller(){

        let lager = Rectangle{

            width: 10,
            height:8,
        };

        let smaller = Rectangle{

            width: 5,
            height: 9,
        };

        assert!(!lager.can_hold(&smaller));
    }

    pub fn add_two(a: i32) -> i32{

        a + 2
    }


    //使用assert_eq!和assert_ne!测试
    //assert_eq,比较left和right值相等为true，来测试函数
    #[test]
    fn it_adds_two(){

        assert_eq!(4, add_two(3));
    }

    //assert_ne!,要求left和right不相等为true，测试函数
    #[test]
    fn it_adds_two1(){

        assert_ne!(4, add_two(3));
    }

    //我们不确 定值 会 是什么，不过能确定值绝对 不会 是什么的时候，这个宏最有用处

    //对于自定义结构体和枚举必须实现PartialEq和Debug trait
    //可以使用#[derive(PartialEq, Debug)]为结构体和枚举添加这些属性（实现trait）



    //自定义失败信息
    //你也可以向 assert! 、 assert_eq! 和 assert_ne! 
    //宏传递一个可选的失败信息参数，可以在测试 失败时将自定义失败信息一同打印出来
    //任何在 assert! 的一个必需参数和 assert_eq! 和 assert_ne! 的两个必需参数
    //之后指定的参数都会传递给 format! 宏

    //传递值中加入一个占位字符”{}“，方便记录断言意义，测试失败时理解代码出了什么问题


    pub fn greeting(name: &str) -> String {
        
        //format!("Hello, {}!", name)

        format!("Hello")
    }
    
    #[test]
    fn greeting_contains_name(){

        let result = greeting("Carol");

        assert!(result.contains("Carol"));
    }
    //自定义一个失败信息参数，打印出调用函数的值,告诉我们更多有用的信息
    #[test]
    fn greeting_contains_name1(){

        let result = greeting("Catol");

        assert!(
            result.contains("Carol"), 
            "Greeting did not contain name, value was '{}'",result
        );
    }
    //使用should panic 检查panic


    pub struct Guess{

        value: i32,
    }

    impl Guess{

        fn new(value: i32) -> Guess{

            if value < 1 || value > 100{

                panic!("Guess must be between 1 and 100, got {}", value);
        
            }
            Guess{
                
                value
            }
        }
        fn new1(value: i32) -> Guess{

            if value < 1 {

                panic!("Guess must be between 1 and 100, got {}", value);
        
            }
            Guess{
                
                value
            }
        }
        fn new2(value: i32) -> Guess{

            if value < 1 {

                panic!("Guess must be between 1 and 100, got {}", value);
        
            }
            if value > 100{


                panic!("Guess must be between 1 and 100, got {}", value);
            }
            Guess{
                
                value
            }
        }
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]

    fn greater_than_100(){
        
        //Guess::new(200);
        //Guess::new1(200);
        Guess::new2(200);
    }

    //给should_panic增加参数expect可以使其更加的精确，去除其中模糊不清和不符合预期的部分

    //使用Result<T, E>y用于测试

    #[test]
    fn it_works() -> Result<(), String> {

        if 2 + 2 == 4 {

            Ok(())
        }else {
            
            Err(String::from("Two plus two does not equal four"))
        }
    }
    //使用result<T, E>适合任何运算符会返回Err成员的测试
    //对使用Result<T, E>的测试不能使用should_panic,因为两者之间是矛盾的，
    //panic会直接退出程序，不会将错误的处置权提供给调用者，
    //而Result泛型返回错误成员，让用户选择是否处理这个错误


    //控制测试如何运行

    //可以使用命令行参数控制cargo test

    //生成的二进制文件的默认行为是并行的运行所有测试，并截获测试运行过程
    //中产生的输出，阻 止他们被显示出来，使得阅读测试结果相关的内容变得更容易

    //使用命令行参数可向运行的二进制文件传递参数

    //并行测试和连续测试

    //rust默认是并行测试
    //注意并行测试时，各项测试之间不能有依赖，包括对任何共享的状态，公共环境

    //连续测试：每个测试逐个运行不存在测试依赖问题，但是会花费更多的时间。
    //得使用命令行参数显式的指出
    //eg ： cargo test --- --test-threads=1

    //显示函数输出

    //对于成功的测试，测试函数的标准输出内容会被Rust测试库截取，不会打印输出
    //对于失败的测试，测试函数的标准输出内容不会被Rust测试库截取，直接打印输出，并输出其他错误信息

    //eg

    //#[cfg(test)]

    //mod tests

    //use super::*;

    fn prints_and_rerurns_10(a: i32) -> i32 {
        
        println!("I got the value {}", a);
        10
    }

    #[test]
    fn this_test_will_pass() {

        let value = prints_and_rerurns_10(10);

        assert_eq!(10, value);
        
    }

    #[test]

    fn this_test_fail() {
        
        let value = prints_and_rerurns_10(5);

        assert_eq!(value, 5);
    }
    //可以通过使用命令行参数，使成功通过测试也打印出本身的标准输出

    //eg：cargo test -- --nocapture

    //指定名字运行部分测试

    //eg： cargo test + test_function_name

    //过滤运行多个测试

    //可以运行包含指定字符串的所有测试

    //eg：cargo test add（运行所有包含add的测试，如果模块内包含add，整个模块的测试都会被运行）


    //忽略某些测试
    //同给需要忽略的测试添加：#[ignore]属性,则该测试不会被执行
    //eg:

    #[test]

    #[ignore]

    fn ignore_this_test() {
        
        let five = add_two(3);

        assert_eq!(five, 5);

    }
    //注意可以使用参数运行ignore测试
    //eg ：cargo test -- --ignored



    //11-3 测试组织结构

    //单元测试和集成测试

    /*
    
    单元测试（unit tests）
    
    单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口

    集成测试（integration tests）

    集成测试对于你的库来说则完全是外部的
    它们与其他外部代码一样，通过相同的方式使用你的代
    码，只测试公有接口而且每个测试都有可能会测试多个模块

     */

    //单元测试
    /*

    单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便
    于快速而准确的某个单元 的代码功能是否符合预期。单元测试与他们要测试
    的代码共同存放在位于 src 目录下相同的文件中。
    规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块
    
     */


    //测试模块和#[cfg(test)]

    /*
    1测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行
    1测试代码，而在运 行 cargo build 时不这么做
    2使用#[cfg(test)]构建库的时候可以节省编译时间，并且因为它们并没有包含测试，所以能减少
    2编译产生的文件的大小。
    3集成测试因为位于另一个文件夹，所以它们 并不需要 #[cfg(test)] 注解。
    4单元测试位于与源码相同的文件中，所以你需要使用 #[cfg(test)] 来指定
    4他们不应该被包含进编译结果中
    
    注意：rust只会在主动使用cargo test才会编译测试代码
     */

    //测试私有函数
    //Rust 的私有性规则允许你测试私有函数。
    //对于其他语言这是很难实现的
    use super::*;

    #[test]
    fn test_internal_add_two() {

        let six = internal_add_two(4);

        assert_eq!(six, 6);
        
    }

}

fn internal_add_two(a: i32) -> i32{

    a + 2
}
//二进制crate的集成测试

/*

只有库 crate 才会向其他 crate 暴露 了可供调用和使用的函数；二进制 crate 只意在单独运行

为什么 Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？

因为通过这种 结构，集成测试 就可以 通过 extern crate 测试库 
crate 中的主要功能了，而如果这些重要的功能
没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。

*/

//第11章总结

/*

1Rust 的测试功能提供了一个确保即使你改变了函数的实现方式，也能继续以期望的方式运行的途径。
2单元测试独立地验证库的不同部分，也能够测试私有函数实现细节。
3集成测试则检查多个部分是否能 结合起来正确地工作，并像其他外部代码那样测试库的公有 API。
3即使 Rust 的类型系统和所有权规则
4可以帮助避免一些 bug，不过测试对于减少代码中不符合期望行为的逻辑 bug 仍然是很重要的

*/