//什么是枚举
//可以 枚举 出所有可能的值，这也正是此枚举名 字的由来。用于处理能枚举的数据
//是一种自定义的类型

//定义一个枚举类型

enum IppAddrkind{

    v4,//用于确定枚举值属于哪个成员（variants）
    v6,

}
//定义鉴定ip类型的函数

fn route(Ip_type: IppAddrkind){


}

//创建结构体存储IP地址

struct IpAddr{

    kind: IppAddrkind,
    address: String,
}
//枚举值

fn variant(){

    let four = IppAddrkind::v4;//IppAddrkind::v4就是枚举值
    let six = IppAddrkind::v6;//枚举成员位于标字符的命名空间下
                                //”：：“创建命名空间

}

fn main1(){

    let home = IpAddr{
        kind: IppAddrkind::v4,
        address: String::from("127.0.0.1"),

    };

    let looplock = IpAddr{

        kind: IppAddrkind::v6,
        address: String::from("::1"),
    };
}

//使用枚举代替结构体
//每个枚举成员可以处理不同类型和数量的数据,结构体的实例的数据类型是统一的
//结构体不能对字符段数据再进行数据定义
//eg：

enum IpAddr1{

    v4(String),
    V6(String),
}

//定义每个成员为不同的数据类型

enum IpAddr{

    v4(u8, u8, u8, u8),
    v6(String),
}

fn ip_vari1(){

    /*let home = IpAddr::v4(String::from("127.0.0.1"));

    let looplock = IpAddr::v6(String::from("::1"));*/

    let home = IpAddr::v4(127,0,0,1);

    let looplock = IpAddr::v6(String::from("::1"));


}

//枚举成员的数据类型可以是结构体

//如果每个成员的数据类型不同，得使用多个结构体才能实现枚举

//使用枚举是因为成员变量的数据类型不同，导致对每个成员的操作不同

/*enum Message(){

    Quit,
    Move{i32, i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}*/

//可以使用impl块来定义枚举类型上的方法

//Option枚举；

//用于表达一种只为空值和非空值的变量
//空值是一个因为某种原因目前无效或缺失的值
//Rust 并没有空值，不过它确实拥有一个可以编码存在或不 存在概念的枚举

enum Option<T>{//<T>是泛参数类型可以接受任意类型的数据

    Some(T),
    None,//使用枚举变量None可以表示存在或不存在的概念

}

fn Option1(){

    let some_number = Some(5);
    let some_String = Some("a string");

    let absent_number: Option<i32> = None; //指定变量为空值
}

//使用Option<T>的成员值

fn Option_plus() {
    let x: i8 = 5;
    let y: Option<i8> = Some(9);//通过Option<T>的设计，在对Option<T>运算时必须将其转换成
                                //T，确保值不为空。保证rust中不会存在空值泛滥，增加代码的安全性

    let sum = x + y;
}

//match表达式用于处理枚举的控制流结构
//根据每个成员的值进行模式匹配执行对应的代码
//模式可由字面值、变量、通配符和许多其他内容构成
// match 的力量来源于模式的表现力以及编译器检 
//查，它确保了所有可能的情况都得到处理
//类似一个硬币分拣器，合适的硬币会在第一个匹配落入的轨道中eg：

enum Coin{

    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {

    match coin{
        Coin::Penny => {
            println("Luck Penny!");
            1
        },
        Coin::Nickel => 5,//"=>"符号将模式匹配和执行代码（数值）部分的操作分开
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
//枚举变量中套用枚举变量

enum UsState{

    Alabama,
    Alaska,
}
enum Coin1{

    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents1(coin: Coin1, state: UsState) -> u8{
    
    match coin{

        Coin::Penny => 1,
        Coin::Nickel => 12,
        Coin::Dime => 13,
        Coin::Quarter(state) => {

            println!("State quarter from", state);
            25
        },

    }
}

//匹配Option<T>

fn plus_one(num: i32) -> i32{

    match Option{

        Option::None => None,
        Option::some() => some(num + 1),
    };

}

//rust中匹配是穷尽的
//没有完全匹配的枚举变量的代码是不会被编译运行的，
//特别是对于Option<T>中的None而言
//通配匹配

fn dice_roll(){

    dice_roll = 9;
    match dice_roll{

        3 => println!("move roll!"),
        7 => println!("add roll!"),
        other => println!("go other"),//注意通配模式要放在最后，否则位于通配模式后的模式匹配永远不能执行
        _ => println!("Don`t !"), //使用"_"表示不使用通配模式匹配值,即已设置模式匹配外，其余模式匹配为无
    }
}

//if let 是match中 '_'匹配模式的替换
// match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检 查的权衡取舍。
//if let忽略了match中要求模式一定要穷尽的特性

//eg：

fn u8_value(){

    let some_value = Some(0u8);
    let count = 0;
    match some_value{
        
        9 => println!("9"),
        _ => count += 1,
    };
}

fn u8_value1(){

    let some_value = Some(0u8);
    let count = 0;

    if let Some(9) = some_value {
        println("9");
    }
    //添加else可以为没有在模式匹配中的数据做出回应
    else{
        count = count + 1;
    }
}
