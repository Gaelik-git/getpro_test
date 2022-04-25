#[macro_use]
extern crate lazy_static;

use std::slice::Iter;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Facette<'a>(pub &'a Domaine, pub &'a FacetteIndex);

#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct Phrase<'a>(pub &'a Facette<'a>, pub &'a PhraseIndex);

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
}

fn main() {
    println!("{:?}", *FACETTES);
    println!("{:?}", *PHRASES);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(PhraseIndex::P2.to_nbr(), 2);

        assert_eq!(FacetteIndex::F3.to_nbr(), 3);
    }
}
