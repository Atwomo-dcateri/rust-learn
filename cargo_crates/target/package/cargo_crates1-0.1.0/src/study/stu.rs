//chapter14 more cargo and crate.io


//14.01 采用发布配置自定义构建


/*
自定义构建

cargo build

采用默认配置：dev 配置去构建编译项目，用于开发时，多次编译。注意dev配置是一类配置的总称，这一类配置包括多个参数

eg：

[profile.dev]
opt.level = 0;
在使用dev配置时构建项目采用最低级代码优化设置

cargo build --release

发布版本配置：release有着良好发布构建的配置，一般会对多个配中的参数进行重新设定，而不采用dev配置中的参数设置

release配置下只编译一次而使用多次，一般采取使代码性能高的参数设置，但是一次编译的时间会更长

*/
