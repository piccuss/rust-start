# 结构体

```
//定义结构体
struct Rectangle {
    width: u32,
    height: u32,
}

//实例化结构体
let rect1 = Rectangle {
    width: 30,
    height: 50,
};

//用已存在的实例进行实例化
let rect2 = Rectangle {
    width: 40,
    ..rect1
};

//方法定义
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

//无实例方法
impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle {
            width: size,
            height: size,
        }
    }
}
Rectangle::square(3);
```