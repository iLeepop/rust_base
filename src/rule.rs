/// 宏
mod macros {
    /// 一个简单的宏
    macro_rules! oops {
        ($name: ident) => {
            println!("oops! is {}!", $name);
        };
    }

    /// 提供返回值 尽量使用大括号包裹
    macro_rules! ten {
        () => {
            {
                // 根据需求提供自定义过程
                10 // 值返回出去
            }
        };
    }

    /// 读取表达式
    macro_rules! next {
        // 匹配
        ($( $x:expr ),*) => {
            // 生成
            $(
                println!("next is {}.", $x);
            )*
        };
    }

    /// expr 与 ident 的区别

    #[test]
    fn test_base_rule() {
        let name = "ilee";
        oops!(name);
        /* 
            会展开为：
            println!("oops! is {}!", name);
         */
        // 类型不定
        let num = 123;
        oops!(num);
        /* 
            会展开为：
            println!("oops! is {}!", num);
         */
    }

    #[test]
    fn test_return_rule() {
        let num = ten!(); // num = 10
        oops!(num);
        /* 
            会展开为：
            println!("oops! is {}!", num);
         */
    }

    #[test]
    fn test_expr_rule() {
        next!(1, 2, 3, 4, 'a');
        /* 
            会展开为：
            println!("next is {}.", 1);
            println!("next is {}.", 2);
            println!("next is {}.", 3);
            println!("next is {}.", 4);
            println!("next is {}.", 'a');
         */
    }
}