#[derive(Debug, Clone, Eq)]
pub enum Path<TTransition> {
    FromSource(Vec<TTransition>),
    FromTarget(Vec<TTransition>),
}

impl<TTransition> PartialEq for Path<TTransition>
    where TTransition : Eq {
    
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FromSource(l0), Self::FromSource(r0)) => l0 == r0,
            (Self::FromSource(l0), Self::FromTarget(r0)) => l0.iter().rev().eq(r0),
            (Self::FromTarget(l0), Self::FromSource(r0)) => l0.iter().rev().eq(r0),
            (Self::FromTarget(l0), Self::FromTarget(r0)) => l0 == r0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    enum PathElement {
        A,
        B,
        C
    }

    #[test]
    fn should_equate_from_different_direction() {
        let p1 = Path::FromSource(vec![PathElement::A, PathElement::B]);
        let p2 = Path::FromTarget(vec![PathElement::B, PathElement::A]);

        assert_eq!(p1, p2);
    }

    #[test]
    fn should_equate_from_different_direction_len_3() {
        let p1 = Path::FromSource(vec![PathElement::A, PathElement::B, PathElement::C]);
        let p2 = Path::FromTarget(vec![PathElement::C, PathElement::B, PathElement::A]);

        assert_eq!(p1, p2);
    }
}