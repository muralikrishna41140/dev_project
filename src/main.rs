mod models;
mod handlers;

use models::{Developer, Role};
use handlers::*;
use uuid::Uuid;

fn main() {
    let mut dev1 = Developer {
        id: Uuid::new_v4(),
        name: String::from("Murali"),
        role: Role::Fullstack,
        skills: vec![String::from("Rust"), String::from("React")],
        experience: 5.0,
    };

    let dev2 = Developer {
        id: Uuid::new_v4(),
        name: String::from("Bob"),
        role: Role::Backend,
        skills: vec![String::from("Node.js"), String::from("SQL"), String::from("Docker")],
        experience: 3.5,
    };

    print_developer(&dev1);
    add_skill(&mut dev1, String::from("GraphQL"));

    println!("\nUpdated Developer Info:");
    print_developer(&dev1);

    println!("\nUsing Trait Implementation:");
    print_list(&[dev1, dev2]);

    let bio1 = "Experienced backend engineer.";
    let bio2 = "Fullstack developer.";
    println!("\nLongest bio: {}", longest(bio1, bio2));

    let skills_vec = vec![
        "Rust".into(),
        "TypeScript".into(),
        "GraphQL".into(),
        "HTML".into(),
    ];
    let top = top_skills(&skills_vec);
    println!("\nTop skills: {:?}", top);
}
