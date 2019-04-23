use std::error::Error;
use std::fs;

fn run(conf: Config) -> Result<(), Box<dyn Error>>{    //对于错误类型返回trait对象， dyn是动态dynamic的缩写

    let contents = fs::read_to_string(conf.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config{
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {        //注意这里接受vector参数可以使用&[type]的形式

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();      //使用字符串的复制，在这里是最舒服的

        Ok(Config { query, filename })      //表达式返回值
    }
}
