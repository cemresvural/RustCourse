#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;
const VERI:i8=23;//belirli bir adresi yok
static  mut m:i8=22;//mutable olabilir unsafe ile

fn main() {
let a=44;
    {
        let a=56;
        println!("dongunun içi a:{}",a);

    }
println!(" dongunun disi a:{}",a);
    println!("veri:{}",VERI);

    unsafe {
        m=66;
    }
    println!("m:{}",m);

}