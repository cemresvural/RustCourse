#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem;
fn main() {
    let a: u8 = 125; //u =unsigned 0-255 8 bit
    println!("a = {}",a);

    //  unsigned 0-2^N-1 e kadar
    let  mut b:i8 = 0;//-128---127
    // i signed,-2^(N-1) ---- 2^(N-1)-1

    println!("önce  b = {}",b);
    //immutable ı mutable yapmak için mut yazarız.
    b=23;
    println!("sonra b ={}",b);

    let c=123456789;//i32 == 32 bit=4 byte
    println!("c ={} ve boyutu {} bytedır.",c,mem::size_of_val(&c));
    //boyut bilgisini almak için kütüphane çağır.


}
