#[allow(dead_code)]
#[allow(unused_variables)]
fn while_loop(){
    let mut x=1;
    while x<1000 {
        x *= 2;
        if x == 128 { continue }
        println!("x={}", x);
    }
    println!("loop konusu");
    let mut y=1;
    loop // while true
    {
        y*=2;
        println!("y = {}",y);
        if y==1<<10{break;}

    }

}

