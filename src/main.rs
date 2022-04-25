#[macro_use]
extern crate lazy_static;

use crate::Domaine::*;
use std::{collections::HashMap, fmt::Display, ops::Add, slice::Iter};

lazy_static! {
    static ref FACETTES: Vec<Facette<'static>> = {
        let mut facettes = vec![];
        for domain in Domaine::iterator() {
            for facette_number in FacetteIndex::iterator() {
                let facette = Facette(domain, facette_number);
                facettes.push(facette)
            }
        }
        facettes
    };
    static ref PHRASES: Vec<Phrase<'static>> = {
        let mut phrases = vec![];
        for facette in FACETTES.iter() {
            for phrase_index in PhraseIndex::iterator() {
                phrases.push(Phrase(facette, phrase_index));
            }
        }
        phrases
    };
    static ref REPARTITIONS: Vec<Repartition> = {
        vec![
            Repartition(A, C, E, N),
            Repartition(A, C, E, N),
            Repartition(A, A, C, E),
            Repartition(A, A, C, E),
            Repartition(C, C, A, E),
            Repartition(C, C, A, E),
            Repartition(E, E, A, C),
            Repartition(E, E, A, C),
            Repartition(N, N, A, C),
            Repartition(N, N, A, C),
            Repartition(O, O, A, C),
            Repartition(O, O, A, C),
            Repartition(A, C, E, O),
            Repartition(A, C, E, O),
            Repartition(A, A, C, N),
            Repartition(A, A, C, N),
            Repartition(C, C, A, N),
            Repartition(C, C, A, N),
            Repartition(E, E, A, N),
            Repartition(E, E, A, N),
            Repartition(N, N, A, E),
            Repartition(N, N, A, E),
            Repartition(O, O, A, E),
            Repartition(O, O, A, E),
            Repartition(A, C, N, O),
            Repartition(A, C, N, O),
            Repartition(A, A, C, O),
            Repartition(A, A, C, O),
            Repartition(C, C, A, O),
            Repartition(C, C, A, O),
            Repartition(E, E, A, O),
            Repartition(E, E, A, O),
            Repartition(N, N, A, O),
            Repartition(N, N, A, O),
            Repartition(O, O, A, N),
            Repartition(O, O, A, N),
            Repartition(A, E, N, O),
            Repartition(A, E, N, O),
            Repartition(A, A, E, N),
            Repartition(A, A, E, N),
            Repartition(C, C, E, N),
            Repartition(C, C, E, N),
            Repartition(E, E, C, N),
            Repartition(E, E, C, N),
            Repartition(N, N, C, E),
            Repartition(N, N, C, E),
            Repartition(O, O, C, E),
            Repartition(O, O, C, E),
            Repartition(C, E, N, O),
            Repartition(C, E, N, O),
            Repartition(A, A, E, O),
            Repartition(A, A, E, O),
            Repartition(C, C, E, O),
            Repartition(C, C, E, O),
            Repartition(E, E, C, O),
            Repartition(E, E, C, O),
            Repartition(N, N, C, O),
            Repartition(N, N, C, O),
            Repartition(O, O, C, N),
            Repartition(O, O, C, N),
            Repartition(A, A, N, O),
            Repartition(A, A, N, O),
            Repartition(C, C, N, O),
            Repartition(C, C, N, O),
            Repartition(E, E, N, O),
            Repartition(E, E, N, O),
            Repartition(N, N, E, O),
            Repartition(N, N, E, O),
            Repartition(O, O, E, N),
            Repartition(O, O, E, N),
        ]
    };
}

#[derive(Debug)]
struct Repartition(pub Domaine, pub Domaine, pub Domaine, pub Domaine);

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Domaine {
    A,
    C,
    E,
    N,
    O,
}

impl Domaine {
    pub fn iterator() -> Iter<'static, Domaine> {
        static DOMAINES: [Domaine; 5] =
            [Domaine::A, Domaine::C, Domaine::E, Domaine::N, Domaine::O];
        DOMAINES.iter()
    }
}

impl Display for Domaine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            A => "A",
            C => "C",
            E => "E",
            N => "N",
            O => "O",
        };
        write!(f, "{}", v)
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum FacetteIndex {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
}

impl FacetteIndex {
    pub fn iterator() -> Iter<'static, Self> {
        static FACETTEINDEX: [FacetteIndex; 6] = [
            FacetteIndex::F1,
            FacetteIndex::F2,
            FacetteIndex::F3,
            FacetteIndex::F4,
            FacetteIndex::F5,
            FacetteIndex::F6,
        ];
        FACETTEINDEX.iter()
    }

    fn to_nbr(&self) -> u8 {
        match self {
            FacetteIndex::F1 => 1,
            FacetteIndex::F2 => 2,
            FacetteIndex::F3 => 3,
            FacetteIndex::F4 => 4,
            FacetteIndex::F5 => 5,
            FacetteIndex::F6 => 6,
        }
    }
}

impl Display for FacetteIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_nbr())
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Facette<'a>(pub &'a Domaine, pub &'a FacetteIndex);

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum PhraseIndex {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
}

impl Display for PhraseIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            PhraseIndex::P0 => 0,
            PhraseIndex::P1 => 1,
            PhraseIndex::P2 => 2,
            PhraseIndex::P3 => 3,
            PhraseIndex::P4 => 4,
            PhraseIndex::P5 => 5,
            PhraseIndex::P6 => 6,
            PhraseIndex::P7 => 7,
            PhraseIndex::P8 => 8,
            PhraseIndex::P9 => 9,
        };

        write!(f, "{}", a)
    }
}

