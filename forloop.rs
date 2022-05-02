#[allow(dead_code)]
#[allow(unused_variables)]
 fn forloop(){
 for x in 1..11{
  if x==3 {continue};
  if x==8 {break}
  println!("{}",x);
 }



 println!("------enumerate-------");
 for (poz,y)in (50..61).enumerate(){
  println!("{}. indeksteki değerin iki katı : {}",poz+1,y*2);
 }
}

