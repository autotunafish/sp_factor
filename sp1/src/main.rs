//No 1
//path combinations 1 and 7

//For benchmarking
use std::time::Instant;

use std::io;

//These are arrays of primes belonging to the MOD path to increase the check speed.
//The rmod# files are sibling in the src.
pub mod rmod1;
use rmod1::LMOD1ARR;

pub mod rmod7;
use rmod7::LMOD7ARR;

pub mod rmod11;
use rmod11::LMOD11ARR;

pub mod rmod13;
use rmod13::LMOD13ARR;

pub mod rmod17;
use rmod17::LMOD17ARR;

pub mod rmod19;
use rmod19::LMOD19ARR;

pub mod rmod23;
use rmod23::LMOD23ARR;

pub mod rmod29;
use rmod29::LMOD29ARR;

//Starts the program :p
fn main() {
    //Get the semiprime to factor
    println!("ENTER THE SEMIPRIME TO FACTOR:\n");
    let mut prime_input = String::new();
    io::stdin().read_line(&mut prime_input).unwrap();
    //Starts the benchmark
    let now = Instant::now();
    //pop newline
    prime_input.pop();
    let height = prime_input.parse::<u128>().unwrap();

    //Subtract the 36 products of the MOD path orders 1, 7, 11, 13, 17, 19, 23, 29 and
    //assign each a variable. This is done for thoughroughness as the overlap in
    //the product MODs reduces the check to only these very eight values needing
    //subtracted.

    //Create a vector to collect the (excessive) values
    let mut z0vektor = vec![];

    {
        let mut z1vektor = vec![];
        let mut z7vektor = vec![];
        let mut z11vektor = vec![];
        let mut z13vektor = vec![];
        let mut z17vektor = vec![];
        let mut z19vektor = vec![];
        let mut z23vektor = vec![];
        let mut z29vektor = vec![];

        //Products of 1 and the rest
        let z1mod1 = (height - 1) % 30;
        z1vektor.push(z1mod1.clone());
        z0vektor.push(z1mod1);
        let z7mod1 = (height - 7) % 30;
        z7vektor.push(z7mod1.clone());
        z0vektor.push(z7mod1);
        let z11mod1 = (height - 11) % 30;
        z11vektor.push(z11mod1.clone());
        z0vektor.push(z11mod1);
        let z13mod1 = (height - 13) % 30;
        z13vektor.push(z13mod1.clone());
        z0vektor.push(z13mod1);
        let z17mod1 = (height - 17) % 30;
        z17vektor.push(z17mod1.clone());
        z0vektor.push(z17mod1);
        let z19mod1 = (height - 19) % 30;
        z19vektor.push(z19mod1.clone());
        z0vektor.push(z19mod1);
        let z23mod1 = (height - 23) % 30;
        z23vektor.push(z23mod1.clone());
        z0vektor.push(z23mod1);
        let z29mod1 = (height - 29) % 30;
        z29vektor.push(z29mod1.clone());
        z0vektor.push(z29mod1);

        //Products of 7 and the rest
        let z7mod7 = (height - 49) % 30;
        z19vektor.push(z7mod7);
        let z11mod7 = (height - 77) % 30;
        z17vektor.push(z11mod7);
        let z13mod7 = (height - 91) % 30;
        z1vektor.push(z13mod7);
        let z17mod7 = (height - 119) % 30;
        z29vektor.push(z17mod7);
        let z19mod7 = (height - 133) % 30;
        z13vektor.push(z19mod7);
        let z23mod7 = (height - 161) % 30;
        z11vektor.push(z23mod7);
        let z29mod7 = (height - 203) % 30;
        z23vektor.push(z29mod7);

        //Products of 11 and the rest
        let z11mod11 = (height - 121) % 30;
        z1vektor.push(z11mod11);
        let z13mod11 = (height - 143) % 30;
        z23vektor.push(z13mod11);
        let z17mod11 = (height - 187) % 30;
        z7vektor.push(z17mod11);
        let z19mod11 = (height - 209) % 30;
        z29vektor.push(z19mod11);
        let z23mod11 = (height - 253) % 30;
        z13vektor.push(z23mod11);
        let z29mod11 = (height - 319) % 30;
        z19vektor.push(z29mod11);

        //Products of 13 and the rest
        let z13mod13 = (height - 169) % 30;
        z19vektor.push(z13mod13);
        let z17mod13 = (height - 221) % 30;
        z11vektor.push(z17mod13);
        let z19mod13 = (height - 247) % 30;
        z7vektor.push(z19mod13);
        let z23mod13 = (height - 299) % 30;
        z29vektor.push(z23mod13);
        let z29mod13 = (height - 377) % 30;
        z17vektor.push(z29mod13);

        //Products of 17 and the rest
        let z17mod17 = (height - 289) % 30;
        z19vektor.push(z17mod17);
        let z19mod17 = (height - 323) % 30;
        z23vektor.push(z19mod17);
        let z23mod17 = (height - 391) % 30;
        z1vektor.push(z23mod17);
        let z29mod17 = (height - 493) % 30;
        z13vektor.push(z29mod17);

        //Products of 19 and the rest
        let z19mod19 = (height - 361) % 30;
        z1vektor.push(z19mod19);
        let z23mod19 = (height - 437) % 30;
        z17vektor.push(z23mod19);
        let z29mod19 = (height - 551) % 30;
        z11vektor.push(z29mod19);

        //Products of 23 and the rest
        let z23mod23 = (height - 529) % 30;
        z19vektor.push(z23mod23);
        let z29mod23 = (height - 667) % 30;
        z7vektor.push(z29mod23);

        //Products of 29 and the rest
        let z29mod29 = (height - 841) % 30;
        z1vektor.push(z29mod29);

        //These are for test-printing the collections.
        //The vectors printed contain either all zeroes or all non-zeroes.
        //The all zeroes array indicates within which MOD paths to check for
        //factors. For instance, the z7vektor contains the mod30 of the
        //remainder after subtracting 1x7, 11x17, 13x19 and 23x29.
        //Those mod path combinations are what get checked against.
        //The z0vektor contains one from each and so is used below.
        //println!("{:?}", &z1vektor);
        //println!("{:?}", &z7vektor);
        //println!("{:?}", &z11vektor);
        //println!("{:?}", &z13vektor);
        //println!("{:?}", &z17vektor);
        //println!("{:?}", &z19vektor);
        //println!("{:?}", &z23vektor);
        //println!("{:?}", &z29vektor);
    }

    //We'll use this vector to iter through with if statements by z#vektor[0] == 0
    //as each branch will be unique and only one found to be true.
    println!("{:?}\n", &z0vektor);

    //get the sqrt of the semiprime as a starting point
    let semisqrt = (height as f64).sqrt();
    let trsqrt = semisqrt.clone().trunc() + 1.0;
    println!("trsqrt-- {}\n", &trsqrt);

    if z0vektor[0] == 0 {
        println!("path1 combinations");

        //1x1
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut j: f64;
            let mut kownt = 0;
            //Find the closest primes above on the first list

            //This is the 1x1 fn so can be condensed into one match.
            for i in LMOD1ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    kownt = kownt + 1;
                }
            }

            let mut dnval = (kownt).clone();
            dnval = dnval - 1;
            let mut upval = (kownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if dnval <= 1 {
                    break;
                }
                if upval > LMOD1ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD1ARR[upval].clone().into();
                let chkval2: u128 = LMOD1ARR[dnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD1ARR[dnval], &LMOD1ARR[upval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    dnval = dnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    upval = upval + 1;
                    continue;
                }
            }
        }

        //11x11
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut j: f64;
            let mut kownt = 0;
            //Find the closest primes above on the first list

            //This is the 11x11 fn so can be condensed into one match.
            for i in LMOD11ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    kownt = kownt + 1;
                }
            }

            let mut dnval = (kownt).clone();
            dnval = dnval - 1;
            let mut upval = (kownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if dnval <= 1 {
                    break;
                }
                if upval > LMOD11ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD11ARR[upval].clone().into();
                let chkval2: u128 = LMOD11ARR[dnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD11ARR[dnval], &LMOD11ARR[upval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    dnval = dnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    upval = upval + 1;
                    continue;
                }
            }
        }

        //19x19
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut j: f64;
            let mut kownt = 0;
            //Find the closest primes above on the first list

            //This is the 19x19 fn so can be condensed into one match.
            for i in LMOD19ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    kownt = kownt + 1;
                }
            }

            let mut dnval = (kownt).clone();
            dnval = dnval - 1;
            let mut upval = (kownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if dnval <= 1 {
                    break;
                }
                if upval > LMOD19ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD19ARR[upval].clone().into();
                let chkval2: u128 = LMOD19ARR[dnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD19ARR[dnval], &LMOD19ARR[upval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    dnval = dnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    upval = upval + 1;
                    continue;
                }
            }
        }

        //29x29
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut j: f64;
            let mut kownt = 0;
            //Find the closest primes above on the first list

            //This is the 29x29 fn so can be condensed into one match.
            for i in LMOD29ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    kownt = kownt + 1;
                }
            }

            let mut dnval = (kownt).clone();
            dnval = dnval - 1;
            let mut upval = (kownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if dnval <= 1 {
                    break;
                }
                if upval > LMOD29ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD29ARR[upval].clone().into();
                let chkval2: u128 = LMOD29ARR[dnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD29ARR[dnval], &LMOD29ARR[upval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    dnval = dnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    upval = upval + 1;
                    continue;
                }
            }
        }

        //7x13
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut rascd_list = 0;
            let mut rdesc_list = 0;
            let mut j: f64;
            let mut lkownt = 0;
            let mut rkownt = 0;
            //Find the closest primes above on the first list

            //This is the 7x13 fn so two matches
            for i in LMOD7ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    lkownt = lkownt + 1;
                }
            }
            let mut ldnval = (lkownt).clone();
            ldnval = ldnval - 1;

            let mut lupval = (lkownt).clone();

            for i in LMOD13ARR {
                j = i.into();

                if j <= trsqrt {
                    rascd_list = rascd_list + 1;
                    rdesc_list = rdesc_list + 1;

                    rkownt = rkownt + 1;
                }
            }

            let mut rdnval = (rkownt).clone();
            rdnval = rdnval - 1;

            let mut rupval = (rkownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if rdnval <= 1 {
                    break;
                }
                if lupval > LMOD7ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD7ARR[lupval].clone().into();
                let chkval2: u128 = LMOD13ARR[rdnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD7ARR[lupval], &LMOD13ARR[rdnval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    rdnval = rdnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    lupval = lupval + 1;
                    continue;
                }
            }

            loop {
                if ldnval <= 1 {
                    break;
                }
                if rupval > LMOD13ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD7ARR[ldnval].clone().into();
                let chkval2: u128 = LMOD13ARR[rupval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD7ARR[ldnval], &LMOD13ARR[rupval]
                    );
                    break;
                }
                if chkval < height.try_into().unwrap() {
                    rupval = rupval + 1;
                    continue;
                }
                if chkval > height.try_into().unwrap() {
                    ldnval = ldnval - 1;
                    continue;
                }
            }
        }

        //17x23
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut rascd_list = 0;
            let mut rdesc_list = 0;
            let mut j: f64;
            let mut lkownt = 0;
            let mut rkownt = 0;
            //Find the closest primes above on the first list

            //This is the 17x23 so two matches
            for i in LMOD17ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    lkownt = lkownt + 1;
                }
            }
            let mut ldnval = (lkownt).clone();
            ldnval = ldnval - 1;

            let mut lupval = (lkownt).clone();

            for i in LMOD23ARR {
                j = i.into();

                if j <= trsqrt {
                    rascd_list = rascd_list + 1;
                    rdesc_list = rdesc_list + 1;

                    rkownt = rkownt + 1;
                }
            }

            let mut rdnval = (rkownt).clone();
            rdnval = rdnval - 1;

            let mut rupval = (rkownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if rdnval <= 1 {
                    break;
                }
                if lupval > LMOD17ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD17ARR[lupval].clone().into();
                let chkval2: u128 = LMOD23ARR[rdnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD17ARR[lupval], &LMOD23ARR[rdnval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    rdnval = rdnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    lupval = lupval + 1;
                    continue;
                }
            }

            loop {
                if ldnval <= 1 {
                    break;
                }
                if rupval > LMOD23ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD17ARR[ldnval].clone().into();
                let chkval2: u128 = LMOD23ARR[rupval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD17ARR[ldnval], &LMOD23ARR[rupval]
                    );
                    break;
                }
                if chkval < height.try_into().unwrap() {
                    rupval = rupval + 1;
                    continue;
                }
                if chkval > height.try_into().unwrap() {
                    ldnval = ldnval - 1;
                    continue;
                }
            }
        }
    }

    if z0vektor[1] == 0 {
        println!("path7 combinations");

        //7x1
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut rascd_list = 0;
            let mut rdesc_list = 0;
            let mut j: f64;
            let mut lkownt = 0;
            let mut rkownt = 0;
            //Find the closest primes above on the first list

            //This is the 1x7 so two matches
            for i in LMOD7ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    lkownt = lkownt + 1;
                }
            }
            let mut ldnval = (lkownt).clone();
            ldnval = ldnval - 1;

            let mut lupval = (lkownt).clone();

            for i in LMOD1ARR {
                j = i.into();

                if j <= trsqrt {
                    rascd_list = rascd_list + 1;
                    rdesc_list = rdesc_list + 1;

                    rkownt = rkownt + 1;
                }
            }

            let mut rdnval = (rkownt).clone();
            rdnval = rdnval - 1;

            let mut rupval = (rkownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if rdnval <= 1 {
                    break;
                }
                if lupval > LMOD7ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD7ARR[lupval].clone().into();
                let chkval2: u128 = LMOD1ARR[rdnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD7ARR[lupval], &LMOD1ARR[rdnval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    rdnval = rdnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    lupval = lupval + 1;
                    continue;
                }
            }

            loop {
                if ldnval <= 1 {
                    break;
                }
                if rupval > LMOD1ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD7ARR[ldnval].clone().into();
                let chkval2: u128 = LMOD1ARR[rupval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD7ARR[ldnval], &LMOD1ARR[rupval]
                    );
                    break;
                }
                if chkval < height.try_into().unwrap() {
                    rupval = rupval + 1;
                    continue;
                }
                if chkval > height.try_into().unwrap() {
                    ldnval = ldnval - 1;
                    continue;
                }
            }
        }

        //17x11
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut rascd_list = 0;
            let mut rdesc_list = 0;
            let mut j: f64;
            let mut lkownt = 0;
            let mut rkownt = 0;
            //Find the closest primes above on the first list

            //This is the 17x11 so two matches
            for i in LMOD17ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    lkownt = lkownt + 1;
                }
            }
            let mut ldnval = (lkownt).clone();
            ldnval = ldnval - 1;

            let mut lupval = (lkownt).clone();

            for i in LMOD11ARR {
                j = i.into();

                if j <= trsqrt {
                    rascd_list = rascd_list + 1;
                    rdesc_list = rdesc_list + 1;

                    rkownt = rkownt + 1;
                }
            }

            let mut rdnval = (rkownt).clone();
            rdnval = rdnval - 1;

            let mut rupval = (rkownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if rdnval <= 1 {
                    break;
                }
                if lupval > LMOD17ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD17ARR[lupval].clone().into();
                let chkval2: u128 = LMOD11ARR[rdnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD17ARR[lupval], &LMOD11ARR[rdnval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    rdnval = rdnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    lupval = lupval + 1;
                    continue;
                }
            }

            loop {
                if ldnval <= 1 {
                    break;
                }
                if rupval > LMOD11ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD17ARR[ldnval].clone().into();
                let chkval2: u128 = LMOD11ARR[rupval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD17ARR[ldnval], &LMOD11ARR[rupval]
                    );
                    break;
                }
                if chkval < height.try_into().unwrap() {
                    rupval = rupval + 1;
                    continue;
                }
                if chkval > height.try_into().unwrap() {
                    ldnval = ldnval - 1;
                    continue;
                }
            }
        }

        //19x13
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut rascd_list = 0;
            let mut rdesc_list = 0;
            let mut j: f64;
            let mut lkownt = 0;
            let mut rkownt = 0;
            //Find the closest primes above on the first list

            //This is the 19x13 so two matches
            for i in LMOD19ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    lkownt = lkownt + 1;
                }
            }
            let mut ldnval = (lkownt).clone();
            ldnval = ldnval - 1;

            let mut lupval = (lkownt).clone();

            for i in LMOD13ARR {
                j = i.into();

                if j <= trsqrt {
                    rascd_list = rascd_list + 1;
                    rdesc_list = rdesc_list + 1;

                    rkownt = rkownt + 1;
                }
            }

            let mut rdnval = (rkownt).clone();
            rdnval = rdnval - 1;

            let mut rupval = (rkownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if rdnval <= 1 {
                    break;
                }
                if lupval > LMOD19ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD19ARR[lupval].clone().into();
                let chkval2: u128 = LMOD13ARR[rdnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD19ARR[lupval], &LMOD13ARR[rdnval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    rdnval = rdnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    lupval = lupval + 1;
                    continue;
                }
            }

            loop {
                if ldnval <= 1 {
                    break;
                }
                if rupval > LMOD13ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD19ARR[ldnval].clone().into();
                let chkval2: u128 = LMOD13ARR[rupval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD19ARR[ldnval], &LMOD13ARR[rupval]
                    );
                    break;
                }
                if chkval < height.try_into().unwrap() {
                    rupval = rupval + 1;
                    continue;
                }
                if chkval > height.try_into().unwrap() {
                    ldnval = ldnval - 1;
                    continue;
                }
            }
        }

        //29x23
        {
            //will walk one up one list and the other down the other starting from the sqrt.
            let mut lascd_list = 0;
            let mut ldesc_list = 0;
            let mut rascd_list = 0;
            let mut rdesc_list = 0;
            let mut j: f64;
            let mut lkownt = 0;
            let mut rkownt = 0;
            //Find the closest primes above on the first list

            //This is the 29x23 so two matches
            for i in LMOD29ARR {
                j = i.into();

                if j <= trsqrt {
                    lascd_list = lascd_list + 1;
                    ldesc_list = ldesc_list + 1;

                    lkownt = lkownt + 1;
                }
            }
            let mut ldnval = (lkownt).clone();
            ldnval = ldnval - 1;

            let mut lupval = (lkownt).clone();

            for i in LMOD23ARR {
                j = i.into();

                if j <= trsqrt {
                    rascd_list = rascd_list + 1;
                    rdesc_list = rdesc_list + 1;

                    rkownt = rkownt + 1;
                }
            }

            let mut rdnval = (rkownt).clone();
            rdnval = rdnval - 1;

            let mut rupval = (rkownt).clone();

            //begin multiplying dnval and kownt and checking for eq
            loop {
                if rdnval <= 1 {
                    break;
                }
                if lupval > LMOD29ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD29ARR[lupval].clone().into();
                let chkval2: u128 = LMOD23ARR[rdnval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD29ARR[lupval], &LMOD23ARR[rdnval]
                    );
                    break;
                }
                if chkval > height.try_into().unwrap() {
                    rdnval = rdnval - 1;
                    continue;
                }
                if chkval < height.try_into().unwrap() {
                    lupval = lupval + 1;
                    continue;
                }
            }

            loop {
                if ldnval <= 1 {
                    break;
                }
                if rupval > LMOD23ARR.len() - 1 {
                    break;
                }

                let chkval1: u128 = LMOD29ARR[ldnval].clone().into();
                let chkval2: u128 = LMOD23ARR[rupval].clone().into();
                let chkval: u128 = chkval1 * chkval2;

                if chkval == height.try_into().unwrap() {
                    println!(
                        "******************\n--result-> {:?} x {:?}\n***************",
                        &LMOD29ARR[ldnval], &LMOD23ARR[rupval]
                    );
                    break;
                }
                if chkval < height.try_into().unwrap() {
                    rupval = rupval + 1;
                    continue;
                }
                if chkval > height.try_into().unwrap() {
                    ldnval = ldnval - 1;
                    continue;
                }
            }
        }
    }

    println!("total millis: {}", now.elapsed().as_millis());
}
