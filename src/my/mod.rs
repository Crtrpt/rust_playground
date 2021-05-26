pub fn func1( a: &mut i32){
    *a=400;
    println!("{}",&a);
}


pub fn func2<T: std::fmt::Display>( a: T){
    println!("泛型函数xxxxxxxxxxxxxx");
    println!("{}",&a);
}

pub fn func3(){
   let u1=U1{
       name:String::from("狗的不行的rust"),
   };
   u1.age();

   println!("rust的名称 age {}",u1.age());
   u1.hello();

   let u2=U2{
        name:String::from("狗的不行的rust2"),
    };
    u2.hello();
}



pub struct U1 {
    pub name:String,

} 

impl U1 {
    fn age(&self) -> i32 {
        return 20;
    }
}

impl Say for U1 {
    fn name(&self) -> String {
        return self.name.clone();
    }

}

pub struct U2 {
    pub name:String,
} 

impl Say for U2 {
    fn name(&self) -> String {
        return self.name.clone();
    }
}



pub trait Say {
    fn name(&self) -> String;
    fn hello(&self)  {
        println!("说话 {}",self.name())
    }
}