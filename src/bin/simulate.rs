extern crate pinknoise;

use clap::{
    App
    ,Arg    
};

use std::{
    fs::{
        File
    }
    , io::{
        Write
        ,BufWriter
    }
};

use pinknoise::{
    VmPinkRng
    , RandVmPinkRng
};

use rand::{
    thread_rng
};

fn main()->Result<(), std::io::Error>{
    let matches=App::new("simulate 1/f noise")
    .arg(
        Arg::new("order")
        .short('r')
        .long("order")
        .takes_value(true)
        .value_name("order")
        .required(true)
        .about("order")
    )
    .arg(
        Arg::new("length")
        .short('l')
        .long("len")
        .takes_value(true)
        .default_value("65536")
        .value_name("length")
        .required(false)
        .about("dta length")
    )
    .arg(
        Arg::new("output")
        .short('o')
        .long("out")
        .takes_value(true)
        .value_name("outfile")
        .required(true)
        .about("output file")
    )
    .get_matches();

    let length=matches.value_of("length").unwrap().parse::<usize>().unwrap();
    let mut outfile=BufWriter::new(File::create(matches.value_of("output").unwrap())?);
    let order=matches.value_of("order").unwrap().parse::<usize>().unwrap();
    let mut rng=thread_rng();
    let mut vmpn=RandVmPinkRng::<f64>::new(order, &mut rng);
    
    for _ in 0..length{
        let x=vmpn.get(&mut rng);
        let bytes=x.to_le_bytes();
        outfile.write(&bytes)?;
    }

    Ok(())
}
