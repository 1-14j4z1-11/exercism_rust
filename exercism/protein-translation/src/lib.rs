use std::str;

pub struct CodonsInfo<'a> {
    pairs: Vec<(&'a str, &'a str)>
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        for pair in &self.pairs {
            if pair.0 == codon {
                return Some(pair.1);
            }
        }

        None
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut proteins = vec![];
        
        for codon in rna.bytes().collect::<Vec<_>>().chunks(3).map(|cs| str::from_utf8(cs).unwrap()) {
            match self.name_for(codon) {
                None => return None,
                Some("stop codon") => break,
                Some(name) => proteins.push(name),
            }
        }

        if proteins.is_empty() {
            None
        } else {
            Some(proteins)
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo{pairs:pairs}
}
