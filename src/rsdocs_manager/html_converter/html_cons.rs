use strum_macros::{EnumString, AsRefStr};


#[derive(EnumString, AsRefStr, PartialEq, Eq)]
pub enum DTAGS{
    NONE,
    H1,
    H2,
    H3,
    H4,
    P,
    IMG, //IMAGE
    TAB, //TABLE
    OL,//ORDERED LIST
    UOL, //UNORDERED LIST
    LNK, //LINK FILE
    HLNK, //HYPERLINK == WEB LINK
    TAG, //TAGS FOR THE FILE
    CMT, //COMMENT
}

pub enum Cons {
    SCN = 59, //SEMICOLON
    SPACE = 32,
    CR = 13, //CARRIAGUE RETURN
    DOT = 46,
    STRING = 34,
    CHAR = 39,
    ORB = 40, //OPEN ROUND BRACKET
    CRB = 41, //CLOSE ROUND BRACKET
    OSB = 91, //OPEN SQUARE BRACKET
    CSB = 93, //CLOSE SQUARE BRACKET
    OB = 123, //OPEN BRACKET
    CB = 125, //CLOSE BRACKET
    SLASH = 47,
    EQUALS = 61,
    NOT = 33,
    MULT = 42,
    POW = 94,
    MINUS = 45,
    ADD = 43,
}

pub fn get_flag(word: String) -> DTAGS {
    if word == W_H1 {return DTAGS::H1}
    if word == W_H2 {return DTAGS::H2}
    if word == W_H3 {return DTAGS::H3}
    if word == W_H4 {return DTAGS::H4}
    if word == W_HLNK {return DTAGS::HLNK}
    if word == W_IMG {return DTAGS::IMG}
    if word == W_LNK {return DTAGS::LNK}
    if word == W_OL {return DTAGS::OL}
    if word == W_P {return DTAGS::P}
    if word == W_TAB {return DTAGS::TAB}
    if word == W_UOL {return DTAGS::UOL}
    if word == W_CMT {return DTAGS::CMT}
    DTAGS::NONE
}


pub const W_H1:&str = ".H1";
pub const W_H2:&str = ".H2";
pub const W_H3:&str = ".H3";
pub const W_H4:&str = ".H4";
pub const W_HLNK:&str = ".HWLNK";
pub const W_IMG:&str = ".IMG";
pub const W_LNK:&str = ".LNK";
pub const W_OL:&str = ".OL";
pub const W_P:&str = ".P";
pub const W_TAB:&str = ".TAB";
pub const W_UOL:&str = ".UOL";
pub const W_CMT:&str = "..";
pub const W_OCMT:&str = "<!-- ";
pub const W_CCMT:&str = " -->";
pub const W_OT:&str = "<";
pub const W_CT:&str = "</";

pub const L_OTEM:&str = stringify!(<!DOCTYPE html><html lang="en">
    <head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Document</title></head><body>);

pub const L_CTEM:&str = stringify!(</body></html>);