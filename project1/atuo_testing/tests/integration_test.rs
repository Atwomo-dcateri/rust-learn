use atuo_testing;
mod common;

#[test]
fn it_adds_two() {
    
    common::setup();
    assert_eq!(4, atuo_testing::add_two(2));
}
//每个测试部分的都会有对应的测试结果，位于target/debug/dep中

//但是还是可以通过指定参数运行指定tests目录下的测试文件的所有测试
//cargo test -- --test check_module_name



//集成测试中的子模块
//将功能相似的测试和为一个模块，方便我们后期的管理，同时由于tests中的每个文件都是一个可以独立编译的
//crate但是tests中的文件不存在共享行为
//对于在tests中被公共调用的部分模块得采用一种规范的语法才能使其被各个模块调用的同时，有不会在测试时被单独编译
//eg；tests/common/mod.rs（公共部分的内容得这样声明）


