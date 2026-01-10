pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();

        match sides {
            [0, _, _] => None,
            [min, middle, max] if min + middle < max => None,
            _ => Some(Self { sides }),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        matches!(self.sides, [a, b, c] if a == b && a == c && b == c)
    }

    pub fn is_scalene(&self) -> bool {
        matches!(self.sides, [a, b, c] if a != b && a != c && b != c)
    }

    pub fn is_isosceles(&self) -> bool {
        match self.sides {
            [a, b, _] if a == b => true,
            [a, _, c] if a == c => true,
            [_, b, c] if b == c => true,
            _ => false,
        }
    }
}
