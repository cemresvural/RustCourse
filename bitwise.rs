#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

fn bitwise_logic(){
    let c=1|2;
    //  | -> veya
    // & AND
    //^ ->XOR
    // 1|2  -> 1=01
    //2 -> 10
    // 01| 10 =11
    println!(" 1|2 ={}",c);
//logical işlemler
    let pi_kucukmu_4den=std::f64::consts::PI<4.0;//true
    // mantıksal olarak >,>=,<,<=,==, != sorgulaması yapabiliriz.
    println!(" sonuc {}",pi_kucukmu_4den);

    let x=5;
    let x_esit_5=x==5;
    println!("eşitlik :{}",x_esit_5);

}
fn main(){
    bitwise_logic();
}