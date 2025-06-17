use crate::models::{Developer, Role};

pub fn print_developer(dev: &Developer) {
    println!("ID: {}", dev.id);
    println!("Name: {}", dev.name);
    println!("Experience: {} years", dev.experience);
    println!("Skills: {:?}", dev.skills);

    match dev.role {
        Role::Frontend => println!("Role: Frontend Developer"),
        Role::Backend => println!("Role: Backend Developer"),
        Role::Fullstack => println!("Role: Fullstack Developer"),
    }
}

pub fn add_skill(dev: &mut Developer, skill: String) {
    dev.skills.push(skill);
}

pub trait DisplayInfo {
    fn show_info(&self);
}

impl DisplayInfo for Developer {
    fn show_info(&self) {
        println!("{} - {} years of experience", self.name, self.experience);
    }
}

pub fn print_list<T: DisplayInfo>(items: &[T]) {
    for item in items {
        item.show_info();
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn top_skills(skills: &[String]) -> Vec<String> {
    let mut sorted = skills.to_vec();
    sorted.sort_by(|a, b| b.len().cmp(&a.len()));
    sorted.truncate(3);
    sorted
}
