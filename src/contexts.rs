#[derive(Debug)]
pub struct Contexts {
    pub total_switches: u64,
}

impl Contexts {
    pub fn empty () -> Self {
        Self {
            total_switches: 0
        }
    }

    pub fn from_vec(context_val: Vec<u64>) -> Self {
        if context_val.len() == 0 { return Contexts::empty() }
        
        Self {
            total_switches: *context_val.first().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::contexts::Contexts;

    #[test]
    fn creates_context_from_vector() {
        let test_data: Vec<u64> = vec![37115488136];
        let contexts: Contexts = Contexts::from_vec(test_data);
        
        assert_eq!(contexts.total_switches, 37115488136);
    }

    #[test]
    fn returns_default_if_vector_is_empty() {
        let contexts: Contexts = Contexts::from_vec(Vec::new());
        
        assert_eq!(contexts.total_switches, 0);
    }
}