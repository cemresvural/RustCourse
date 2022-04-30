#[allow(dead_code)]
#[allow(unused_variables)]

fn if_statement(){
    let sicaklik=30;
    if sicaklik>35
    {
        println!("gerçekten çok sıcak");
    }
    else if sicaklik<10
    {
        println!("hava soğuk");
    }
    else {
        println!("hava şartlari ideal");
    }

    let gun=if sicaklik>20 {"günesli"} else{"bulutlu"};
    println!("bugün {}",gun);


    println!("hava durumu :{}",
             if sicaklik>20 {"sıcak"} else if sicaklik<10 {"soğuk"} else{"ideal"});

}