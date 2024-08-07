use std::{env, thread};

use  std::time::Duration;

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    
    println!("calculating slow ...");

    thread::sleep(Duration::from_secs(2));

    intensity
} 

#[cfg(test)]
mod tests{
    use std::result;
   
   #[test]
   fn colusure_own() {
       
    let x = 4;
    
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
   }

   #[test]
   fn move_own(){

    let x = vec![1, 2, 3];

    let equal_to_x = move |z: Vec<i32>| z == x;

    //println!("can`t used x {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
    }

    #[test]
    fn make_iterator(){

        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        for iter in v1_iter  {
            
            println!("Gotta: {}", iter);
        }
    }

    #[test]
    fn iterator_demonstartion(){

        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();
        
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]

    fn itemator_sum() {
        
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(6, total);
    }
}
struct Cacher<T>
    where T: Fn(u32) -> (u32)
    {
        calculation: T,
        value: Option<u32>
    }

impl<T> Cacher<T>
    where T : Fn(u32) -> u32
    {

        fn new(calculation: T) -> Cacher<T>{
            
            Cacher{

                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32{

            match self.value{

                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

#[derive(Debug, PartialEq)]


struct Shoe{

    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()//filter迭代适配器和collect()消费适配器同时作用,filter使得闭包返回值为true的才能被collect使用转化为集合
    
}


#[test]

fn filters_by_size() {
    
    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker")},
        Shoe {size: 13, style: String::from("sandal")},
        Shoe {size: 10, style: String::from("boot")}

    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {size: 10, style: String::from("sneaker")},
            Shoe { size: 10, style: String::from("boot")},
        ]
    );
}
struct Counter{

    count: u32,
}

impl Counter {
    
    fn new() -> Counter {
        
        Counter{

            count: 0
        }
    }
}

impl Iterator for Counter{

    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {

        self.count += 1;
        if self.count < 6 {
            
            Some(self.count)
        }else {
            None
        }
    }
}


#[test]

fn using_other_iterstor_trait_methods(){

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    print!("{:?}", sum);
    assert_eq!(18, sum);
}


//性能对比： 循环和迭代器
//迭代器是高级抽象
//迭代器是零成本抽象意味零开销，这使得在符合迭代器的规则下使用，他就是最好的代码和手写代码一样，但是可以直接使用不用再次编写

//展开（unoll）：移除循环控制代码的开销并替换为每个迭代中的重复的代码优化
//优化为：所有的系数都被储存在了寄存器中，这意味着访问他们非常快。这里也没有运行时数组访
//问边界检 查。所有这些 Rust 能够提供的优化使得结果代码极为高效

//rust本身提供高级抽象，rust本身就实现诸多基本操作同时将其封装使用这是rust 的优势，零成本抽象的一部分




