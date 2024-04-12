use lazy_static::lazy_static;

// 运行时，在需要的地方只进行一次耗时计算进行静态数据分配，后续可以直接使用
fn main() {
    lazy_static!{
        static ref VEC_VALUES:Vec<i32> = {
            println!("init in lazy");
            let mut vs:Vec<i32> = Vec::new();
            for i in (0..=10) {
                vs.push(i);
            }
            vs
        };
    }

    println!("main start");

    // 可以发现在这里才开始初始化VEC_VALUES的数据
    for value in VEC_VALUES.iter() {
        print!("{:?} ", value);
    }

    println!();

    // 再次使用VEC_VALUES，没有重复执行初始化
    for value in VEC_VALUES.iter() {
        print!("{:?} ", value * 2);
    }
}