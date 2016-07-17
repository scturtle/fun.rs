
#[derive(PartialEq, Debug)]
pub struct RibonucleicAcid {
    s : String
}

impl RibonucleicAcid {
    pub fn new(s : &str) -> RibonucleicAcid {
        RibonucleicAcid {s : s.to_string()}
    }
}

#[derive(PartialEq, Debug)]
pub struct DeoxyribonucleicAcid {
    s : String
}

impl DeoxyribonucleicAcid {
    pub fn new(s : &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid {s : s.to_string()}
    }
    pub fn to_rna(&self) -> RibonucleicAcid {
        let r : String = self.s.chars().map(
            |c| match c {'G' => 'C', 'C' => 'G', 'T' => 'A',
                         'A' => 'U', _ => ' '}).collect();
        RibonucleicAcid::new(&r)
    }
}
