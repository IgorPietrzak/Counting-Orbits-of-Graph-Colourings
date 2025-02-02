use super::Algorithm;

pub fn log(alg: &Algorithm) {
    println!("====================================================================");
    println!("\n\n\n-----------------------------------------------------------");
    println!("CURRENT STATE");
    println!("-----------------------------------------------------------\n");
    println!("INFECTION SET: \n");
    for i in alg.infection_set.clone() {
        println!("* {:?}\n", i);
    }
    println!("-----------------------------------------------------------\n");
    println!("ORBIT REPS: \n");
    for i in alg.orbit_reps.clone() {
        println!("* {:?}\n", i);
    }
    println!("-----------------------------------------------------------\n");
    println!("CURRENT REPS: \n");
    for i in alg.current_reps.clone() {
        println!("* {:?}\n", i);
    }
    println!("-----------------------------------------------------------\n");
    println!("N: \n");
    println!("{:?}", alg.n);
    println!("-----------------------------------------------------------\n");
    println!("PI");
    alg.pi.print();
    println!("-----------------------------------------------------------");
    println!("\n\n\n====================================================================");
}

pub fn quiet_log(alg: &Algorithm) {
    println!("====================================================================");
    println!("ORBITS FOUND SO FAR:{:?}", alg.orbit_reps.len());
    println!("====================================================================");
}
