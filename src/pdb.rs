use std::collections::{BTreeMap, BTreeSet};

#[derive(Parser)]
#[grammar = "src/grammar/pdb.pest"]
pub struct PdbParser;

impl serde::Serialize for Rule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let rule_string = match self {
            Rule::pdbfile => "pdbfile",
            Rule::titleSection => "titleSection",
            Rule::header => "header",
            Rule::classification => "classification",
            Rule::depDate => "depDate",
            Rule::idCode => "idCode",
            Rule::obslte => "obslte",
            Rule::title => "title",
            Rule::split => "split",
            Rule::caveat => "caveat",
            Rule::compnd => "compnd",
            Rule::source => "source",
            Rule::keywds => "keywds",
            Rule::expdta => "expdta",
            Rule::nummdl => "nummdl",
            Rule::mdltyp => "mdltyp",
            Rule::author => "author",
            Rule::revdat => "revdat",
            Rule::modnum => "modnum",
            Rule::sprsde => "sprsde",
            Rule::jrnl => "jrnl",
            Rule::remark => "remark",
            Rule::remarkNum => "remarkNum",
            Rule::primaryStructureSection => "primaryStructureSection",
            Rule::dbref => "dbref",
            Rule::chainId => "chainId",
            Rule::seqBegin => "seqBegin",
            Rule::insertBegin => "insertBegin",
            Rule::seqEnd => "seqEnd",
            Rule::insertEnd => "insertEnd",
            Rule::database => "database",
            Rule::dbAccession => "dbAccession",
            Rule::dbIdCode => "dbIdCode",
            Rule::dbseqBegin => "dbseqBegin",
            Rule::idbnsBeg => "idbnsBeg",
            Rule::dbseqEnd => "dbseqEnd",
            Rule::dbinsEnd => "dbinsEnd",
            Rule::dbref1 => "dbref1",
            Rule::dbIdCode1 => "dbIdCode1",
            Rule::dbref2 => "dbref2",
            Rule::dbAccession2 => "dbAccession2",
            Rule::seqBegin2 => "seqBegin2",
            Rule::seqEnd2 => "seqEnd2",
            Rule::seqadv => "seqadv",
            Rule::resName => "resName",
            Rule::seqNum => "seqNum",
            Rule::iCode => "iCode",
            Rule::seqadvDatabase => "seqadvDatabase",
            Rule::seqadvDbAccession => "seqadvDbAccession",
            Rule::dbRes => "dbRes",
            Rule::dbSeq => "dbSeq",
            Rule::conflict => "conflict",
            Rule::seqres => "seqres",
            Rule::serNum => "serNum",
            Rule::numRes => "numRes",
            Rule::modres => "modres",
            Rule::stdRes => "stdRes",
            Rule::comment => "comment",
            Rule::heterogenSection => "heterogenSection",
            Rule::het => "het",
            Rule::hetId => "hetId",
            Rule::numHetAtoms => "numHetAtoms",
            Rule::hetText => "hetText",
            Rule::hetnam => "hetnam",
            Rule::hetNamText => "hetNamText",
            Rule::hetsyn => "hetsyn",
            Rule::hetSynText => "hetSynText",
            Rule::formul => "formul",
            Rule::compNum => "compNum",
            Rule::asterisk => "asterisk",
            Rule::formulText => "formulText",
            Rule::secondaryStructureSection => "secondaryStructureSection",
            Rule::helix => "helix",
            Rule::helixId => "helixId",
            Rule::initResName => "initResName",
            Rule::initChainId => "initChainId",
            Rule::initSeqNum => "initSeqNum",
            Rule::initICode => "initICode",
            Rule::endResName => "endResName",
            Rule::endChainId => "endChainId",
            Rule::endSeqNum => "endSeqNum",
            Rule::endICode => "endICode",
            Rule::helixClass => "helixClass",
            Rule::helixComment => "helixComment",
            Rule::helixLength => "helixLength",
            Rule::sheet => "sheet",
            Rule::strand => "strand",
            Rule::sheetId => "sheetId",
            Rule::numStrands => "numStrands",
            Rule::sense => "sense",
            Rule::curAtom => "curAtom",
            Rule::curResName => "curResName",
            Rule::curChainId => "curChainId",
            Rule::curResSeq => "curResSeq",
            Rule::curICode => "curICode",
            Rule::prevAtom => "prevAtom",
            Rule::prevResName => "prevResName",
            Rule::prevChainId => "prevChainId",
            Rule::prevResSeq => "prevResSeq",
            Rule::prevICode => "prevICode",
            Rule::connectivityAnnotationSection => "connectivityAnnotationSection",
            Rule::ssbond => "ssbond",
            Rule::ssbondResName1 => "ssbondResName1",
            Rule::chainId1 => "chainId1",
            Rule::seqNum1 => "seqNum1",
            Rule::iCode1 => "iCode1",
            Rule::ssbondResName2 => "ssbondResName2",
            Rule::chainId2 => "chainId2",
            Rule::seqNum2 => "seqNum2",
            Rule::iCode2 => "iCode2",
            Rule::sym1 => "sym1",
            Rule::sym2 => "sym2",
            Rule::ssbondLength => "ssbondLength",
            Rule::link => "link",
            Rule::name1 => "name1",
            Rule::altLoc1 => "altLoc1",
            Rule::resName1 => "resName1",
            Rule::resSeq1 => "resSeq1",
            Rule::name2 => "name2",
            Rule::altLoc2 => "altLoc2",
            Rule::resName2 => "resName2",
            Rule::resSeq2 => "resSeq2",
            Rule::linkLength => "linkLength",
            Rule::hydbnd => "hydbnd",
            Rule::sltbrg => "sltbrg",
            Rule::cispep => "cispep",
            Rule::pep1 => "pep1",
            Rule::pep2 => "pep2",
            Rule::modNum => "modNum",
            Rule::cispepMeasure => "cispepMeasure",
            Rule::site => "site",
            Rule::siteSeqNum => "siteSeqNum",
            Rule::siteId => "siteId",
            Rule::siteNumRes => "siteNumRes",
            Rule::seq => "seq",
            Rule::crystallographicTransformationSection => "crystallographicTransformationSection",
            Rule::cryst1 => "cryst1",
            Rule::cryst1A => "cryst1A",
            Rule::cryst1B => "cryst1B",
            Rule::cryst1C => "cryst1C",
            Rule::cryst1Alpha => "cryst1Alpha",
            Rule::cryst1Beta => "cryst1Beta",
            Rule::cryst1Gamma => "cryst1Gamma",
            Rule::sGroup => "sGroup",
            Rule::cryst1Z => "cryst1Z",
            Rule::origxn => "origxn",
            Rule::On1 => "On1",
            Rule::On2 => "On2",
            Rule::On3 => "On3",
            Rule::Tn => "Tn",
            Rule::scalen => "scalen",
            Rule::Sn1 => "Sn1",
            Rule::Sn2 => "Sn2",
            Rule::Sn3 => "Sn3",
            Rule::Un => "Un",
            Rule::tvect => "tvect",
            Rule::tvectx => "tvectx",
            Rule::tvecty => "tvecty",
            Rule::tvectz => "tvectz",
            Rule::matrixn => "matrixn",
            Rule::matrixnSerial => "matrixnSerial",
            Rule::Mn1 => "Mn1",
            Rule::Mn2 => "Mn2",
            Rule::Mn3 => "Mn3",
            Rule::Vn => "Vn",
            Rule::iGiven => "iGiven",
            Rule::coordinateSection => "coordinateSection",
            Rule::chain => "chain",
            Rule::model => "model",
            Rule::modelSerial => "modelSerial",
            Rule::atom => "atom",
            Rule::atomName => "atomName",
            Rule::altLoc => "altLoc",
            Rule::resSeq => "resSeq",
            Rule::atomX => "atomX",
            Rule::atomY => "atomY",
            Rule::atomZ => "atomZ",
            Rule::occupancy => "occupancy",
            Rule::tempFactor => "tempFactor",
            Rule::element => "element",
            Rule::charge => "charge",
            Rule::sigatm => "sigatm",
            Rule::anisou => "anisou",
            Rule::u11 => "u11",
            Rule::u22 => "u22",
            Rule::u33 => "u33",
            Rule::u12 => "u12",
            Rule::u13 => "u13",
            Rule::u23 => "u23",
            Rule::siguij => "siguij",
            Rule::ter => "ter",
            Rule::hetatm => "hetatm",
            Rule::serial => "serial",
            Rule::endmdl => "endmdl",
            Rule::connectivitySection => "connectivitySection",
            Rule::connect => "connect",
            Rule::bookkeepingSection => "bookkeepingSection",
            Rule::master => "master",
            Rule::numRemark => "numRemark",
            Rule::numUnused => "numUnused",
            Rule::numHet => "numHet",
            Rule::numHelix => "numHelix",
            Rule::numSheet => "numSheet",
            Rule::numTurn => "numTurn",
            Rule::numSite => "numSite",
            Rule::numXform => "numXform",
            Rule::numCoord => "numCoord",
            Rule::numTer => "numTer",
            Rule::numConect => "numConect",
            Rule::numSeq => "numSeq",
            Rule::end => "end",
            Rule::STRING => "STRING",
            Rule::CHAIN_ID => "CHAIN_ID",
            Rule::ATOM_NAME => "ATOM_NAME",
            Rule::ALT_LOC => "ALT_LOC",
            Rule::DATE => "DATE",
            Rule::IDCODE => "IDCODE",
            Rule::INTEGER => "INTEGER",
            Rule::SEQ_NUM => "SEQ_NUM",
            Rule::CONTINUATION => "CONTINUATION",
            Rule::RESIDUE_NAME => "RESIDUE_NAME",
        };
        serializer.serialize_str(rule_string)
    }
}

pub fn get_feature_value_map() -> BTreeMap<Rule, BTreeSet<String>> {
    BTreeMap::from([
        (Rule::database, BTreeSet::<String>::new()),
        (Rule::conflict, BTreeSet::<String>::new()),
        (Rule::comment, BTreeSet::<String>::new()),
        (Rule::hetText, BTreeSet::<String>::new()),
        (Rule::hetNamText, BTreeSet::<String>::new()),
        (Rule::hetSynText, BTreeSet::<String>::new()),
        (Rule::formulText, BTreeSet::<String>::new()),
        (Rule::helixComment, BTreeSet::<String>::new()),
        (Rule::ssbondResName1, BTreeSet::<String>::new()),
        (Rule::ssbondResName2, BTreeSet::<String>::new()),
        (Rule::resName1, BTreeSet::<String>::new()),
        (Rule::resName2, BTreeSet::<String>::new()),
        (Rule::pep1, BTreeSet::<String>::new()),
        (Rule::pep2, BTreeSet::<String>::new()),
        (Rule::element, BTreeSet::<String>::new()),
        (Rule::charge, BTreeSet::<String>::new()),
    ])
}