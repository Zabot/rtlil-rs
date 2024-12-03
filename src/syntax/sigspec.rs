use super::*;

#[derive(Debug, Clone)]
pub enum Index {
    Offset(i64),
    Range((i64, i64)),
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Index::Offset(offset) => write!(f, "[{}]", offset),
            Index::Range((l, r)) => write!(f, "[{}:{}]", l, r),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SigSpec {
    Const((Const, Option<Index>)),
    Refer((String, Option<Index>)),
    List(Vec<SigSpec>),
}

impl fmt::Display for SigSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigSpec::Const((n, index)) => {
                write!(f, "{}", n)?;
                if let Some(index) = index {
                    write!(f, "{}", index)?;
                }
            }
            SigSpec::Refer((n, index)) => {
                write!(f, "{}", n)?;
                if let Some(index) = index {
                    write!(f, "{}", index)?;
                }
            }
            SigSpec::List(n) => {
                write!(f, "{{")?;
                for m in n.iter() {
                    write!(f, "{}", m)?;
                }
                write!(f, "}}")?;
            }
        };
        Ok(())
    }
}
