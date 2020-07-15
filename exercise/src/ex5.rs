pub fn ex5() {
    let types_of_people = 10;
    let x = format!("There are {} types of people", types_of_people);

    let binary = "binary";
    let do_not = "don't";
    let y=format!("Those who know {} and those who {}", binary, do_not);

    println!("{}", x);
    println!("{}", y);

    println!("I sid: {}",x);
    println!("I also said: '{}'",y);

    let hilarious = false;
    let joke_evaluation = "Isn't that joke so funny?!";
    println!("{} {}", joke_evaluation, hilarious);

    let w = "This is the left side of...";
    let e = "a string with a right side";

    println!("{}{}", w , e);
}