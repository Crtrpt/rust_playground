mod my;

fn main() {
    let mut a=20;
    let  b=30;
    my::func1(&mut a);
    my::func2::<&mut i32>(&mut a);

    my::func3();
    println!("{},{}",a,b);
}