impl PhraseIndex {
    fn iterator() -> Iter<'static, Self> {
        static PHRASEINDEX: [PhraseIndex; 10] = [
            PhraseIndex::P0,
            PhraseIndex::P1,
            PhraseIndex::P2,
            PhraseIndex::P3,
            PhraseIndex::P4,
            PhraseIndex::P5,
            PhraseIndex::P6,
            PhraseIndex::P7,
            PhraseIndex::P8,
            PhraseIndex::P9,
        ];
        PHRASEINDEX.iter()
    }

    fn to_nbr(&self) -> u8 {
        match self {
            PhraseIndex::P0 => 0,
            PhraseIndex::P1 => 1,
            PhraseIndex::P2 => 2,
            PhraseIndex::P3 => 3,
            PhraseIndex::P4 => 4,
            PhraseIndex::P5 => 5,
            PhraseIndex::P6 => 6,
            PhraseIndex::P7 => 7,
            PhraseIndex::P8 => 8,
            PhraseIndex::P9 => 9,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Phrase<'a>(pub &'a Facette<'a>, pub &'a PhraseIndex);

impl Phrase<'_> {
    fn valid_with(&self, phrase: &Phrase) -> bool {
        self.0 != phrase.0
    }
}

impl Display for Phrase<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}.{}", self.0 .0, self.0 .1, self.1)
    }
}

#[derive(Debug)]
struct Quadruplet<'a>(pub [&'a Phrase<'a>; 4]);

impl Quadruplet<'static> {
    fn new(phrases: [&'static Phrase; 4]) -> Result<Quadruplet<'static>, ()> {
        //Check to 2 phrase have same Facette
        for i in 0..4 {
            for j in (i + 1)..4 {
                if !phrases[i].valid_with(&phrases[j]) {
                    return Err(());
                }
            }
        }

        //Check all not of same domain

        if phrases[0].0 .0 == phrases[1].0 .0
            && phrases[0].0 .0 == phrases[2].0 .0
            && phrases[0].0 .0 == phrases[3].0 .0
        {
            return Err(());
        }

        Ok(Quadruplet(phrases))
    }
}

impl Display for Quadruplet<'static> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ; {} ; {} ; {}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

fn main() {
    // println!("{:?}", *FACETTES);
    // println!("{:?}", *PHRASES);

    let mut all_quadrupletes: Vec<Result<Quadruplet, ()>> = vec![];

    let mut hashmap: HashMap<&Phrase, u8> =
        PHRASES.iter().map(|p| (p, 0)).collect::<HashMap<_, _>>();

    for repartition in REPARTITIONS.iter() {
        let mut phrases: Vec<&Phrase> = vec![];
        for domaine in [
            &repartition.0,
            &repartition.1,
            &repartition.2,
            &repartition.3,
        ] {
            let p: &Phrase = get_phrase(&hashmap, domaine, &phrases);
            let val = hashmap.get(p).unwrap();
            hashmap.insert(p, val + 1);
            phrases.push(p);
        }

        let value = Quadruplet::new([phrases[0], phrases[1], phrases[2], phrases[3]]);
        all_quadrupletes.push(value);
    }

    let all_valid_quadruplet: Vec<Quadruplet> = all_quadrupletes
        .into_iter()
        .filter_map(|q| q.ok())
        .collect();

    println!("number of result {}", all_valid_quadruplet.len());
    for (phrase, nbr) in hashmap {
        println!("{}:{}", phrase, nbr);
    }

    all_valid_quadruplet.iter().for_each(|f| println!("{}", f))

    //println!("{:?}", all_valid_quadruplet);
}

fn get_phrase<'a>(
    hashmap: &HashMap<&'a Phrase, u8>,
    repartition: &Domaine,
    phrases: &Vec<&Phrase>,
) -> &'a Phrase<'a> {
    let mut phrases_of_domaine: Vec<&Phrase<'a>> = PHRASES
        .iter()
        .filter(|&p| p.0 .0 == repartition)
        .filter(|p| phrases.iter().all(|a| p.valid_with(a)))
        .collect();

    phrases_of_domaine.sort_by(|a, b| {
        let nbr_of_use_a = hashmap.get(a).unwrap();
        let nbr_of_use_b = hashmap.get(b).unwrap();
        nbr_of_use_a.partial_cmp(nbr_of_use_b).unwrap()
    });

    phrases_of_domaine[0]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(PhraseIndex::P2.to_nbr(), 2);

        assert_eq!(FacetteIndex::F3.to_nbr(), 3);

        assert!(!PHRASES[0].valid_with(&PHRASES[0]));
        assert!(PHRASES[0].valid_with(&PHRASES[10]));

        let phrases = [&PHRASES[0], &PHRASES[0], &PHRASES[45], &PHRASES[100]];
        println!("{:?}", phrases);
        assert!(Quadruplet::new(phrases).is_err());

        let phrases = [&PHRASES[0], &PHRASES[1], &PHRASES[2], &PHRASES[3]];
        println!("{:?}", phrases);
        assert!(Quadruplet::new(phrases).is_err());

        let phrases = [&PHRASES[0], &PHRASES[99], &PHRASES[199], &PHRASES[299]];
        println!("{:?}", phrases);
        assert!(Quadruplet::new(phrases).is_ok());
    }
}
