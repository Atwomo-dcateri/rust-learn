//结构体类似于元组，但是由于每部分数据可以单独命名，指定数据类型（可以相同），同时可以作为数据类型定义数据

//一个基本的结构体

struct User{

    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
//结构体通过创建实例来使用

//5-2创建结构体实例

fn struct_user(){

    let mut user1 = User{//rust结构体中不存在将某个字段标记为可变的，应该整体标记

        username: String::from("Li Hua"),
        email: String::from("345667@163.com"),
        sign_in_count: 123456,
        active: true,
    };

    user1.email = String::from("123456@163.com");

}


//5-4返回一个结构体实例
//通过函数简单修改结构体实例的值

fn bulid_user(username: String, email: String) -> User {

    user1{
        username: username,//当参数名和字段名完全相同时可以简化写法，修改后行为和之前完全相同
        email,//字段初始化间写语法
        active: true,
        sign_in_count: 123456,
    }
}

//结构体更新语法
//使用旧实例的值创建新实例

fn new_user(User: user) -> User{

    let user2 = User{

        username: user.username,
        email: user.email,
        sign_in_count: 12334,//user不能再被使用，user数据被移动到user2中，但是如果使
                            //用的sign_in_count,和active的值user还可以使用，数据是被克隆
                            //之前数据交互方式，对结构体一样有效
    
        active: true,
    };

    let user3 = User{

        username: String::from("Li Qiang"),
        ..user//只更新username而其值使用user
    };
    user2
    
}


//元组结构体（tuple struct）
//有结构体名称，无字段名，但有字段类型定义

fn tuple_s(){

    struct color(i32, i32, i32);//定义的每一个结构体都是一个类型，他们之间
                                    //不能互相传递数值，即使结构体字段类型和字段名相同

}

//类单元结构体

//类似于元组（），在想要的类型上实现的trait但是不用存储值时使用，

fn unit_struct(){

    struct alls ;

    let a = alls;
}

//结构体所有权

//一般使用String类型定义字段，不使用&str（slice string），这需要搭配生命周期在实例化的时候使用
//生命周期保证结构体引用的数据有效性跟结构体本身保持一致


//为什么有时需要使用结构体

//计算长方形面积（rectangle area）

//数据关联即不同数据结合在一起才能有意义，一般数据结构不能体现数据的关联性

//普通数据结构的形式
fn area(width: u32, height: u32) -> u32{

    width * height//没有体现这是长方形高和宽的关系，从而表现这是计算长方形面积

}

//使用元组

fn area1(dimensions: (u32, u32)) -> u32{

    dimensions.0 * dimensions.1;//这样虽然方便计算，增加了数据的结构性，但是
                                //没有给出数据的名称，不便于理解，计算面积时还没有太大问题
                                //但是绘制屏幕时长宽搞错可是会出问题的

}

//使用结构体进一步增加数据结构性，赋予数据更多意义，便于理解和使用

//关联数据


struct Rectangle{

    width: u32,
    height: u32,
}

fn area3(Rectangle: &re) -> u32{//此时主函数对re仍有使用权，因为参数传递的是结构体的引用

    re.height * re.width//明显看出计算的是长方形的面积

}

fn main(){

    let rect1 = Rectangle{

        width: 50,
        height: 30,
    };
    
    println!("The Rectangle of area is {}", rect1);

}

//派生trait增加功能

//println！不能直接打印结构体类型变量，因为结构体提供更多的可能性，从构成上有逗号，字段名，大括号
//rust不会推断你的意图。之前的数据类型都有一个确切的值供于打印是唯一的判断基准


fn main1(){

    rect1 = Rectangle{

        width: 50,
        height: 30,
    };

    println!("rect1 is " , rect1);
}

//使用另一种debug的形式打印出值(这是调试的调用)，dbg！宏：接受一个表达式的所有权，打印结束，返回结果的所有权
//dubug的信息输入的到stderr控制流

fn main1(){

    rect1 = Rectangle{

        width: dbg(50),
        height: 30,
    };

    dbg!(rect1);
}

//方法语法
//方法与函数没有区别，第一个参数永远为self，只在结构体枚举或trait对象上下文定义

//定义一个方法

fn main(){

    let rect1 = Rectangle{

        width: 50,
        height: 30,
    };

fn auto_refer_dere(){

    rect1.width();//自动补齐为
    (&rect1).width();//解引用访问对象，对象使用”.“调用方法
                    //自动引用（&rect1），自动引用是因为有明确的接收者（&self指向实例引用）和方
                    //法名，rust能自动计算参数做出的修改（&mut self， &self， self）

}
    println!(
        "The area of the Reatangle is {}", 
        rect1.area()//方法中的第一个参数是self类型的
        //self：Self表示第一个参数的类型（Self是impl块类型的别名），使用&self实则上是是使用self: &Self
        //至于使用&self，是借用实例的所有权，而非修改实例本身。
        //修改实例&mut self 活动实例的所有权，并将实例转化为另一个实例的技术

        //方法替代函数是便于组织的
        //将所有与实例相关的方法放在impl块中是方便之后的使用，
        //方法可以和字段同名，便于使用只是使用时注意加了（）时使用方法，否则使用字段值


    );

}

fn main3(){

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 }; 
    let rect3 = Rectangle { width: 60, height: 45 };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


//所有在impl块中的函数被称为关联函数
//而第一个参数不是self类型的是不是方法的关联函数称为构造函数，


impl Rectangle{

    fn area(&self){

        self.width * self.height//计算Rectangle面积并返回结果
    }

    fn width(&self) -> bool{//只能访问字段值的方法getter，rust不会自动实现，实现字的私有，但是方法是公有的

        Reatangle.width > 0;
    }

    fn can_hold(&self, other: &Rectangle) -> bool{

        self.width > other.width && self.height > other.height//为方法添加参数便于访问
    }

    fn square(size: u32) -> Rectangle{

        Rectangle{
            width: size,
            height: size,
        }
    }
}

//使用构造函数
//一个结构体可以有多个impl块，但是一般不用分成多个块表示不同的关联函数
fn main4(){
    let rect1 = Rectangle{

        width: 32,
        height: 23,
    };

    let squ = Rectangle::square(34);//"::" :: 语法用于关联函数和模块创建的命名空间

}