#[allow(dead_code)]
#[allow(unused_variables)]
fn match_case(){
    let ulke_kodu =90;
    let ulke= match ulke_kodu {
        44=>"UK",
        46=>"Sweden",
        7=>"Russia",
        90=>"Turkiye",
        1..=1000=>"Unkown",
        _=>"invalid"
    };
    println!("the country code{} is {}",ulke_kodu,ulke);
let x=false;
    let deger=match x {
        false=>"yanlış",
        _=>"dogru"
    };
    println!("{}",deger);

}