# 基本用法

```
let x= 5;      //不可变变量

let mut x = 5; //可变变量

const MAX_POINTS: u32 = 100_000;  //常量,只允许常量表达式(不受运行时影响)赋值

let spaces = "   ";
let spaces = spaces.len();  //允许重复定义,减少变量数

let x = 2.0; // 默认f64

let y: f32 = 3.0; // f32

let tup: (i32, f64, u8) = (500, 6.4, 1); //元组
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("tup = ({}, {}, {})", tup.0, tup.1, tup.2); //通过索引取值

let a = [1, 2, 3, 4, 5]; //数组,长度不可变
let a: [i32; 5] = [1, 2, 3, 4, 5];  //定义类型
let a = [3; 5];    //值完全相同, =[3, 3, 3, 3, 3]

let y = 6;  //statement 
{
    let x = 3;
    x + 1
}      //expression,句尾没有分号

fn plus_one(x: i32) -> i32 {
    x + 1
} // 函数返回类型用 -> 标记,同时返回的内容为expression

//循环 
let mut count = 0;
let result = loop {
    count += 1;

    if count == 10 {
        break count * 2;
    }
};

//for 循环
let array = [10, 20, 30, 40];
for element in array.iter() {
    println!("Element value is {}", element);
}
for number in (1..4).rev() {
    println!("{}!", number);
}
```