#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem;
fn veriTipleri(){
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

    let d:isize = -200;
    let d_boyut= mem::size_of_val(&d);
    println!("d = {} ve boyutu { },bilgisayarınız {} bit mimariye sahiptir.",
             d,d_boyut,d_boyut+8);

    let e:char='g';//_,;+%'!?
    println!("e={} ve boyutu {} byte tır.",e,mem::size_of_val(&e));
    //f32 veya f64 u olamaz varsayılan olarak işaretli IEEE754 ile nan,+- sonsuz değerleri de alabilir.
    let f:f32=2.000000005;
    println!("f={} ve boyutu {} byte tır.",f,mem::size_of_val(&f));

    let g:bool=false;//true
    println!("g= {} ve boyutu {} byte tır.",g,mem::size_of_val(&g));

}
fn aritmetikIslemler(){
    let mut a=2+5+3;
    println!("sonuc a :{}",a);

    a=a+1;//önemli!  -- veya ++ gibi arrtırma veya azaltma Rust da yok.
    a+=1;
    //-=,*=,/=,%= gibi işlemler yapılabilir.
    println!("{}/ {} işleminden kalan= {}",a,4,(a%4));
    let a_kupu =i32::pow(a,3);
    println!("a değeriinin küpü {}",a_kupu);

    let b=2.6;
    let b_kupu=f64::powi(b,3);
    let b_ustu_pi=f64::powf(b,std::f64::consts::PI);
    print!("b {0} üstü 3 :{1} ve {0} üstü pi:{2}",b,b_kupu,b_ustu_pi);


}

fn main() {
    // veriTipleri()
    aritmetikIslemler();


}
