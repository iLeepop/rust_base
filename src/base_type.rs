/// 基础类型
mod b_types {
    /// integers 整数类型
    /// signed 有符号 能够表达负数
    pub struct SignedIntegers {
        bit8: i8,
        bit16: i16,
        bit32: i32,
        bit64: i64,
        bit128: u128,
        arch: isize,
    }
    /// unsigned 没有符合 只能表达非负数
    pub struct UnsignedIntegers {
        bit8: u8,
        bit16: u16,
        bit32: u32,
        bit64: u64,
        bit128: u128,
        arch: usize,
    }
    /// Numeral System 数字进制
    pub enum Numeral_System {
        /// 十进制 示例: 98_001
        Decimal,
        /// 十六进制 以0x为前缀 示例: 0xff
        Hexadecimal,
        /// 八进制 以0o为前缀 示例: 0o66
        Octal,
        /// 二进制 以0b为前缀 示例: 0b1100_0010
        Binary,
        /// 字节 只能是u8 以b为前缀 示例: b'A'
        Byte
    }

    /// float_point 浮点类型
    /// 主要两种 f32 f64
    struct FloatPoint {
        /// f32 单精度浮点数
        f32a: f32,
        /// f64 双精度浮点数 速度与f32差不多，但是精度更高，在现代CPU中支持更好
        f64a: f64,
    }

    /// boolean 布尔值 布尔类型
    struct Boolean {
        t: bool,
    }

    /// character 字符类型 由四字节组成
    struct Char {
        c: char,
    }

    /// Compound 组合类型
    /// 元组 长度固定 一旦被创建就无法创建或删除其中的元素
    /// 可以包含不同的数据类型
    /// 可以解构 (a, b, c) = abc;
    /// 可以通过索引访问 e.g. abc.0;
    struct Tuples {
        /// 这就是一个包含两个不同数据类型的元组
        t: (i32, f64),
        /// 这是一个包含三个不同数据类型的元组
        tt: (i32, f32, u8)
    }

    /// 数组 长度固定 数组大小在编译时就被确定
    /// 数组被分配在栈上
    struct Array {
        /// [数据类型; 数组长度]
        arr: [i32; 5],
    }

    /// 自定义类型
    /// struct 结构体 自定义数据类 组合并命名多个有关联属性的值，组成一个具有意义的组合
    /// 类似于元组 但是每个值都具有命名字段
    /// 上面的所有结构都是自定义属性 不必再示例
    struct Ex {
        n: u8,
    }
    
    /// 自定义类型
    /// enumerations 枚举类型
    enum In {
        // 第一个枚举默认为 0
        A,
        // 后面依次递增 D为1
        D,
        // 修改某个值为 4
        B = 4,
        // 后面的数会在上一个数的基础上递增 C为5
        C,
    }

    /// 有符号整数示例测试
    #[test]
    fn test_signed_integers() {
        // 使用MAX和MIN可以获取该类型的最大最小值
        let si8 = i8::MAX;
        let si32: i32 = -18;
        // 默认数字类型为 i32, 大于i32::MAX的数值必须手动强调类型
        let si32_a = 900;
        let si128: i128 = 230_900_033_120;

        println!("{:#?}", si8);
        println!("{:#?}", si32);
        println!("{:#?}", si32_a);
        println!("{:#?}", si128);
    }

    /// 无符合整数示例测试
    #[test]
    fn test_unsigned_integers() {
        // 使用MAX和MIN可以获取该类型的最大最小值
        let su8 = u8::MAX;
        // unsigned 不可以为负数
        let su32: u32 = 18;

        println!("{:#?}", su8);
        println!("{:#?}", su32);
    }

    /// 进制表达示例测试
    #[test]
    fn test_numeral_system() {
        // 注意默认类型为 i32
        let decimal = 98_001;
        //
        let hex = 0xff;
        //
        let octal = 0o66;
        //
        let binary = 0b1100_0010;
        // 注意字节类型为 u8
        let byte = b'A';

        println!("{:#?}", decimal);
        println!("{:#?}", hex);
        println!("{:#?}", octal);
        println!("{:#?}", binary);
        println!("{:#?}", byte);
    }

    /// 浮点数示例测试
    #[test]
    fn test_float_point() {
        //
        let f32aa: f32 = 1.0;
        // 浮点数类型默认为 f64
        let f64aa = 2.0;

        // 可以为负
        let f32_max = -f32::MAX;

        println!("{:#?}", f32aa);
        println!("{:#?}", f64aa);
        println!("{:#?}", f32_max);
    }

    /// 布尔值示例测试
    #[test]
    fn test_boolean() {
        // true/false 关键字默认为bool类型
        let t = true;
        let f: bool = false;

        assert_eq!(t, true);
        assert_ne!(f, true);
    }


    /// 字符示例测试
    #[test]
    fn test_character() {
        let c = 'z'; // ASCII 码
        let z = '𝕏'; // Unicode 
        let heart_eyed_cat = '😻'; // Emoji

        println!("{}, {}, {}", c, z, heart_eyed_cat);

        for char in "Hello, 世界! 🚀".chars() {
            println!("{}", char);
        }
    }

    /// 元组示例测试
    #[test]
    fn test_tuple() {
        let abc = (1, 1.0, 'A');
        let (a, b, c) = abc;
        let _a = abc.0;
        let _b = abc.1;
        let _c = abc.2;

        println!("{:#?}", abc);
        println!("{:#?}", a);
        println!("{:#?}", b);
        println!("{:#?}", c);

        assert_eq!(a, _a);
        assert_eq!(b, _b);
        assert_eq!(c, _c);
    }

    /// 数组示例测试
    #[test]
    fn test_array() {
        // 注意看类型
        let arr = [1, 2, 3, 4, 5];

        let first = arr.first();
        println!("firts:{}", first.unwrap());
        let one = arr[0];
        let second = arr[1];
        println!("one:{}, second{}", one, second);

        for elem in arr.iter() {
            println!("Value: {}", elem);
        }
    }

    /// 结构示例测试
    #[test]
    fn test_struct() {
        struct Dog {
            name: String,
            age: u8,
        }
        // 注意看类型
        let dog = Dog {
            name: String::from("Mimi"),
            age: 3,
        };

        println!("Dog's Name:{}", dog.name);
        println!("Dog's Age:{}", dog.age);
    }

    /// 枚举示例测试
    #[test]
    fn test_enum() {
        let alrbet = In::B;

        // 经常用于match
        match alrbet {
            In::A => println!("A"),
            In::D => println!("D"),
            In::B => println!("B"),
            In::C => println!("C"),
        }
    }
}