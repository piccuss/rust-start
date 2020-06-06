# 枚举与match匹配

## 枚举

```
//枚举定义
enum IpAddr {
    V4,
    V6,
}

//枚举支持不同结构定义
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//枚举方法定义
impl IpAddr {
    fn call(&self) {
        //method body    
    }
}

//泛型枚举
enum Option<T> {
    Some(T),
    None,
}
```

## match匹配

```
//match匹配相应值进行=>后的语句
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//match取出枚举里的值
let opt = Option::Some(8);
let result = match opt {
    Some(t) => t,
    None => 0,
};

//match匹配使用_表示剩余类型
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

## if let匹配
```
//if let用来只匹配一种类型
let some_u8_value = Some(0u8);
if let some_u8_value = Some(3) {
    println!("three")
};

//使用else表示剩余类型
if let some_u8_value = Some(3) {
    println!("three")
} else {
    //do something else.    
}
```