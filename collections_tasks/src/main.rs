mod collections_tasks;

fn main() {
    println!(
        "Median: {:?}",
        collections_tasks::get_median(&mut vec![-2, 6, 5, 43, 5, 9, 32])
    );

    println!(
        "Median empty list: {:?}",
        collections_tasks::get_median(&mut vec![])
    );

    println!(
        "Mod empty list: {:?}",
        collections_tasks::get_mode(vec![])
    );

    println!(
        "Mod empty list: {:?}",
        collections_tasks::get_mode(vec![1,2,3,4,5,6,2,3,2,2,46,76,76,76,76,76])
    );

    println!(
        "Pig latin for first is {}",
        collections_tasks::Word::of("first").to_pig_latin().to_string()
    );

    println!(
        "Pig latin for apple is {}",
        collections_tasks::Word::of("apple").to_pig_latin().to_string()
    );

    let mut company = collections_tasks::Company::new();

    company.add_employee("Engineering", "G name");
    company.add_employee("Engineering", "F name");
    company.add_employee("Engineering", "A name");

    company.add_employee("Sales", "J name");
    company.add_employee("Sales", "C name");
    company.add_employee("Sales", "S name");

    println!("Engineering employees {:?}", company.get_employees_from_department("Engineering"));
    println!("Sales employees {:?}", company.get_employees_from_department("Sales"));
    println!("All company employees {:?}", company.get_all_employees());
}
