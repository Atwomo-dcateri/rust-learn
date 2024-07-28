#[allow(dead_code)]

struct User{

    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn struct_user(){

    let mut user1 = User{

        username: String::from("Li Hua"),
        email: String::from("345667@163.com"),
        sign_in_count: 123456,
        active: true,
    };

    println!("user1.email: {}", user1.email);

    //user1.email = String::from("123456@163.com");
    user1 = bulid_user("Li Fa".to_string(), "123".to_string());
    
    println!("user1.name: {}", user1.username);
}

fn bulid_user(username: String, email: String) -> User {

    User{
        username: username,
        email,
        active: true,
        sign_in_count: 123456,
    }
}
#[derive(Debug)]
struct Rectangle{

    height: u32,
    width: u32,

}

fn area3(re: Rectangle) -> u32{//此时主函数对re仍有使用权，因为参数传递的是结构体的引用

    re.height * re.width//明显看出计算的是长方形的面积

}

impl Rectangle{

    fn area(&self) -> u32 {

        self.width * self.height//计算Rectangle面积并返回结果
    }

    fn width(&self) -> bool{

        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool{

        self.width > other.width && self.height > other.height//为方法添加参数，实现更多功能
    }

    fn square(size: u32) -> Rectangle{

        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main(){

    let squ = Rectangle::square(34);

    dbg!(&squ);
}