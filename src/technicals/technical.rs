use crate::config::definition::TacDefinition;

pub trait Technical {
    fn definition() -> TacDefinition;
}
