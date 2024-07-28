/*
常见集合

//集合一种数据结构
1.集合可以包含多个值。
2.不同于内建的数组和元组类型，这些集合指向的数据是储存在堆上的，这意味着（集合的存储和使用类似于Sting）
数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩
小。
3.每种集合都有着不同功能和成本，而根据当前情况选择合适的集合，


集合的类型：

1.vector 允许我们一个挨着一个地储存一系列数量可变的值
2.字符串（string）是字符的集合。我们之前见过 String 类型，不过在本章我们将深入了解。
3.哈希 map（hash map）允许我们将值与一个特定的键（key）相关联。这是一个叫做 map 的
更通用的数据结构的特定实现。


*/
/*
第一个类型vector：
关键字Vector<T>

和Option<T>一样



*/

//创建Vectorb变量


#[allow(dead_code)]


pub mod fornt_of_vector{
   
    pub fn v_1(){

        let v: Vec<i32> = Vec::new();//普通创建
    
        let mut v1 = vec![1, 2, 3];//调用宏创建，不用指明具体的数据类型，rust会自动推断出类型，方便使用
    
        //更新vector
    
        //使用push（）方法压入
    
        v1.push(5);
    
        //丢弃vector
    
       // {
            //let v = vec![1, 2, 3, 4]; // 处理变量 v
           
        //} // <- 这里 v 离开作用域并被丢弃
    
        //读取vector
    
        //1 使用索引获取值
    
        let third: &i32 = &v1[2] ;
        //直接使用索引超过索引值的数时会造成程序崩溃
    
    
        //使用get（）方法.使用match控制流结构进行模式匹配，对get（）的值进行处理
        //非常有用的数据类型，访问数据失败不会引起程序崩溃
        match v1.get(2){

            Some(third) => println!("The third element is {} ", third),
            None => println!("The third element is None"),
        }

        //使用get（）方法索引不存在的值时，会返回none，此时使用Option<t>可以进行处理，而不会造成程序崩溃
        
    }
    
    pub fn v_2(){
    
        //rust vector可以处理索引不存在值时进行处理
        //借用规则和所有权对他仍然有效，确保引用在作用域的有效性
    
    
        let mut v = vec![1, 2, 3];
    
        
        v.push(8);


    
        //println!("The first element is: {}", v1);


        //eg:
        let v1 = &v[100];

        let v2 = v.get(100);
    
        
    }

    pub fn v_3_refer_borrow (){
        //#[derive(Debug)]
        let mut v = vec![1, 2, 3, 4, 5];
        
        let first = &v[0];
        //不可变引用和可变借用不能同时出现在一个作用域中
        //v.push(6);

        println!("The value of v is {:?} \nv5 {}", v, v[5]);

        println!("The first element is: {}", first);
    }

    //遍历Vector中的所有元素

    pub fn v_4_traverse(){

        let v = vec![100, 32, 57];
        //使用for循环按序逐个遍历vector的每个元素

        let mut v1 = vec![100, 54, 56];
        for i in &v{

            print!("{} ", i);
        }

        println!("{:?}", v1);

        for j in &v1{

            print!("{} ", j);
        }

        print!("\n");

        for k in &mut v1{

            *k += 10;

            print!("{} ", k);
            
        }
    }

    //使用枚举类型存放多类型

    //电子表格
    #[derive(Debug)]
    pub enum Spreadsheet{

        Int(i32),
        Float(f64),
        Text(String),
    }

    
    pub fn v_5_Spreadsheet(){

        //枚举类型的Vector

        let v = vec![

            Spreadsheet::Int((3)),
            Spreadsheet::Float((45.0)),
            Spreadsheet::Text((String::from("bule"))),

        ];

        print!("{:?}", v);

    }

    //对于枚举类型的Vector，rust必须开始就知道所有枚举类型的成员类型，这样就知道每个元素需要多少内存
    //在事先不知道类型的情况下rust会出错枚举技术失效。当然此时可以trait对象来解决
    //

    
//详解String数据类型

