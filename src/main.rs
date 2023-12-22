mod plopper;

use plopper::plop_region;

fn main() {
    let x_lower_bound = -25;
    let x_upper_bound = 21;
    let z_lower_bound = -20;
    let z_upper_bound = 22;

    for z in z_lower_bound..(z_upper_bound + 1) {
        for x in x_lower_bound..(x_upper_bound + 1) {
            let upper_path = format!("regions/upper/r.{:?}.{:?}.mca", x, z);
            let lower_path = format!("regions/lower/r.{:?}.{:?}.mca", x, z);

            println!("Begin plopping r.{:?}.{:?}.mca", x, z);
            
            plop_region(&upper_path, &lower_path);

            println!("End plopping r.{:?}.{:?}.mca", x, z);
        }
    }
}