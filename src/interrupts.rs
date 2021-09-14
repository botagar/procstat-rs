#[derive(Debug)]
pub struct Interrupts {
    pub total: u64,
    pub all: Vec<u64>,
}

impl Interrupts {
    pub fn empty() -> Self { Self { total: 0, all: Vec::new() } }

    pub fn from_vec(mut interrupt_vals: Vec<u64>) -> Self {
        if interrupt_vals.len() == 0 { return Self::empty() }

        let total_interrupts = interrupt_vals.remove(0);
        
        Interrupts {
            total: total_interrupts,
            all: interrupt_vals,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interrupts::Interrupts;

    #[test]
    fn creates_interrupt_from_vec_of_numbers() {
        let test_data: Vec<u64> = vec![24461286452, 34, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 309975325, 0, 0, 74, 0, 0];

        let interrupts: Interrupts = Interrupts::from_vec(test_data);
        
        assert_eq!(interrupts.total, 24461286452);
        assert_eq!(interrupts.all.len(), 20);
        assert_eq!(interrupts.all[0], 34);
    }

    #[test]
    fn returns_only_total_if_vec_is_only_1_element() {
        let test_data: Vec<u64> = vec![24461286452];

        let interrupts: Interrupts = Interrupts::from_vec(test_data);
        
        assert_eq!(interrupts.total, 24461286452);
        assert_eq!(interrupts.all.len(), 0);
    }

    #[test]
    fn returns_default_if_vec_is_empty() {
        let interrupts: Interrupts = Interrupts::from_vec(Vec::new());
        
        assert_eq!(interrupts.total, 0);
        assert_eq!(interrupts.all.len(), 0);
    }
}
