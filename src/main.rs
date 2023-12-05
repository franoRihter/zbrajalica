use rand::Rng;
//use std::sync::Barrier;
use std::thread;
use std:: time::{Duration, Instant};
use std::io;

use mysql::*;
use mysql::prelude::*;


fn main(){
//ispis rezultata
/*
    let url = "mysql://root:root@localhost:3306/zbrajalica";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn.query_iter("SELECT id_igre, rezultat FROM rezultat")
    .unwrap()
    .for_each(|row| {
        let r:(i32, i32)=from_row(row.unwrap());
    println!("{}, {}",r.0, r.1);
    });

*/
    //spajanje na bazu
    let url = "mysql://root:root@localhost:3306/zbrajalica";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    //progress bar
    println!("The game starts soon.\n");
    let pb = indicatif::ProgressBar::new(100);
    for _i in 0..100{
        thread::sleep(Duration::from_millis(30));
        pb.inc(1);
    }
    println!("\n\nGo!\n");
    let mut konacni_rezultat = 0;
    let start_time = Instant::now();

    while start_time.elapsed() < Duration::from_secs(60){
    
        let x = rand::thread_rng().gen_range(-100..100);
        println!("{}", x);
        let y = rand::thread_rng().gen_range(-100..100);
        println!("{}", y);
        let a=x+y;
  

        //unos
        let mut unos = String::new();
        io::stdin().read_line(&mut unos).expect("Greska u obradi unosa!");
        println!("This is input {unos}");
        
        //usporedba
        let unos: i32 = unos.trim().parse().expect("Please type a number!");
        if a==unos{
            println!("Correct");
            konacni_rezultat=konacni_rezultat+1;
        }else{
            println!("Wrong");
        }
    }


    let mut id_zadnjeg_rezultata:i32 = 0;

    conn.query_iter("SELECT id_igre, rezultat FROM rezultat")
    .unwrap()
    .for_each(|row| {
        let r:(i32, i32)=from_row(row.unwrap());
        println!("{}, {}",r.0, r.1);
        id_zadnjeg_rezultata=r.0;
    });
    
    //println!("Rezultat: {}",id_zadnjeg_rezultata);
    println!("Result: {}",konacni_rezultat);

    conn.exec_drop("INSERT INTO rezultat (id_igre, rezultat) VALUES (:id_igre, :rezultat)",
    params!{
        "id_igre" => id_zadnjeg_rezultata+1,
        "rezultat" => konacni_rezultat,
    }
    ).unwrap();

}