static TESTNUMBER: u32 = 325489;


pub fn day() -> String {
    //// Right side
    //println!("Right Side");
    //check_number(9);
    //check_number(10);
    //check_number(11);
    //check_number(12);
    //// Top Side
    //println!("Top Side");
    //check_number(13);
    //check_number(14);
    //check_number(15);
    //check_number(16);
    //// Left side
    //println!("Left Side");
    //check_number(17);
    //check_number(18);
    //check_number(19);
    //check_number(20);
    //check_number(21);
    //// Bottom side
    //println!("Bottom Side");
    //check_number(22);
    //check_number(23);
    //check_number(24);
    //check_number(25);
    //println!("Test num");
    check_number(TESTNUMBER)
}


fn check_number(num: u32) -> String {
    //println!("===============Testing num: {}===============", num);
    let f_test_number: f32 = num as f32;
    let sqrt = f_test_number.sqrt();
    let whole_sqrt = sqrt as u32;

    // Landed perfectly on a sqrt? then sqrt - 1 is the answer
    if f_test_number / sqrt == (whole_sqrt as f32) {
        //println!("Nailed it! => {}", whole_sqrt - 1);
        return (whole_sqrt - 1).to_string();
    }

    let lower_bounds = whole_sqrt.pow(2);
    let mut num_to_one;
    let mut curval;
    let midval;
    if whole_sqrt % 2 == 0 {
        let corner = lower_bounds + 1 + whole_sqrt;
        midval = whole_sqrt / 2;
        // even sqrt on the bottom
        if num <= corner {
            //left side
            curval = corner;
            num_to_one = whole_sqrt;

        } else {
            //bottom side
            curval = (whole_sqrt + 1).pow(2); // puts you in bottom right corner
            num_to_one = whole_sqrt;
        }
    } else {
        let corner = lower_bounds + 1 + whole_sqrt;
        midval = (whole_sqrt + 1 /* because odd root */) / 2;
        // odd sqrt on the bottom
        if num <= corner {
            //right side
            curval = corner;
            num_to_one = whole_sqrt + 1;
        } else {
            //top side
            curval = (whole_sqrt + 1).pow(2) + 1; // puts you in top left corner
            num_to_one = whole_sqrt + 1;
        }
    }
    let mut count_down = true;
    while curval != num {
        if count_down {
            num_to_one -= 1;
        } else {
            num_to_one += 1;
        }
        if midval == num_to_one {
            count_down = false;
        }
        curval -= 1;
    }
    //println!("Num to one: {}", num_to_one);
    num_to_one.to_string()
}
