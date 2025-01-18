# 介绍
`regex_capture`是一个宏，用于根据正则表达式为结构体实现`FromStr` trait.

# 用法
示例如下:
```rust
#[derive(PartialEq, Debug)]
enum Gender {
    Male,
    Female,
}

fn gender_parser(s: &str) -> Result<Gender, String> {
    match s {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err("invalid gender".to_string()),
    }
}

#[derive(RegexCapture)]
#[converter(regex = r"name=(?P<name>.+?), age=(?P<age>\d+), gender=(?P<gender>male|female)")]
struct Person {
    name: String,
    age: u32,
    #[converter(func = gender_parser)]
    gender: Gender,
}
```
其中, `#[converter(regex = ...)]`指定一个正则表达式, 该表达式必须包含和结构体中所有字段同名的捕获;
`#[converter(func = ...)]`为特定字段指定转换函数, 该函数类型为`&str -> Result`, 用于将正则表达式的捕获转换为字段类型.
未指定转换函数的字段, 会调用`parse`方法, 你也可以自行为其类型实现相应trait;

之后, 你可以使用如下方法来将字符串解析为结构体:
```rust
    static PERSON_STR: &str = "name=Chihaya Anon, age=15, gender=female";
    let pre = Person::from_str(PERSON_STR).unwrap();
    assert_eq!(pre.gender, Gender::Female);
    assert_eq!(pre.age, 15);
    assert_eq!(pre.name, "Chihaya Anon");
```
