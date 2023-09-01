use std::collections::{BTreeMap, BTreeSet};

#[derive(Parser)]
#[grammar = "src/grammar/balsa.pest"]
pub struct SmilesParser;

impl serde::Serialize for Rule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let rule_string = match self {
            Rule::String => "String",
            Rule::sequence => "sequence",
            Rule::union => "union",
            Rule::branch => "branch",
            Rule::gap => "gap",
            Rule::atom => "atom",
            Rule::bracket => "bracket",
            Rule::isotope => "isotope",
            Rule::symbol => "symbol",
            Rule::virtual_hydrogen => "virtual_hydrogen",
            Rule::charge => "charge",
            Rule::bridge => "bridge",
            Rule::parity => "parity",
            Rule::star => "star",
            Rule::dot => "dot",
            Rule::shortcut => "shortcut",
            Rule::selection => "selection",
            Rule::element => "element",
            Rule::bond => "bond",
            Rule::digit => "digit",
            Rule::nonzero => "nonzero",
        };
        serializer.serialize_str(rule_string)
    }
}

pub fn get_feature_value_map() -> BTreeMap<Rule, BTreeSet<String>> {
    BTreeMap::from([
        (Rule::bracket, BTreeSet::<String>::new()),
        (Rule::isotope, BTreeSet::<String>::new()),
        (Rule::symbol, BTreeSet::<String>::new()),
        (Rule::virtual_hydrogen, BTreeSet::<String>::new()),
        (Rule::charge, BTreeSet::<String>::new()),
        (Rule::bridge, BTreeSet::<String>::new()),
        (Rule::parity, BTreeSet::<String>::new()),
        (Rule::shortcut, BTreeSet::<String>::new()),
        (Rule::selection, BTreeSet::<String>::new()),
        (Rule::element, BTreeSet::<String>::new()),
        (Rule::bond, BTreeSet::<String>::new()),
    ])
}
