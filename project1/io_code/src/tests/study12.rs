//chapter 12



//I/O项目构：构建一个命令行工具（grep）

//12-01接受一个命令参数

//可以使用rust标准库实现

fn main1() {
    let args: Vec<String> = env::args().collect();
    //std::env::args返回一个迭代器（iterator）生成一系列值并被collect转换为一个集合
    //无效的unicode字符args不会接受并且会直接panic

    //println!("{:?}", args);//第一个参数是程序的名称和路径
    
    let query = &args[1];//参数保存到变量

    let filename = &args[2];

    println!("Searching for {}", query);

    println!("In file {}", filename);
}

//12-02 reading a file (读取文件)

fn main2() {
    let args: Vec<String> = env::args().collect();

    //println!("{:?}", args);
    
    let query = &args[1];

    let filename = &args[2];

    println!("Searching for {}", query);
    //snip

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)//读取filename存取文件名，并读取文件内容，转化为String存入变量中
        .expect("Something went wrong reading the file");//错误处理d

    println!("With text: \n{}", contents);

}  

//12-03 improving error haandling and modularity (重构改进模块性和错误处理)

//二进制项目的关注分离度

//从mian函数中分离出程序的指导型原则

//不仅函数要进行分离重构，包括要对使用中的变量进行组织使其具备结构性，对于错
//误处理也得使其集中到一处，而且具体使得错误处理的原因得以输出，用户得到有用的信息
//不能一直使用expect处理错误，没有足够的参数不会产生有效的错误输出信息，将错误处理
//合到一处，便于修改，使打印的信息是对用户有用的

/*

分离指导性原则：


将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。 
当命令行解析逻辑比较小时，可以保留在 main.rs 中。
当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。


检测分离是否成功的标准：

使用参数值调用命令行解析逻辑 
设置任何其他的配置 
调用 lib.rs 中的 run 函数
如果 run 返回错误，则处理这个错误


目的：main函数负责运行，lib.rs负责任务的逻辑

*/


//提取参数解析器

fn config_prase(args: &[String]) -> (&str, &str) {

    let query = &args[1];
    let filename = &args[2];

    (query, filename)
    
}//注意每次进行函数重构后都要进行测试，验证提前解决可能存在的问题

//组合配置值

/*
//基本类型偏执

是用结构体配置使用值

*/

struct Config {

    pub query: String,
    pub filename: String,
}
fn config_prase(args: &[String]) -> Config {

    let query = args[1].clone();
    let filename = args[2].clone();//对值进行复制，会花费比引用更多的时间和内存，
    //但是使代码简洁直白方便使用

    //对于是否使用clone，一般是尽量避免

    Config { query, filename }//返回一个结构体，意料之外

    
    
}

    
//创建一个Config 的构造函数，
//构造函数就是用于创建一个结构体实例的，与方法不同在于其不使用&self，传递自身参数
//我们可以将 parse_config 从一个 普通函数变为一个叫做 new 的与结构体关联的函数
//使之和标准库的习惯一样，使之便于使用


//1修复错误处理

//1.1改善错误信息

impl Config{

    fn new(args: &[String]) -> Config {

        if args.len() > 3{

            panic!("not enough arguments");//增加一个参数检查
        }
        let query = args[1].clone();

        let filename = args[2].clone();

        Config{query, filename}
        
    }
}
//1.2从new是、中返回Result而不是panic！

impl Config{

    fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() > 3{

            return Err("not enough arguments");//返回错误信息到main
        }
        let query = args[1].clone();

        let filename = args[2].clone();

        Ok(Config{query, filename})
        //返回一个 Err 值，这就允许 main 函数处理 new 函数返回的
        // Result 值并 在出现错误的情况更明确的结束进程。
        
    }
}


//Config::new调用并处理错误


//非零错误码（惯用信号）退出命令行工具
//panic就是执行这样的操作使程序崩溃

