#[derive(Debug)]
pub struct SoftIrq {
    pub total_irqs: u64,
    pub all: Vec<u64>
}

impl SoftIrq {
    pub fn empty() -> Self { Self { total_irqs: 0, all: Vec::new() } }

    pub fn from_vec(mut soft_irq_vals: Vec<u64>) -> Self {
        if soft_irq_vals.len() == 0 { return Self::empty() }
    
        let total_irqs = soft_irq_vals.remove(0);
        
        Self {
            total_irqs,
            all: soft_irq_vals,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::soft_irq::SoftIrq;

    #[test]
    fn creates_interrupt_from_vec_of_numbers() {
        let test_data: Vec<u64> = vec![2095397048, 59605681, 87064650, 75257, 115853410, 3389780, 0, 27371450, 1253180383, 6010986, 542845451];

        let soft_irqs: SoftIrq = SoftIrq::from_vec(test_data);
        
        assert_eq!(soft_irqs.total_irqs, 2095397048);
        assert_eq!(soft_irqs.all.len(), 10);
        assert_eq!(soft_irqs.all[0], 59605681);
    }

    #[test]
    fn returns_only_total_if_vec_is_only_1_element() {
        let test_data: Vec<u64> = vec![2095397048];

        let soft_irqs: SoftIrq = SoftIrq::from_vec(test_data);
        
        assert_eq!(soft_irqs.total_irqs, 2095397048);
        assert_eq!(soft_irqs.all.len(), 0);
    }

    #[test]
    fn returns_default_if_vec_is_empty() {
        let soft_irqs: SoftIrq = SoftIrq::from_vec(Vec::new());
        
        assert_eq!(soft_irqs.total_irqs, 0);
        assert_eq!(soft_irqs.all.len(), 0);
    }
}
