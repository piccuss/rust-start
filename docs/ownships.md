# 所有权

## 所有权规则
* Rust里的每个值都有对应的绑定,即对应的变量名;
* 同时只能存在一个绑定;
* 当绑定超出作业域时，值将被丢弃.

```
//创建hello的绑定s
let s = String::from("hello");

//移交绑定
let t = s;

//方法调用也会移交绑定
do_something(t);
```

## 引用规则
* 任意时间内,你只能拥有一个可变引用，或有任意数量的不可变引用;
* 引用必须是有效的

```
//创建引用进行函数调用,不会移交绑定,此过程皆为借用
do_something(&s);

//创建可变引用,允许借用时更改值
do_something_change(&mut s);
```

## 切片

```
let s = String::from("hello");
//指定切片 1 -> 3
let slice = &s[1..3];
//切片 1 -> len
let slice = &s[1..];
//切片 0 -> 3
let slice = &s[..3];
//切片 0 -> len
let slice = &s[..];
```