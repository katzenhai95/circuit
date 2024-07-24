pub mod boolean_simplify {
    use std::{
        cmp::min,
        collections::{HashMap, HashSet},
        iter::zip,
    };

    #[derive(Debug)]
    struct BooleanTerm {
        var_cnt: usize,
        group: u8,
        minterms: HashSet<u32>,
        terms: Vec<Option<bool>>, // TODO: use OnceCell
    }

    impl BooleanTerm {
        fn new(var_cnt: usize, minterm: u32) -> Self {
            let mut group: u8 = 0;
            let mut terms = vec![None; var_cnt];
            let mut minterm_tmp = minterm;
            for idx in (0..var_cnt).into_iter().rev() {
                if minterm_tmp % 2 == 1 {
                    group += 1;
                    terms[idx] = Some(true);
                } else {
                    terms[idx] = Some(false);
                }
                minterm_tmp = minterm_tmp >> 1;
            }
            let mut minterms = HashSet::new();
            minterms.insert(minterm);
            Self {
                var_cnt,
                group,
                minterms,
                terms,
            }
        }

        fn merge(&self, other: &Self) -> Self {
            assert_eq!(self.var_cnt, other.var_cnt);
            let mut new_minterms = self.minterms.clone();
            new_minterms.extend(other.minterms.clone().iter());
            Self {
                var_cnt: self.var_cnt,
                group: min(self.group, other.group),
                minterms: new_minterms,
                terms: zip(self.terms.clone(), other.terms.clone())
                    .map(|term| if term.0 == term.1 { term.0 } else { None })
                    .collect(),
            }
        }

        fn get_term(&self) -> Vec<Option<bool>> {
            self.terms.clone()
        }

        fn diff(&self, other: &Self) -> bool {
            let mut diff = false;
            for term in zip(&self.terms, &other.terms) {
                match term {
                    (Some(val), Some(val_other)) => {
                        if val ^ val_other {
                            if !diff {
                                diff = true;
                            } else {
                                return false;
                            }
                        }
                    }
                    (None, None) => {}
                    _ => return false,
                }
            }
            return diff;
        }
    }

    pub fn simplify(
        var_cnt: usize,
        ones: &Vec<u32>,
        dont_care: &Vec<u32>,
    ) -> Vec<Vec<Option<bool>>> {
        let mut all_ones = ones.clone();
        all_ones.extend(dont_care.iter());
        all_ones.sort();
        let mut bool_terms: Vec<BooleanTerm> = Vec::new();
        for one in all_ones {
            bool_terms.push(BooleanTerm::new(var_cnt, one));
        }
        bool_terms.sort_by(|a, b| a.group.cmp(&b.group));
        return qm_simplify(&bool_terms); // TODO: 2nd simplified and dont care
    }

    fn qm_simplify(terms: &Vec<BooleanTerm>) -> Vec<Vec<Option<bool>>> {
        println!("{:?}", terms);
        let mut simplified_term: Vec<Vec<Option<bool>>> = Vec::new();
        let mut term_map: HashMap<Vec<u32>, BooleanTerm> = std::collections::HashMap::new();
        let mut term_is_include: Vec<bool> = Vec::from_iter(terms.iter().map(|_| false));
        for (idx, bool_term) in terms.iter().enumerate() {
            for (idx_other, bool_term_other) in terms[idx..].iter().enumerate() {
                assert!(bool_term_other.group >= bool_term.group);
                if bool_term_other.group - bool_term.group != 1 {
                    continue;
                }
                if bool_term.diff(&bool_term_other) {
                    let new_term = bool_term.merge(bool_term_other);
                    let term = new_term.get_term();
                    if let None = term_map.insert(
                        new_term.minterms.clone().into_iter().collect::<Vec<u32>>(),
                        new_term,
                    ) {
                        simplified_term.push(term);
                    }
                    term_is_include[idx] = true;
                    term_is_include[idx + idx_other] = true;
                }
            }
        }
        let mut res = if !simplified_term.is_empty() {
            let mut new_terms: Vec<BooleanTerm> = term_map.into_values().into_iter().collect();
            new_terms.sort_by(|a, b| a.group.cmp(&b.group));
            qm_simplify(&new_terms)
        } else {
            terms.iter().map(|term| term.get_term()).collect()
        };
        for (idx, bool_term) in terms.iter().enumerate() {
            if !term_is_include[idx] {
                res.push(bool_term.get_term());
            }
        }
        res
    }
}

struct Circuit;

impl Circuit {
    fn new() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::boolean_simplify;

    #[test]
    fn it_works() {
        // https://blog.csdn.net/rshchx007/article/details/109732433
        let result = boolean_simplify::simplify(
            5,
            &vec![2, 3, 7, 9, 10, 11, 12, 13, 18, 19, 22, 23, 26, 27, 30, 31],
            &vec![],
        );
        // TODO: solve repeated result
    }
}
