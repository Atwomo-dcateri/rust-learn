//chapter 9 error-handling
/*

rust的风险把控：Rust要求你承认出错的可能性，并在编译代码之前就采取行动。这些要求使得程序更为健壮，它
们确保了你会在将代码部署到生产环境之前就发现错误并正确地处理它们rust中的程序运行异常记为错误.

rust中的程序运行异常记为错误：
rust中没有异常
错误组合成两个主要类别：可恢复错误（recoverable）和 不可恢复错误（unrecoverable）。


可恢复错误：通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件

记为：Result<T, E>

不可恢复错误：通常是 bug 的同义词，比如尝试访问超过数组结尾的位置

记为：不可恢复(遇到错误时停止程序执行)错误 panic!
*/

#[allow(dead_code)]
#[warn(unused_variables)]

pub mod front_of_error_handling{


//对应panic！的栈终止和展开
//当出现 panic 时，程序默认会开始 展开（unwinding）
//展开：这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作
//终止：abort，不清理数据就退出程序，程序所使用的内存需要由操作系统来清理

//配置展开还是终止
//默认情况下是展开
/*
        配置终止：
        需要项目的最终二进制文件越小越好，panic 时通过在 Cargo.toml 的 [profile] 部分增加
        panic = 'abort'

    eg：配置release中的终止

    [profile.release]
    panic = 'abort'

*/

    pub fn p_1_abort(){

        //panic!出错的地方即可能是我们代码，也可能是我们调用代码中的panic！（panic！是个宏）

        panic!("crash and burn");
    }

    //使用panic！的backtrace
    //backstrace 寻找代码具体出错的地方

    pub fn p_2_backtrase(){

        let v = vec![1, 2, 3];

        v[99];
        //缓冲区溢出（buffer overread）：
        //对于其他像 C 这样语言会尝试直接提供所要求的值，即便这可能不是你期望的：你会得到任何
        //对应 vector 中这个元素的内存位置的值，甚至是这些内存并不属于 vector 的情况
        //形成安全漏洞


        //解读backtrase
        //设置 RUST_BACKTRACE (不能为零)环境变量来得到一个 backtrace:backtrace 是一
        //个执行到目前位置所有被调用的函数的列表

        //阅读backtrace 的关键是从头开始读直到发现你编写的文件，就是问题的发源地
    
        //这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码

        //注意为了看到这些信息，必须开启--debug ，但是--debug在 cargo run执行时默认打开

        
    }

    //result与可恢复错误
    /*
    
    enum Result<T, E>{

        Ok(T),

        T 和 E 是泛型类型参数
        Err(E),
        T 代表成功时返回的 Ok成员中的数据的类型
        E 代表失败时返回的 Err 成员中的错误的类型
        可以将 Result 类型和标准库中为其定义的函数用于很多不同的场景，这些情况中
        需要返回的成功值和失败值可能会各不相同

    }
    
     */


    use std::error::Error;
    //use std::ptr::DynMetadata;
    use std::{error, fs::File};

    pub fn p_3_result() {

        let f = File::open("hello.txt");
        
        println!("f: {:?}", f);
    }
//prelude是一个包含常用类型和函数的集合，在编写rust程序时自动导入
//标准prelude

//不需要导入就能直接使用的宏和部分基本类型：Option， Result， Vec， String，Box

//自定义的Prelude

    pub fn p_4_Err(){

        let f = File::open("hello");

        let f =  match f {
            
            Ok((file)) => file,
            Err((error)) => {
                panic!("Problem opening the file: {:?}", error)
            },
        };
    }

    //匹配不同的错误原因，采取不同的行为

    use std::io::ErrorKind;

    pub fn p_5_errorkind() {

        let f = File::open("hello.txt");
        
        let f = match f{

            Ok(file) => file,
            Err(error) => match  error.kind() {

                ErrorKind::NotFound => match File::create("C:/User/hp/Desktop/hello.txt") {

                    Ok(fc) => fc,
                    Err(e) => panic!("Probleem creating the file: {:?}", e),
                    
                },

                other_error => panic!("Problem opening the file: {:?}", other_error),
                
            },
        };
        
    }

    //使用闭包（closure）实现匹配不同错误处理

