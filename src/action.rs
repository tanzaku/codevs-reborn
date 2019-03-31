
pub enum Action {
    PutBlock { pos: usize, rot: usize },
    UseSkill
}


impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::PutBlock { pos, rot } => write!(f, "{} {}", pos, rot),
            Action::UseSkill => write!(f, "S"),
        }
    }
}
