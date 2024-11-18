//use std::f32::consts::E;
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, usize};
use std::thread::JoinHandle;

//线程池核心DBS
pub struct ThreadPool {
    workers: Vec<Worker>,
    //线程工作传递信息
    sender: mpsc::Sender<Message>,
}
//处理消息DBS。定义了线程池中线程可以处理的两种消息类型
enum Message {
    NewJob(Job),
    Terminate,
}
//pub struct Job;

//定义一个动态分发闭包类型
//Box<>包含一个指针，指向堆栈用于存储不同类型的闭包
//FnOnce()表示闭包只会被闭包只会被调用一次，并可能消耗其捕获的值。
//'static 表示闭包及其捕获的值的生命周期是 'static，即可以在程序运行期间安全地存活。
//表示闭包可以在线程之间安全传递.Send 是 Rust 的线程安全
//标记，用于保证类型在多线程环境下可以安全移动
type Job = Box<dyn FnOnce() + Send + 'static>;

//等待工作线程定义
pub struct Worker {
    //使用Option包裹JoinHandle<()>（所有权类型），
    //由于Option的值为Some（T）和None使得其值可以被取出，原来位置的数值变为None
    //数值转移但是所有权没有移动
    sever_threads: Option<thread::JoinHandle<()>>,
    id: usize,
}

//创建能够通过通道接收消息工作的线程
impl Worker {
    pub fn new(wrok_id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let threads = thread::spawn(move || {
            // loop {
            //     let job = receiver.lock().unwrap().recv().unwrap();

            //     println!("Worker {} got a job; executing.", wrok_id);

            //     job();
            // }
            //线程会在循环中不断接收和处理 Message 枚举
            // 一直监测是否有消息传入
            loop {
                //只允许一个线程接收到消息，并执行对应处理
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Woker {} got a job; executing.", wrok_id);
                        job();
                    },
                    //等待工作的线程直接退出
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", wrok_id);
                        break;
                    },
                }
            }
        });

        Worker {
            
            sever_threads: Some(threads),
            id: wrok_id,
        }
    }  
}
//借助wroker实现一次创建多个能接听消息的线程
impl ThreadPool {
    /// 创建线程池
    /// 
    /// 线程池中线程的数量
    /// 
    /// # Panic
    /// 
    /// 'new' 函数在size为0时会panic
    //pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> { 
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        //用于存储创建的worker对象
        let mut workers = Vec::with_capacity(size);
        //创建通道，用于主线程和工作线程之间的信息交换。基于一个生产者多个消费者
        let (sender, receiver) = mpsc::channel();
        //使用智能指针使得一个不可借用的变量可以同一时间被一个进程修改借用。
        //即多个线程都能访问和修改值，但是同一时间只能有一个线程修改和访问
        let receiver = Arc::new(Mutex::new(receiver));
        //根据参数size创建等数量的等待的工作线程。将每个创建的对象压入wokers队列（wokers本身是向量所以可以这样操作）
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        //返回向量worker作为工作线程字段
        ThreadPool{
            workers,
            sender,
        }
    }
    //发送信息到等待工作的线程
    pub fn execute<F>(&self, f: F)
        where 
        //指定F实现的trait bounds
            F:FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        //self.sender.send(job).unwrap();
        //通知等待线程来活了
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

//为thread实现Drop trait
//自动回收工作线程

impl Drop for ThreadPool {
    //离开工作域后为每个等待或者工作的线程发送Message->Terminate的信号
    fn drop(&mut self) {
        println!("Sending terminate message to all worker.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        //之前工作的线程被加入Drop的队列当中。自动被回收
        for work in &mut self.workers {
            println!("Shutting down worker {}", work.id);
            //使用模式匹配可以对Option取出的值，加入join（）取出值就被消耗（所有权进行了转移）
            //Option定义变量就适应于一次性消耗的资源
            //满足了Rust对借用检查规则，也对可变引用的修改
            //take用于拿出sever_threads的值
            if let Some(sever_thread) = work.sever_threads.take() {
                sever_thread.join().unwrap();
            }
        }
    }
}