use std::thread;
use std::time::Duration;
fn main() {
    
    //generate_workout(13, 7);

    conmuse_iter();
  
}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    
    println!("calculating slow ...");

    thread::sleep(Duration::from_secs(2));

    intensity
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

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num|{

        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });//慢计算只进行一次,而结果能被复用多次,未被移出作用域都是有效的
    if intensity < 25{
        println!(
            "Today, do {} pushps!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity));
    }else {
        if random_number == 3 {

            println!("Take a break today! Remember to stay hybrated!");
        }else {
            
            println!(
                "Today, run for {} mintues!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn conmuse_iter(){

    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    
}
