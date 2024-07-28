// ************************************************************************************************
// Vendor           : INWA
// Author           : ning jiu
// Filename         : lib 
// Date Created     : 2024.07.24 
// Version          : V1.0
// ************************************************************************************************


//泛型：编程语言都有高效处理重复概念的工具
/*

1.编程语言都有高效处理重复概念的工具
2.泛型是具体类型或其他属性的抽象替代
3.我们可以表达泛型的属性，比如他们的行为或如何与其他泛型相
  关联，而不需要在编写和编译代码时知道他们在这里实际上代表什么
*/
//trait
/*
    这是一个定义泛型行为的方法。
    trait 可以与泛型结合来将泛型限制为拥有特定行为的类型，而不是任意类型

*/


//生命周期（lifetimes）

/*
    它是一类允许我们向编译器提供引用如何相互关联的泛型。
    Rust 的生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性。

*/
//提取函数来减少重复
//不使用泛型的处理重复的技术：提取一个函数



pub mod front_of_gen_tra_lif {
    use core::str;
    use std::{fmt::{format, Debug, Display}, result};


    pub fn g_1_max() {
        
        let number_list = vec![1, 2, 45, 56, 87];

        let mut largest = number_list[0];

        for number in number_list{

            if number > largest{

                largest = number;
            }
        }

        println!("The largest of number is {}", largest);
    }

    pub fn g_2_max_twolist(){

        let number_list = vec![34, 50, 25, 100, 65];
        let mut largest = number_list[0];
        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        println!("The largest number is {}", largest);
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let mut largest = number_list[0];
        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        println!("The largest number is {}", largest);
    }

    pub fn largest(list: &[i32]) -> i32{

        let mut largest = list[0];

        for &number in list{

            if largest < number{

                largest = number;
            }
        }

        largest
    }

