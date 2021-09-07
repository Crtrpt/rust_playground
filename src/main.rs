mod v;

fn main() {
    let p= v::P{
            name:String::from("用户名"),
    };
    println!("{}", p.name());
}


