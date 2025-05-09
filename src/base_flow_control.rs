/// 基础控制流
mod b_flow_control {
    /// if 表达式
    #[test]
    fn test_if() {
        // 简单判断
        let a = true;
        if a {
            println!("PASS!")
        } else {
            println!("NO!")
        }

        // 多分支判断
        let b = 100;
        if b > 99 {
            println!("WIN!")
        } else if b == 100 {
            println!("EQ!")
        } else {
            println!("NO LOSE!")
        }

        // 多条件判断
        // 使用 && 和 ||
        if b < 150 && b > 90 {
            println!("I DONT KNOW")
        }

        // 内嵌判断
        if b > 50 {
            if b < 150 {
                println!("P1");
            } else {
                println!("P2");
            }
        }

        // if 赋值
        let c = if b > 50 { 80 } else { 130 };
    }

    /// match 匹配表达式
    #[test]
    fn test_match() {
        enum Traffical {
            RED,
            GREEN,
            YELLOW,
        }

        let light = Traffical::GREEN;

        match light {
            Traffical::YELLOW => {
                println!("SLOW")
            }
            Traffical::GREEN => {
                println!("PASS")
            }
            Traffical::RED => println!("STOP"),
        }

        let maybe = Option::from(&light);

        // match 内嵌
        match maybe {
            Some(lt) => match lt {
                Traffical::YELLOW => {
                    println!("SLOW")
                }
                Traffical::GREEN => {
                    println!("PASS")
                }
                Traffical::RED => println!("STOP"),
            },
            None => println!("HELL NO"),
        }

        // match 赋值 注意类型
        let b = match light {
            Traffical::YELLOW => 1,
            Traffical::GREEN => 2,
            Traffical::RED => 3,
        };
    }

    /// loop 循环表达式
    /// 不断的循环 loop 中的表达式
    /// 直至手动中断或内部中断
    #[test]
    fn test_loop() {
        let mut counter = 0;

        let result = loop {
            counter += 1;
            println!("c:{}", counter);
            if counter == 10 {
                break counter / 2;
            }
        };

        println!("result: {}", result);
    }

    /// while 循环
    /// 具有条件语句 位于循环块开头
    #[test]
    fn test_while() {
        let mut counter = 5;
        // while 中不可以使用braek携带返回值
        while counter > 0 {
            println!("{}", counter);

            counter -= 1;
        }

        println!("LEFTOFF");
    }

    /// for 循环
    /// 常用于遍历对象或集合
    /// 循环具有一定范围
    #[test]
    fn test_for() {
        // 遍历数组
        let arr = [1, 2, 3, 4, 5];
        for ele in arr.iter() {
            println!("ele: {}", ele);
        }

        // 遍历字符串
        let s = "hello";
        for c in s.chars() {
            println!("c: {}", c);
        }

        // 指定范围 start..end 包含start不包含end e.g. 0..4 == 0->1->2->3
        for i in 0..4 {
            println!("i: {}", i);
        }
    }
}
