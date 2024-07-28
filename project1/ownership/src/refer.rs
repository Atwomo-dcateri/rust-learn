//悬垂指针

//指针指向内存已经被释放并分配给其他持有者

//rust不会出现悬垂指针，在引用之前，数据不会离开作用域（从内存释放）

/*fn no_dangle() -> &String{

    let s = String::from("hello");

    &s//此时会报错，引用未撤去，但是内存被释放
}

fn main(){

    let s = no_dangle();

}*/


//slice类型（数据类型）
//slice是没有所有权的数据类型，同引用。可以引用集合中一段连续的元素序列
//是对于一个完整数据元素集合的部分引用

fn first_word(s: &String) -> usize{

    let tybes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){

        if item == b' '{

            return i;
        }
    }

    s.len
}


//字符串sclie ，用于解决数据在某个状态下同步的问题
fn slice1(){

    let s = String::from("hello world");

    let s1 = &s[0..5];
    let s2 = &s[6..11];

    //..是range语法

    let s3 = &s[..3];
    let s3 = &s[0..3];//这两个引用是一样的

    //可以舍弃尾部数字
    let len = s.len();

    let s4 = &s[3..];
    let s4 = &s[3..len];

    //舍弃两个值获取完整的字符串

    let s5 = &s[0..len];
    let s5 = &s[..];
}

//字符串 slice类型声明  ：&str

fn first_word1(s: &String) -> &str {

    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {

        if item = b' ' {

            return &[0..i];
        }
    }

    &s[..]
}

fn main(){

    let mut s1 = String::from("hello world!");

    let m = first_word1(&s1);

    s1.clear();//清空字符串
    
    print!("m {}", m);//此时会报错，clear（）尝试申请一个可变引用，而m同时指
                        //向s1，是一个不可变引用，不可变引用和可变引用同时出现报错

}

//传递字符串slice参数

//eg：

fn slice2(s: &str) -> &str {

    //这是更有经验的对String变量的处理。这充分利用了deref coercions优势
    //这使得API更加的通用同时不会丢失任何功能
}


//其他类型的slice

fn array(){

    let a = [1, 2, 3, 4, 5];

    let b = &a[0..2];

    assert_eq!(slice, &[2..3]);
}

//4章总结，rust具备的所有权和作用域，使得rust编译内存安全。rust提供与其他编程语言相同的方式控制
//使用内存，数据拥有者在离开作用域后自动清除其数据，无需你编写和调试相关代码