use std::env;

fn divise(a: i32, b:  i32) -> Option<i32> {

    if b == 0 {
        None
    }else{
        Some(a/b)
    }
}

fn div(a: i32, b: i32) -> Option<i32>{

    if b == 0 {
        None
    }else{
        Some(a/b)
    }
}

fn main(){

    let a : i32 = 20;
    let b : i32 = 10;
    
    let args: Vec<String> = env::args().collect();

    let text1 = &args[0];
    let text2 = &args[1];

    let c: i32 = text1.parse().unwrap();
    let d: i32 = text2.parse().unwrap();

    match divise(a, b){
        None => println!{"Eroare, nu se poate imparti la 0"},
        Some(rez) =>(
            println!{"{} / {} = {}", a, b, rez}
        )
    }

    match div(c, d){
        None => return -1,
        Some(rez) =>(
            println!{"{} / {} = {}", c, d, rez}
        )
    }

}