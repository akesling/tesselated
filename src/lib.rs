pub struct Hex {
    // Prevent independent construction so invariants are enforced.
    _secret: (),
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

static HEX_DIRECTIONS: [Hex; 6] = [
    Hex {
        _secret: (),
        r: 1,
        q: 0,
        s: -1,
    },
    Hex {
        _secret: (),
        r: 1,
        q: -1,
        s: 0,
    },
    Hex {
        _secret: (),
        r: 0,
        q: -1,
        s: 1,
    },
    Hex {
        _secret: (),
        r: -1,
        q: 0,
        s: 1,
    },
    Hex {
        _secret: (),
        r: -1,
        q: 1,
        s: 0,
    },
    Hex {
        _secret: (),
        r: 0,
        q: 1,
        s: -1,
    },
];

impl Hex {
    pub fn new(q: i32, r: i32, s: i32) -> Option<Hex> {
        if q + r + s != 0 {
            return None;
        }

        Some(Hex {
            _secret: (),
            q,
            r,
            s,
        })
    }

    pub fn distance(&self, other: &Hex) -> i32 {
        let hex_vec = self - other;

        (hex_vec.q.abs() + hex_vec.r.abs() + hex_vec.s.abs()) / 2
    }

    pub fn neighbor(&self, dir: i32) -> Hex {
        self + &HEX_DIRECTIONS[(dir % 6) as usize]
    }
}

impl std::cmp::PartialEq for Hex {
    fn eq(&self, other: &Hex) -> bool {
        self.q == other.q && self.r == other.r && self.s == other.s
    }
}

impl std::ops::Add<&Hex> for &Hex {
    type Output = Hex;

    fn add(self, other: &Hex) -> Hex {
        Hex {
            _secret: (),
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

impl std::ops::Sub<&Hex> for &Hex {
    type Output = Hex;

    fn sub(self, other: &Hex) -> Hex {
        Hex {
            _secret: (),
            q: self.q - other.q,
            r: self.r - other.r,
            s: self.s - other.s,
        }
    }
}

impl std::ops::Mul<&Hex> for &Hex {
    type Output = Hex;

    fn mul(self, other: &Hex) -> Hex {
        Hex {
            _secret: (),
            q: self.q * other.q,
            r: self.r * other.r,
            s: self.s * other.s,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