    pub fn g_2_1_tmax(){

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let result = largest(&number_list);
        println!("The largest number is {}", result);
        
    }
    pub fn g_2_max_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest

    }
    //使用泛型定义参数，方法，返回值，结构体的项
    
    
    //在函数签名中指定参数和返回值的类型的地方，会改用泛型来表示。采用这种技术，
    //使得代码适应性更强，从而为函数的调用者提供更多的功能，同时也避免了代码的重复

    //定义参数和返回值的参数的泛型类型
    //T这个泛型没有明确的指定是哪些类型的类型

    //没有指定的泛型的范围
    pub fn largest1<T: PartialOrd + Copy>(list: &[T]) -> T{

        let mut largest = list[0];

        for &item in list.iter() {
            
            if item > largest {
                
                largest = item;
            }
        }

        largest
    }

    pub fn g_2_3_max_kind_var() {

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest1(&number_list);
        println!("The largest number is {}", result);
        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest1(&char_list);
        println!("The largest char is {}", result);
        
    }

    //结构体定义的泛型

    struct Point<T>{//声明泛型的名称才能用于定义参数类型

        x: T,
        y: T,
    }

    pub fn g_3_genri() {
        
        //let wont_work = Point{x: 5, y: 4.0};
    }
    #[derive(Debug)]
    struct Point1<T, U>{

        x: T,
        y: U,
    }

    pub fn g_4_genri_1() {
        
        let two_integer = Point1{x: 2, y: 4};
        let two_float = Point1{x: 4.0, y: 3.0};
        let float_int = Point1{x: 3, y: 2.0};
    }
    //你可以在定义中使用任意多的泛型类型参数，不过太多的话，代码将难以阅读和理解
    //当你的代码中需要许多泛型类型时，它可能表明你的代码需要重构，分解成更小的结构。
    //方便你理解


    //枚举定义中的泛型

    /*
    enum Option<T> {//单泛型枚举
        Some(T),
        None,
    }

    enum Result<T, E> {//多泛型枚举
        Ok(T),
        Err(E),
    }
    代码中定义了多个结构体或枚举，它们不一样的地方只是其中的值的类型的时候，可以通
    过泛型类型来避免重复。
    
    
     */

    //方法中定义泛型

    impl<T> Point<T>{
        //impl后面声明泛型作用可以在Point<T>中实现的方法中调用它
        //使Rust知道<>中包含的是泛型而不是其他类型

        pub fn x(&self) -> &T {
            
            &self.x//返回Point中x字段的引用
        }
    }

    pub fn g_5_x() {
        
        let s = Point{
            x: 5,
            y: 7,
        };
        print!("p.x: {}", s.x());
    }

    impl Point<f32> {

        fn distance_from_origin(&self) -> f32 {

            (self.x.powi(2) + self.y.powi(2)).sqrt()            
        }
    }

    //方法使用了与结构体定义中不同类型的泛型,及使用另一个结构体的泛型参数，用结构体调用结构体
    //方法签名中可以使用，不同于自身结构体中的类型

    impl<T,U> Point1<T,U>{

        fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {

            Point1{

                x: self.x,
                y: other.y,
            }
            
        }

    }
       
    pub fn g_6_making_Point() {
            
        let p1 = Point1{x: 4, y: 10.4};

        let p2 = Point1{x: "Hello",y: "c"};

        let p3 = p1.mixup(p2);

        println!("p3: {:?}", p3);


    }

    //泛参函数的性能
    /*
    
    Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
    单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
    这样保证了代码运行的效率和手写的重复代码的运行效率一样，没有额外的开销

    Rust本身类型系统和推断能力实现的

    泛型类型，将不同类型的相同操作集合到一起使得，同一个方法签名能对不同类型的参数进行操作，实现目的，区分泛型参数的具体类型
    这一操作由rust自动单态化实现

     */

    //trait定义共享行为

    /*
    trait 的作用：

    trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。
    可以通过 trait 以一种抽象的方式定义共享的行为。
    可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

    定义trait

    一个类型的行为由其可供调用的方法构成。
    如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
    trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

    定义了方法集合，这些方法是不同类型具有的相同行为的统一实现
    */

    //eg：Summary trait 定义，它包含由 summarize 方法提供的行为对不同类型数据进行总结


    pub trait Summary {
        
        //fn summarize(&self) -> String;
        

        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }

        
    }
    /*
    
    1.大括号中声明描述实现这个 trait 的类型所需要的行为的方法签名
    2.在方法签名后跟分号，而不是在大括号中提供其实现
    3.具体实现这个 trait 的类型都需要提供其自定义行为的方法体
    4.由rust编译器确保每个任何实现trait 的类型都拥有与这个签名的定义完
    全一致的方法
    编译器保证每个实现该trait的类型都具备相同行为，和与之对应的方法（是由trait签名定义的）

     */

    //实现trait

    pub struct NewArticle{

        pub healine: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    //实现trait定义方法签名定义的具体行为

    impl Summary for NewArticle {

        fn summarize(&self) -> String{

            format!("{}, by {} ({})", self.healine, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            
            format!("@{}", self.healine)
        }
    }

    pub struct Tweet{

        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }


    impl Summary for Tweet{

        //fn summarize(&self) -> String {

            //format!("{}; {}", self.username, self.content)
        //}

        fn summarize_author(&self) -> String {
            
            format!("@{}", self.username)
        }
    }


    //调用trait方法和调用类型上非trait方法一样


    pub fn c_1_use_trait() {
        
        let tweet = Tweet{

            username: String::from("horse_ebooks"),

            content: String::from("of course, as you probably already know, people"),
            
            reply: false,
            
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    /*
    
    注意你可以在crate本地作用域中实现trait，且trait必须位于本地作用域中，同时你不在外部作用域中实现Crate本地作用域中的trait
    这个限制被称为相干性{coherence}，或者称为孤儿规则O（orphan rule），其没有父类型。

    这条规则确保了其他人编写的代码不会破坏你代码，反之亦然

    没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现
    
     */


    //默认实现

    pub trait Summary1 {
        
        //fn summarize(&self) -> String;
        fn summarize(&self) -> String{

            String::from("Read more...")
            
        }
    }

    pub struct Trait_1{

        default: String,

        kinds: String,
    }

    impl Summary1 for Trait_1 {
        
        
    }
    //默认实现在trait中直接定义方法签名的内容，而无需再在每个实现该trait类型中的方法去定义，该trait的行为

    //在该类型的impl块中留空imal块后就可以继续调用

    pub fn c_2_trait_default(){

        let trait1 = Trait_1{

            default: String::from("trait_function"),

            kinds: String::from("summmary"),
        };

        //let trait1 = Trait1::new();
       // println!("Trait defult function is summaries ", trait1.summarize());
    }
    
    //trait默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现 ，只要在实现trait中定义

    pub fn c_3_default() {
        
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
            };
            println!("1 new tweet: {}", tweet.summarize());
            
    }//当一个方法默认实现时再在实现trait中的类型中定义相同的方法时相当于方法重载，修改方法的内容，而采用同样的方式调用

    //无法从相同方法的重载实现中调用默认方法，既无法从trait summary实现中，调用和要实现方法同名的方法

    //trait作为参数

    impl NewArticle{

        fn test() {
            
            print!("hello world!");
        }
    }

    pub fn notify(item: impl Summary){

        println!("Breaking News! {}", item.summarize());
    }

    pub fn c_4_trait_to_parameter() {
        
        let tweet = Tweet{
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
            
        };

        notify(tweet);
    }
    /*
    item 参数，我们指定了 impl 关键字和 trait 名称，而不是具体的类型。
    该参数支持任何实现了指定 trait 的类型，可以调用来自于Summary trait中的方法
    可以传递实现了trait的实例来调用使用trait作为参数的函数
    
     */

    //Trait Bound语法

    //是一种限制指定trait作为参数的方法，（更短的语法糖）
    //之前的impl trait指一种较长的语法糖（简化指定参数代码）

    //eg：

    pub fn notify1<T: Summary>(item1: T) {
        
        println!("{}", item1.summarize());
    }

    //相比impl 适合简短的例子，而trait bound适合更复杂的场景

    //同时impl可以自由指定多个参数的trait的类型，而trait bound强制要求所有的参数类型一致

    //限制参数类型
    pub fn notify2(item1: impl Summary, item2: impl Summary) {

    }
    pub fn notify3<T: Summary>(item1: T, item2: T) {

    }

    //通过+指定多个trait bound
    //需要同时实现两个不同的 trait

    pub fn notify5(item: impl Summary + Display) {
        
    }

    //+ 语法也适用于泛型的 trait bound：

    pub fn notify6<T: Summary + Display>(item: T) {
        
    }//默认使用Display

    //使用where简化trait bound

    //直接使用trait bound的泛型类型，在有多个参数时过于复杂不便于阅读，使用wher从句便于阅读

    //fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

    fn some_function1<T, U>(t: T, u: U)

        where T: Display + Clone,
              U: Clone + Debug
    {}


    //返回实现了trait的类型
    //返回类型的用处，返回一个只是指定了需要实现的 trait 的类型的能力在闭包和迭代器场景十分的有用

    pub fn c_5_return_trait() -> impl Summary{
        
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
            }
            
    }//这个返回类型调用方不知

    //使用trait有条件地实现方法

    //通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait
    //的类型实现方法

    //use std::fmt::Display

    struct Pair<T>{

        x: T,
        y: T,
    }

    impl <T> Pair<T> {

        fn new(x: T, y: T) -> Self{

            Self{

                x,
                y,
            }
        }
        
    }
    impl <T: Display + PartialOrd> Pair<T> {
        
        fn cmp_display(&self) {
            
            if self.x >= self.y{

                println!("The largest member is x = {}", self.x);
            }else {
                
                println!("The largest member is y = {}",self.y)
            }
        }
    }
    //只有实现特定trait的类型，才能调用的的方法，进一步对trait做了区分，限制

    //可以对任何实现了特定 trait 的类型有条件地实现 trait（指特定类型下使用的方法），只有该类型的实例才能调trait方法

    //对任何满足特定 trait bound 的类型实现

    //trait 被称为 blanket implementations，他们被广泛的用于 Rust 标准库中


    //trait总结

    /*
    trait 和 trait bound 让我们使用泛型类型参数来减少重复，并仍然能够向编译器
    明确指定泛型类型需要拥有哪些行为

    //向编译器提供了 trait bound 信息，它就可以检查代码中所用到的具体类
    型是否提供了正确的行为。

    //rust能在编译时就察觉，一个类型调用没有定义的行为，甚至在编译前就察觉。强制我们修改
    //也无需编写运行时检查行为的代码，因为在编译时就已经检查过了，相比其他那些不
愿放弃泛型灵活性的语言有更好的性能

     */


    //生命周期（泛型）：就是引用保持有效的作用域
    //不同于其他泛型帮助我们确保类型拥有期望的行为，生命周期则有助于确保引用在我们需要
    //他们的时候一直有效。

    //生命周期和引用有效性
    //rust需要明确使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的


    //生命周期避免了悬垂引用


    pub fn l_1_mut(){

        //let r;

        {

            let x = 5;
            //r = &x;
        }

       //d println!("r: {}", r);//使用检查器检测借用，不合规的代码不允许编译执行
    }
                            
    //当r借用x，rust编译器的借用检查器通过比较变量生命周期(作用域大的生命周期长)标记为""`b"和”`a“大小，如果被引用者的生命周期小于引用者，rust拒绝编译


    //一个有效的引用，来自于引用数据比引用的生命周期长

    //泛型的生命周期‘

    //定义引用间的关系便于借用检查器可以对引用的生命周期进行检查比较，以防bug

    /*pub fn L_2_longest(x: &str, y: &y) -> &str {

        if x.len() > y.len(){

            &x
        }else {
            &y
        }
    }
    pub fn main1(){

        let string1 = String::from("abcd");
        let string2 = String::from("ac");

        let s = L_2_longest(&string1, &string2);
    }    */


    //生命周期注解语法

    //用于告诉rust多个引用的泛型生命周期是如何联系的不会出L_2_longest这种不知道该返回x还是y引用的情况
    //对于单个引用没有多少意义
    //eg  

    fn eg1(){

        //let &'a x;//语法T
    }

    //函数签名中的生命周期注解

    pub fn L_2_1_longest<'b,'a>(x: &'a str, y: &'a str ) -> &'a str {//<'a>生命周期泛型声明

        if x.len() > y.len(){//'a用于声明函数签名中参数的生命周期，表示的是生命周期是参数中较短的，确保返回值的引用在最短生命周期参数之前都是有效的

            &x
        }else {
            &y
        }
    }

    pub fn main2(){

        let string1 = String::from("abcd");
        let string2 = String::from("ac");

        let s = L_2_1_longest(&string1, &string2);

        println!("The longest words is {}", s);
    }    
    


    //深入理解生命周期


    pub  fn longest1<'a>(x: &'a str, y: &str) -> &'a str {

        x

    }      

    /*pub  fn longest2<'a>(x: &str , y: &str) -> &'a str{


        let result = String::from("really long string");

        result.as_str()    
    }*/
    
    pub fn main3(){

        let string1 = String::from("abcd");
        let string2 = String::from("ac");

        //let s = longest2(&string1, &string2);

        //println!("The longest words is {}", s);
    }    
    /*
    
    生命周期注释总结
    生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。一旦他们形成了某 种关联，Rust 就
    有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。


     */

    //结构体中定义生命周期注解
    //标量是一个确切的量，不能使用生命周期去注解，标量的生命周期是明确的
    #[derive(Debug)]
    pub  struct ImprotantExcerpt<'a>{

        part: &'a str,
    }

    pub fn main4() {

        let novel = String::from("Call me Ishmael. Some years ago...");

        let frist_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");

        let i = ImprotantExcerpt{

            part: frist_sentence,
        };

        print!("ImprotantExpert: {:?}", i);      
    }


    //生命周期省略（Lifetime Elision）


    //对于有些引用可以不用显式的标注出生命周期注解，
    //rust会根据三条原则推断出每个引用的生命周期注解，
    //如果经过三条原则后rust还是没有推断出引用的生命周期注解，rust就会报错，不能进行编译

    /*原则
    
    编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）

    函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
    返回值的生命周期被称为 输出生命周期（output lifetimes）。

    第一条规则是每一个是引用的参数都有它自己的生命周期参数
    第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
    第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self ，
    说明是 个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章), 那么所有输出生命周期参数被
    赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。
    
     */


    //方法定义中的生命周期
    impl <'a> ImprotantExcerpt<'a>{

        fn level(&self) -> i32{

            3
        }

        //应用第三条省略分析省略原则的例子

        fn announce_and_return_part(&self, announcement: &str) -> &str{

            println!("Attention please: {}", announcement);
            self.part
        }


    }

    //静态生命周期"'static"


    fn eg_2() {
        let s:&'static str = "you are right";
        //注意思考是否要创建一个在整个程序生命周期的范围里都有用的变量

        //大部分情况，代码中的问题是尝试创建一个悬垂引用或者可用的生命周期
        // 不匹配，请解决这些问题而不是指定一个 'static 的生命周期。

    }

    //结合泛型参数，trait bounds和生命周期注释

    //use std::fmt::Display;


    fn longest_with_an_anouncement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
        where T:Display
    {
        
        println!("Announcement! {}", ann);
        if x.len() > y.len(){

            x
        }else {
            
            y
        }


    


    }
}
 //第十章总结
 /*
 
 现在你知道了泛型类型参数、trait 和 trait bounds 以及泛型生命周期类 型，你已经准备好编写既不重复又能适用于多种场景的代码了。
 泛型类型参数意味着代码可以适用于 不同的类型。
 trait 和 trait bounds 保证了即使类型是泛型的，这些类型也会拥有所需要的行为。
 由 生命周期注解所指定的引用生命周期之间的关系保证了这些灵活多变的代码不会出现悬垂引用。
 而所有的这一切发生在编译时所以不会影响运行时效率
 
  */