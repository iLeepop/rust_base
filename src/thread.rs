/// 线程
mod thread {
    // 标准线程库
    use std::{thread, time::Duration};

    fn run1() {
        for i in 0..100 {
            println!("index: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    /// 线程 join 的调用时机
    #[test]
    fn test_base_thread() {
        // 使用闭包
        let t = thread::spawn(|| {
            // 线程循环10次
            for i in 0..10 {
                println!("thread: {}", i);
            }
        });

        // 使用 join 函数等待线程执行完成
        t.join().unwrap();

        // 主线程循环5次
        for i in 0..5 {
            println!("main thread: {}", i);
        }
        
        // 线程执行完成后 主线程才开始循环体
    }

    /// 线程 join 的调用时机
    #[test]
    fn test_base1_thread() {
        // 使用闭包
        let t = thread::spawn(|| {
            // 线程循环10次
            for i in 0..10 {
                println!("thread: {}", i);
            }
        });

        // 主线程循环5次
        for i in 0..5 {
            println!("main thread: {}", i);
        }
        
        // 使用 join 函数等待线程执行完成
        t.join().unwrap();
    }

    /// 线程 sleep
    #[test]
    fn test_base2_thread() {
        // 使用闭包
        let t = thread::spawn(|| {
            // 线程循环10次
            for i in 0..10 {
                println!("thread: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // 主线程循环5次
        for i in 0..5 {
            println!("main thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        
        // 使用 join 函数等待线程执行完成
        t.join().unwrap();

        // 执行结果会有交叉现象
    }

    /// 使用函数代替闭包函数
    #[test]
    fn test_base3_thread() {
        // 使用函数
        let t = thread::spawn(run1);

        // 主线程循环5次
        for i in 0..100 {
            println!("main thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        
        // 使用 join 函数等待线程执行完成
        t.join().unwrap();
    }

    /// 线程使用外部对象 move 关键字
    #[test]
    fn test_base4_thread() {
        // 所有权
        let arr = vec![1, 2, 3];
        // 线程使用数组
        let t = thread::spawn(move || {
            println!("{:#?}", arr);
        });

        // 无法 drop， 没有所有权
        // let _ = drop(arr);

        t.join().unwrap();
    }
}