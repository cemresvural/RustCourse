#![allow(dead_code)]
use std::mem;
 struct Point{
     x:f64,
     y:f64
 }
fn orjin()->Point{
    Point{x:0.0,y:0.0}
}
pub fn stack_and_heap(){
    let p1=orjin();
    let p2=Box::new(orjin());


    println!("p1 boyut olarak {} yer igal eder.",mem::size_of_val(&p1));
    println!("p2 boyut olarak {} yer igal eder.",mem::size_of_val(&p2));


    let p3= *p2;
    println!("değer:{}",p3.y);
}