    pub fn S_1_String(){

        //定义一个字符串

        let mut s = String::new();

        let data = "initial content";
        //用于获取字符串的字面值

        let s = data.to_string();

        let s = "intital content";

        let s = String::from("initital contemt");
        
        //谨记rust使用的是UTF-8进行编码存储，即一个字符4个字节
        //覆盖之前的值
        let hello = String::from("عليكم السالم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("וםֹשלָׁ");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");


        //更新Sting字符串
        //使用 + 运算符或 format! 宏来拼接 String 值

        let mut s1 = String::from("foo");

        let s2 = "bar";
        s1.push_str(s2);

        println!("s1: {}", s1);

        let mut s3 = String::from("foo");

        s3.push('l');

        println!("s3: {}", s3);

        let s4 = String::from("hello");

        let s5 = String::from(", world!");
        // 使用加号拼接意味着我们使用第二个字符串的 引用 与第一个字符串相加，
        //+ 运算符使用了 add 函数:fn add(self, s: &str) -> String {

        //使用 &s5 是因为 &String 可以被强制转换（coerced）将 &s2 变成了 &s2[..]
        //使用了一种强制转换技术（deref ）

        //s4会在相加后没有作用add 获取了 self 的所有权，因为 self没有使用&这意味着示
        //中的 s4 的所有权将被移动到 add 调用中
        let s6 = s4 + &s5;
        println!("s6: {}", s6);


        //对于更为复杂的字符串连接使用format！宏（使用加号连接便于理解rust执行了什么操作）

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);//format的工作原理和println一样只是它返回一个值而不是打印输出到屏幕上

    }
    
    //索引字符串
    pub fn s_2_index(){

        let s1 = String::from("hello");
        //不能使用索引的形式访问一个String类型的值，因为使用UTF-8不同字符编码的长度不同，unicode的标量需要2个字节
        //而字母等只需要一个字节进行编码
        //let h = s1[0];
        let len = String::from("Hola").len();
        print!("{}", len);
        let len = String::from("Здравствуйте").len();
        print!("{}", len);


        let hello = "Здравствуйте";
        //Rust 根本不会编译这些代码，并在开发过程中及早杜绝了误会的发生
        //因为使用unicode编码每个字符两字节，而返回值只是返回了”3“的第一个字节

        //String型变量是一个封装的utf-8编码的Vec类型的
        //let answer = &hello[0];

    }

    //字节，标量值和字型簇

    pub fn s_3_byte_scalar(){

        //计算机中存储的原始数据以不同的角度解释具有不同的意义

        let s = String::from("नमस्ते");

        //在计算机中是存储的字节
        //而从unicode角度是六个字符
        //从字符簇的角度，就是一段文字，没有语法的修饰
        //使用索引rust不知道该返回什么值，对于值的解释有四种
        //rust不允许使用索引的方式访问String的原因，
        //索引必须从开头到索引位置，确定有效值时间为（O（1））

    }

    pub fn s_4_slice(){
        //Rust 会要求你更明确一些。为了更明确索引并表明你需要一个字符串 slice，
        //相比使用 [] 和单个值的索引，可以使用 [] 和一个 range 来创建含特定字
        //节的字符串 slice\
        let hello = "Здравствуйте";
        //明确指定后才不会出错
        let s = &hello[0..4];//注意的指定有效的索引的范围
        

    }
    
    //遍历字符串

    pub fn s_5_traverse(){
        
        for i in "नमस्ते".chars(){

            println!("{}", i);
        }

        for j in "नमस्ते".bytes() {
            
            print!("{} ", j);
        }
    }
    //注意，总而言之，字符串还是很复杂的。不同的语言选择了不同的向程序员展示其复杂性的方式。Rust 选择
    //了以准确的方式处理 String 数据作为所有 Rust 程序的默认行为，这意味着程序员们必须更多的思
    //考如何预先处理 UTF-8 数据。这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性，不过也使
    //你在开发生命周期后期免于处理涉及非 ASCII 字符的错误。
    



    //哈希（hash map）
    // HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射
    //它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中
    //很多编程语言支持这种数据结构，不过通常有不同的名字：哈希、map、对象、哈希表或者关联数组
    use std::{collections::HashMap, string, vec};//使用标准库集合中的hash map包路径，用于导入其中的module
    pub fn m_1_map(){
        //构建哈希表的方法，使用标准库
        //方便，但是标准库对hash表的支持并不多没有相关宏

        let mut scores = HashMap::new();
        
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("red"), 50);

        //使用Vector和collect方法创建哈希表

        let teams = vec![String::from("blue"), String::from("blue")];

        let initial_scores = vec![10, 50];

        let scores1: HashMap<_, _> = teams.iter()
                                        .zip(initial_scores.iter())
                                        .collect();
        //使用HashMap<_, _>保证rust能推断出collect中数据的类型
        //而除非显式指定否则 Rust 无从得知你需要的类型。但是对于键和值的类型参数来说，可以使用下划线占位，而
        //Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型

    }

    pub fn m_2_ownership() {
        
        //标量不会丢失所有权，而对于存储于堆上的数据类型在数据移入哈希表后不能再使用

        //这个原则贯穿rust始终

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);//fielf_name和field_value在这之后不再起作用

        //print!("{} ", field_name);

        
    }

    pub fn m_3_visit() {   
        
        let mut scores = HashMap::new();
        
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 50);

        let team_name = String::from("Blue");

        let score = scores.get(&team_name);

        println!("score: {:?}", score);

        //遍历哈希表

        for (key, value) in &scores  {

            println!("{} {}", key, value);
        
        }

        //更新哈希表
        //覆盖
        //相同键插入不同值以最后次插入为准

        scores.insert(String::from("Blue"), 100);
        scores.insert(String::from("Blue"), 120);
        println!("scores: {:?} ", scores);

        
        //只有键没有值
        //使用entry方法检查特定的键如果没有值就返回一个枚举，代表不存在

        scores.entry(String::from("Blue")).or_insert(100);
        scores.entry(String::from("Yellow")).or_insert(100);

        println!("Score: {:?}", scores);


        //根据旧值更新一个值
        //统计一个单词出现了多少次
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        
        for word in text.split_whitespace() {
        
            let count = map.entry(word).or_insert(0);
            *count += 1;//根据键更新键值，不同键是更新的键值不同
        }
        
        println!("{:?}", map);
    }

    //Hash函数
    /*
    HashMap 默认使用一种 “密码学安全的”（“cryptographically strong” ）1 哈希函数，
    它可以抵抗拒绝服务（Denial of Service, DoS）攻击
    这不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价
    如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 hasher 来切换为其它函数。
    hasher 是一个实现了 BuildHasher trait 的类型
    并不需要从头开始实现你自己的 hasher；crates.io 分享的实现了许多常用哈希算法的 hasher 的库。
     */
}


//第8章总结
    /*
    
    
     */





