/// 泛型
mod generic_type {
    use std::fmt::Debug;

    /// T 代指泛型
    /// 可以用作任何不确定类型的类型声明
    #[derive(Debug)]
    struct GT<T> {
        member: T,
    }

    /// impl 实现泛型方法
    impl<T: Debug> GT<T> {
        fn new(member: T) -> Self {
            GT { member }
        }

        fn set_member(&mut self, m: T) {
            self.member = m;
        }
    }

    #[test]
    fn test_generic_type() {
        let m = "String";
        let gt_s = GT::new(m);

        let mm = 16;
        let gt_n = GT::new(mm);

        println!("{:#?}", gt_s);
        println!("{:#?}", gt_n);
    }
}
