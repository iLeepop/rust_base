/// trait
mod traits {
    /// 标准库 trait 和 宏 macro
    use std::fmt::{Debug, Display, Result};

    /// 自定义对象
    #[derive(Debug)]
    struct Obj<T> {
        name: String,
        value: T,
    }

    /// 实现基本方法
    impl<T: Debug> Obj<T> {
        fn new(value: T) -> Self {
            Obj {
                name: String::from("Default"),
                value,
            }
        }
    }

    /// 自定义显示 trait
    trait Show {
        fn show(&self);
    }
    /// 实现自定义显示 trait
    impl<T: Debug + Display> Show for Obj<T> {
        fn show(&self) {
            println!("{:#?}", self);
        }
    }

    /// 实现标准库 Display trait
    impl<T: Debug + Display> Display for Obj<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "(name: {}, value: {})", self.name, self.name)
        }
    }

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Self {
            Rectangle {
                width,
                height
            }
        }
    }

    /// 高级 trait
    trait Compare<Rhs=Self> {
        /// 关联类型
        type Item;
        /// 可以声明多个关联类型
        type output;
        
        fn cmp(&self, other: &Rhs) -> Self::Item;
    }

    impl Compare<Rectangle> for Rectangle {
        type Item = bool;
        type output = bool;
        fn cmp(&self, other: &Rectangle) -> Self::Item {
            self.width * self.height > other.width * other.height
        }
    }

    /// 测试实现自定义特征 trait 与实现标准特征
    #[test]
    fn test_trait() {
        let o1 = Obj::new("String");
        let o2 = Obj::new(0xff);

        o1.show();
        o2.show();

        println!("{}", o1);
    }

    /// 实现高级特征用法
    #[test]
    fn test_adv() {
        let r1 = Rectangle::new(10, 10);
        let r2 = Rectangle::new(20, 20);

        // assert!(r1.cmp(&r2)); // 失败
        assert!(r2.cmp(&r1));
    }
}
