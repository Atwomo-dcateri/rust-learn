//chapter 11 Atuo testing

//测试用于能正常编译运行源程序，输出结果是否和预期的一致


pub mod front_of_auto{

    pub fn s1() {
        
        
    }
}
//属性（attribute）是关于 Rust 代码片段的元数据；第五章中结构体中用到的 derive 属性就是一个例子
//将一个函数转化为测试函数：在函数前加上#test就行。Cargo testRust 会构建 一个测试执行程序用
//来调用标记了 test 属性的函数，并报告每一个测试是通过还是失败。


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
    



}