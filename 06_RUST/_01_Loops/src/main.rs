fn main() {
    println!("level 2: loops of the river ");
    // for loop
    println!("---iterating using for loop:");
    for i in 1..5 {
        // 5 excluded if we want to include than use 1..=5 instead of 1..5
        println!("for loop iteration: {}", i);
    }

    // while loop
    let mut stamina_to_cross = 5;
    println!("---iterating while loop:");
    while stamina_to_cross > 0 {
        println!("While loop : {}", stamina_to_cross);
        stamina_to_cross -= 1;
    }

    // loop
    let mut water_level = 1;
    println!("---iterating using loop:");
    loop {
        println!("Water level: {}", water_level);
        if water_level == 5 {
            println!("Safe water level reached. Stopping.");
            break;
        }
        water_level += 1;
    }

    // loop with labels
    println!("---nested loop with labels:");
    let mut outer_counter = 0;
    'outer_loop: loop {
        println!("Outer loop iteration: {}", outer_counter);
        let mut inner_counter = 0;

        loop {
            println!("  Inner loop iteration: {}", inner_counter);
            if inner_counter == 2 {
                break; // breaks inner loop
            }
            if outer_counter == 2 {
                break 'outer_loop; // breaks outer loop
            }
            inner_counter += 1;
        }

        outer_counter += 1;
    }

    // for over collection with break and continue
    let river_zones = [
        "clean", "polluted", "clean", "danger", "clean", "dam", "clean",
    ];

    println!("---iterating using for:");
    for zone in river_zones {
        if zone == "dam" {
            println!("reached a dam, stopping the river exploration.");
            break;
        }

        if zone != "clean" {
            println!("skipping water zone: {}", zone);
            continue;
        }

        println!("water is  :{}", zone);
    }

    println!("---iterating using loop:");
    let mut i = 0;
    loop {
        if i >= river_zones.len() {
            break;
        }
        let zone = river_zones[i];
        i += 1;

        if zone == "dam" {
            println!("reached a dam, stopping the river exploration.");
            break; // river ends..
        }

        if zone != "clean" {
            println!("skipping water zone: {}", zone);
            continue; // skip the water not clean
        }

        println!("water is  :{}", zone);
    }

    println!("---iterating using while:");
    let mut j = 0;
    while j < river_zones.len() {
        let zone = river_zones[j];
        j += 1;

        if zone == "dam" {
            println!("reached a dam, stopping the river exploration.");
            break;
        }

        if zone != "clean" {
            println!("skipping water zone: {}", zone);
            continue;
        }

        println!("water is  :{}", zone);
    }
}