    pub fn p_5_1_errorkind() {
        
        let f = File::open("hello.txt").unwrap_or_else(|error|{
            if error.kind() == ErrorKind::NotFound{

                File::create("hello.txt").unwrap_or_else(|error|{

                    panic!("Problem creating the file: {:?}", error);
                })
            }else {
                
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    //失败时panic的简写: unwrap和expext

    //使用Result类型定义的方法替代冗长的match语句

    //unwrap（）：如果 Result 值是成员 Ok ， unwrap 会返回 Ok 中的值。如果 Result 是成员
    //Err ， unwrap 会为我们调用 panic! 。

    //expect（）与unwrap不同在于它允许选择panic！的错误信息

    pub fn p_6_wrap() {

        //let f = File::open("hello1.txt").unwrap();

        let f = File::open("hello1.txt").expect("Failed to open hello1.txt");
    }

    use std::{fs, io};
    use std::io::Read;
    //传播错误，即将失败后的信息返回给调用者，而不是使用panic！使程序崩溃
    pub fn p_7_read_username_from_file() -> Result<String, io::Error> {// Result<String, io::Error>指定返回变量的类型

        let f = File::open("hello.txt");

        let mut f = match f{

            Ok(file) => file,//file是指文件句柄
            Err(e) => return Err(e),//不会再调用panic！，而是直接返回一个值
        };

        let mut s = String::new();

        match f.read_to_string(&mut s){

            Ok(_) => Ok(s),
            Err(e) => Err(e),//没有显式的调用return, 这是最后的表达式
        }
        
    }

    //传播错误的简写：？运算符 
    //？消除了大量代码

    pub fn p_8_read_user_from_file() -> Result<String, io::Error> {

        let mut f = File::open("hello3.txt")?;//这问号就相当于match中对Result<T, E>的处理

        let mut s = String::new();

        f.read_to_string(&mut s)?;//与match相比？运算符会将返回的错误通过from：：trait转换为返回值规定的类型，
                                        //不能体现所有的的错误原因，但是错误类型的转换由？运算符自动完成

        println!("s: {:?}", s);

        Ok(s)
    }

    pub fn p_8_1_read_username_from_file() -> Result<String, io::Error> {
        
        let mut s = String::new();

        File::open("hello3.txt")?.read_to_string(&mut s)?;//工程学的写法（ergonomic）

        Ok(s)
    }

    pub fn read_8_2_read_username_read_from_file() -> Result<String, io::Error> {
        
        fs::read_to_string("hello.txt")
    }

    //?运算符可被用于返回Result的函数
    //或者是实现std::ops::try类型的函数
    pub fn p_9_open() {
        
        //let f = File::open("hello90.txt")?;
    }

    //use std::error::Error;

    pub fn mian1() -> Result<(), Box<dyn Error>> {
        
        let f = File::open("hello.txt")?;

        Ok(())
    }


    //panic!不是panic！
    //如果代码 panic，就没有恢复的可能。代表bug
    //你可以选择对任何错误场景都调用 panic! ，不管是否有可能恢复，不过这样就是你代替调用者决定了这是不可恢复的
    //使调用者有权去处理不同的错误的原因。调用panic！同时意味着可能将可恢复的错误变为可恢复的错误，
    //因此此返回 Result是定义可能会失败的函数的一个好的默认选择。


    //示例和代码原型和测试都适合panic！
    //在示例中他可能是代码不怎么的明确，增强了代码的健壮性

    //unwrap 和 expect 方法在原型设计时非常方便。当我们准备好让程序更加健壮时，它们会在代码中留下清晰的标记

    //在测试时对代码中方法调用的代码进行统一的panic！标记ERror，使用expect，unwrap


    //对于错误处理应该存在于任何可能出错的代码中，哪怕你认为逻辑上不可能出错的代码也是可能出错的
    //eg：遂于字符串不是硬编码进入程序中，而是未知随程序的运行大小不一，长度不断变化

    use std::net::IpAddr;

    pub fn p_10_ipaddr() {
        
        let home: IpAddr ="127.0.0.1".parse().unwrap();//如果此时IP地址使用户输入的该怎么办
    }


    //错误处理的指导原则

    /*
    
    错误处理指导原则
在当有可能会导致有害状态的情况下建议使用 panic! —— 在这里，有害状态是指当一些假设、保
证、协议或不可变性被打破的状态，例如无效的值、自相矛盾的值或者被传递了不存在的值 —— 外
加如下几种情况：
有害状态并不包含 预期 会偶尔发生的错误
在此之后代码的运行依赖于不处于这种有害状态
当没有可行的手段来将有害状态信息编码进所使用的类型中的情况


//什么时候使用panic！
别人调用你的代码并传递了一个没有意义的值，最好的情况也许就是 panic! 并警告使用你的库
的人他的代码中有 bug 以便他能在开发时就修复它。类似的，如果你正在调用不受你控制的外部代
码，并且它返回了一个你无法修复的无效状态，那么 panic! 往往是合适的

//当错误预期是可知的Result是比panic！更合适的。可以对这些错误传播进行处理而不是退出程序

//对于无效的输入直接panic，以免造成安全漏洞，这是一种函数契约（contracts）：函数的契约，尤其
是当违反它会造成 panic 的契约，应该在函数的 API 文档中得到解释。


//可以利用 Rust 的类型系统（以及编译器的类型检查）为你进行很多检查
减少函数中复杂冗长错误检查代码

    
     */

    //创建自定义类型进行有效性验证

    /*
    
    loop {
        // --snip--
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,//在每个函数中都有这样的检查将是非常冗余的（并可
能潜在的影响性能
        };
    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }
    match guess.cmp(&secret_number) {
    // --snip--
    }
    

     */

    //使用结构体在创建实例之前先对输入进行检测以免，保证输入的有效性同时减少对可能的对潜在性能的影响

    pub struct Guess {

        value: i32,
    }

    impl Guess{

        pub fn new(value: i32) -> Guess{

            if value < 1 || value > 100{

                panic!("Guess value must be between 1 and 100, got {}", value);
            }

            Guess{

                value
            }
        
       }

       pub fn value(&self) -> i32 {

            self.value
       }
    }

    pub fn p_11_Guess() {
        
        let s = Guess::new(322);//违反了Gesso：：new（）函数遵循的契约（contract）
                                    //直接调用panic!,返回backtrase,标记处出错的地方

    }

    /*
    第9章总结：

    Rust 的错误处理功能被设计为帮助你编写更加健壮的代码。 panic! 宏代表一个程序无法处理的状
态，并停止执行而不是使用无效或不正确的值继续处理。Rust 类型系统的 Result 枚举代表操作可
能会在一种可以恢复的情况下失败。可以使用 Result 来告诉代码调用者他需要处理潜在的成功或失
败。在适当的场景使用 panic! 和 Result 将会使你的代码在面对不可避免的错误时显得更加可
靠。
    
    
     */

}

