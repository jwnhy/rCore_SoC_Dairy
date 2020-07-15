pub fn ex4() {
    let my_name = "Zed A. Shaw";
    let my_age = 35;
    let my_height = 74;
    let my_weight = 180;
    let my_eyes = "Blue";
    let my_teeth = "White";
    let my_hair = "Brown";

    println! ("Let's talk about {}", my_name);
    println! ("Actually that's not too heavy.");
    println! ("He's got {} eyes and {} hair.", my_eyes, my_hair);
    println! ("His teeth are usually {} depending on the coffee.", my_teeth);

    // this line is tricky, try to get it exactly right
    println! ("If I add {}, {}, and {} I get {}." ,my_age, my_height, my_weight, my_age +my_height + my_weight);
}