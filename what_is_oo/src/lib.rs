use gui::Draw;
pub trait Draw {
    fn draw(&self) {
        
    }
}


pub struct Screen<T: Draw>{
    pub componets: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw{
    pub fn run(&self) {
        for components in self.componets.iter() {
            components.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}


impl Draw for Button {
    
    fn draw(&self) {
        
    }
}

pub struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,    
}


impl Draw for SelectBox {
    fn draw(&self) {
        
    }
}

/*pub struct Post {

    state: Option<Box<dyn State>>,
    content: String::new(),
}

impl Post {
    fn new() -> Post {
        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_test(&mut self, text: &str) {

        self.content.push_str(text);
    }

    pub fn content(&self) -> &str{

        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())
        }
    }

    pub  fn approve(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str{

        ""
    }
}//通过3个trait实现了Post值与状态的分离，实现面向对象继承和封装的特性

struct Draft {

}

impl State for Draft {
    fn request_review(self: Box<Self>) ->Box<dyn State> {
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
}

struct PendingReview{

}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
}   

struct Published{

}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        Box::new(Published{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }

    fn  content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}*/


//实现状态转移为不同类型的转换

pub struct Post{

    content: String,
}


pub struct DraftPost {

    content: String,
}


impl Post {
    pub fn new () -> DraftPost {
        DraftPost{
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str{
        &self.content
    }
}


impl DraftPost {
    pub fn add_text(&mut self, text: &self) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost{
            content:self.content,
        }
    }
}

pub struct PendingReviewPost{
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post{
        Post {
            content: self.content,
        }
    }
}