 use uuid::Uuid;

#[derive(Debug)]
pub enum Role {
    Frontend,
    Backend,
    Fullstack,
}

#[derive(Debug)]
pub struct Developer {
    pub id: Uuid,
    pub name: String,
    pub role: Role,
    pub skills: Vec<String>,
    pub experience: f32,
}