fn main() {
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);
    
    //let config = config_prase(&args);
    //let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);//新建Config实例失败就用错误码退出
        process::exit(100000);//使用错误码
    });
}

//unwrap_or_else是Result<T, E>类型上的方法，当成功会获取OK中的值作为返回值，而失败方法时使用闭包处理
//会将Err的值传递给位于闭包两条竖线中间的参数err，供闭包中的代码使用

//从main中提取run逻辑

//提取run函数： 的函数来存放目前 main 函数中不属于设置配置或处理错误的所 有逻辑。
//一旦完成这些， main 函数将简明得足以通过观察来验证，而我们将能够为所有其他逻辑编
//写测试

fn run(config: Config) {

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
    
}

//从run函数中返回错误

//运行部分的逻辑已经成为一个函数，错误处理


use std::error::Error;    
fn run(config: Config) -> Result<(), Box< dyn Error>>{
    //使用了 trait 对象 Box<dyn Error> （在开头使用了 use 语句将 std::error::Error 引入作用域）
    //使函数返回值实现Error trait，能灵活的返回各种错误类型便于的修改错误
    

    //let contents = fs::read_to_string(config.filename)
        //.expect("Something went wrong reading the file");
    let contents = fs::read_to_string(config.filename)?;
    // ? 。不同于遇到错误就 panic! ， ? 会从 函数中返回错误值并让调用者来处理它
    println!("With text: \n{}", contents);
    
    Ok(())

    //返回ok（（）），返回装有（）的返回值，本身是使用Result 的副作用，不具备意义
}


//将代码拆分到库crate

//拆分时注意将代码设置其访问权限（pub 还是 pri），引用时注意路径的指定


//12-04 testing the librarys functionality 采取测试驱动开发完善库的功能

//使用TDD测试驱动开发（Test Driver Development）进行软件测试开发

/*
TDD实现步骤：

1. 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的。 
2. 编写或修改足够的代码来使新的测试通过。
3. 重构刚刚增加或修改的代码，并确保测试仍然能通过。 
4. 从步骤 1 开始重复！

TDD优点：

1有助于驱动代码的设计
2提高测试覆盖率


*/

//编写失败的测试


// 
pub fn search <'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    //使返回参数的生命周期和被引用数据中较短，保持一致
    //显式的告诉哪个参数是我们真正想用的
    //否则rust不知道我们

vec![]
}


//使用lines（）方法遍历每一行
//同理会返回一个迭代器，生成一系列值


pub fn search <'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
        
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){//用查询字符串搜索每一行

            results.push(line);//存储匹配行
        }

    }
    
    results//返回匹配值
}
//12-05处理环境变量，working with enviroment varlables

//编写一个对大小写不敏感的search函数

//编写一个大小写不敏感 search 函数的失败测试


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let query = query.to_lowercase();//此时query不是引用而是真实存在的值。to_lowercase是创建一个新数据
    let mut results = Vec::new();
    
    for line in contents.lines(){

        if line.to_lowercase().contains(&query){

            results.push(line);
        }
    }

    results
}


pub fn run(config: Config) -> Result<(), Box< dyn Error>>{

    //let contents = fs::read_to_string(config.filename)
        //.expect("Something went wrong reading the file");
    let contents = fs::read_to_string(config.filename)?;
    //println!("With text: \n{}", contents);
    for line in search(&config.query, &contents){

        println!("{}", line);
    }
    Ok(())
}

//12-06 -writing-to-stderr-instead-of-stdout 将错误信息输出到标准错误而不是标准输出

//将错误信息打印到标准错误，同时将标准输出重定向到文件中（未设置之前之恰相反）
//重定向标准错误输出到文件

//cargo run > output.txt



fn main() { 
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| { 
        eprintln!("Problem parsing arguments: {}", err);//将错误打印到标准错误,并输出到屏幕上
        process::exit(1);//
    });
    if let Err(e) = minigrep::run(config) { 
        eprintln!("Application error: {}", e); 
        process::exit(1);
    }
}