use std::io::Bytes;

#[allow(dead_code)]

fn main1() {
    //作用于是一个项在程序中的有效范围
    let _s = "hello"; //s的作用域开始
                            //硬编码格式，数值直接在编码时被输入到变量,即在编码时就知道大小和长度
    string1();//到此s的作用域结束
} //此时s被丢弃，并使用drop删除，释放内存，

fn string1() {
    let mut s = String::from("hello"); 

    s.push_str(", world!");

    println!("{}", s);
    //title：数据与变量的交换方式移动
    //rust中的拷贝和其他语言的拷贝一样
    let x = 4;//数据类型为标量的数据拷贝
    let y = x;//此时x与y绑定的值都是5，两个5在内存中占用了不同的空间

    //数据类型为String的拷贝，允许数据的大小未知，在程序运行时向内存请求空间
    //string型的变量由指针，容量，和长度三部分构成
    let s1 = String::from("hello");
    //let s2 = s1;//两个变量指针指向相同的空间，没有拷贝堆上的数据
    //println!("{}, world!", s2);//由于ownership，s1被GC清除没有意义（s1的作用域已经结束），s2还存在作用域中，能继续起作用
    
    //数据与变量交互方式克隆

    let s3 = s1.clone();//此时s3上的值是s1克隆而来，不是通过指针指向
                        //与s1指向堆上相同的空间，s3指向数据在堆上独立占据了一部分空间
                        //注意，这只是针对于String类型的变量，即大小不知，长度不确定的变量使用
                        //对于标量即一开始就知道大小，长度的变量没有此限制，包括元组，数组除外？
                        //存放于栈
    println!("s1 {} s3 {}", s1, s3)//浅拷贝（shadow copy）只是复制指针和长度和容量
                                    //深拷贝（deep copy）在浅拷贝的基础上同时复制数据（内存内容）
}


//所有权与函数

/*fn takes_ownership(some_string: String){

    println!("some_string: {}", some_string);

}

fn makes_copy(some_integer: i32){

    println!("some_integer: {}", some_integer);
}
fn main(){
    let s = String::from("hello");
    
    takes_ownership(s);//s的值被转移到some_string

    let x = 5;//s只会被移出scope

    makes_copy(x);//x的值是复制的之后还可以使用

}//drop方法用于清除移出scope外的，堆上的数据*/

/*//title: 转移返回值的所有权

fn gives_ownership() -> String {

    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}

fn main(){

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1 {} s3 {}", s1, s3);
}*/

//返回参数所有权和函数结果，可以通过元组的形式

/*fn calculate_length(s: String) -> (String, usize){

    let length = s.len();

    (s, length)//位于返回语句后的语句和表达式不会执行

}

fn main(){

    let s1 = String::from("hello");

    let (s2, length) = calculate_length(s1);

    println!("s2 {} len {}", s2, length);
}*/

//引用和借用

//引用
/*fn calculate_length(s: &String) -> i32 {

    let length = s.len();

    length.try_into().unwrap()
}

fn main(){

    let s1 = String::from("hello");

    let length = calculate_length(&s1);//&符号就是引用，此时实参传递的是引用，
                                        //没有转移所有权，s1可以继续使用到作用域结束后丢弃

    println!("s1 {} length {}", s1, length);
}*/

//借用

/*fn change(s: &mut String){

    s.push_str(", world!");
}

fn main(){

    let mut s1 = String::from("hello");

    change(&mut s1);//引用可变，则实参本身也是可变的

    println!("{}", s1);//注意同一时间不能多次借用一个变量

    //let q1 = &mut s1;
    //let q2 = &mut s2;
    //这是不合法的，因此rust对于变量存在限制允许可变性，不允许同一时间多次修改同一变量
    //这样就不会产生数据竞争，因为数据竞争型的代码不会被编译，但是使用{}（作用域）可以实现多次修改变量值（不是同一时间）
    //eg：let q1 = &mut s1;
    //{let q2 = &mut s1}
    
}*/
//可变引用和不可变引用不能同时存在

/*fn main(){

    let mut s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;

    println!("r1 {} r2 {}", r1, r2);

    let r3 = &mut s1;//判断不可变引用bianl，是否在可变引用变量之后被再次使用，导致作用域重合的能力叫非词法作用域生命周期（NLL）
                        //这对于使数据的更明确，不用跟踪确定数据。

    println!("r3 {}", r3);

    r3.push_str(", world!");

    println!("s1 {}", s1);
    
} */

fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){

        if item == b' ' {

            return &s[0..i];
        }
    }

    &s[..]
}

fn main(){
    /*
    let s1 = String::from("hello world!");返回值只在&String中有用，而与String相分离不能使用在String的上下文中

    let m = first_word(&s1);
     */

    let mut s1 = String::from("hello world!");

    let m = first_word(&s1);

    //s1.clear();//清空字符串
    
    print!("m {}", m);
}