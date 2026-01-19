mod bool;
mod cmp;
mod expr;
mod io;
mod map;
mod num;
mod seq;
mod set;
mod stack;
mod str;
mod r#try;
mod vec;
mod vecz;

pub use bool::*;
pub use cmp::*;
pub use expr::*;
pub use io::*;
pub use map::*;
pub use num::*;
pub use seq::*;
pub use set::*;
pub use stack::*;
pub use str::*;
pub use r#try::*;
pub use vec::*;
pub use vecz::*;

use crate::{
    env::EuEnv,
    types::EuRes,
};

pub type EuDef = fn(&mut EuEnv) -> EuRes<()>;

pub const CORE: phf::Map<&str, EuDef> = phf::phf_map! {
    // stack
    "Stack" => STACK,

    "dup" => DUP,
    "dupd" => DUPD,
    "over" => OVER,
    "ddup" => DDUP,
    "edup" => EDUP,
    "pick" => PICK,
    "pop" => POP,
    "clr" => CLR,
    "nip" => NIP,
    "ppop" => PPOP,
    "qpop" => QPOP,
    "nix" => NIX,
    "swap" => SWAP,
    "rev" => REV,
    "swapd" => SWAPD,
    "tuck" => TUCK,
    "trade" => TRADE,
    "rot" => ROT,
    "rotB" => UNROT,
    "roll" => ROLL,
    "rollB" => UNROLL,
    "wrap" => WRAP,
    "unwrap" => UNWRAP,
    "usurp" => USURP,
    "sub" => SUB_STACK,
    "dip" => DIP,

    // io
    "read" => READ,
    "readL" => READLN,
    "print" => PRINT,
    "printL" => PRINTLN,

    // cmp
    "<=>" => CMP,
    "=" => EQ,
    "!=" => NE,
    "<" => LT,
    "<=" => LE,
    ">" => GT,
    ">=" => GE,

    // bool
    "True" => TRUE,
    "False" => FALSE,

    "bool" => TO_BOOL,
    "!" => NOT,

    // num
    "MinI32" => MIN_I32,
    "MaxI32" => MAX_I32,
    "MinI64" => MIN_I64,
    "MaxI64" => MAX_I64,
    "MinF64" => MIN_F64,
    "MaxF64" => MAX_F64,
    "Inf" => INF,
    "NaN" => NAN,

    "i32" => TO_I32,
    "i64" => TO_I64,
    "f64" => TO_F64,
    "ibig" => TO_IBIG,

    "_" => NEG,
    "+" => ADD,
    "-" => SUB,
    "*" => MUL,
    "/" => DIV,
    "%" => REM,
    "^" => POW,

    // str
    ">str" => TO_STR,

    // try
    "None" => NONE,
    "Some" => SOME,

    "Ok" => OK,
    "Err" => ERR,
    "#Res" => EVAL_RES,
    "?" => COALESCE,

    // vec
    ">Vec" => TO_VEC,
    "Vec" => WRAP_VEC,
    "*Vec" => ALL_VEC,
    "#Vec" => EVAL_VEC,
    "," => PAIR,

    // map
    ">Map" => TO_MAP,
    "Map" => WRAP_MAP,
    "*Map" => ALL_MAP,
    "#Map" => EVAL_MAP,

    // set
    ">Set" => TO_SET,
    "Set" => WRAP_SET,
    "*Set" => ALL_SET,
    "#Set" => EVAL_SET,

    // seq
    "SeqN0" => SEQ_N0,

    ">Seq" => TO_SEQ,
    "Seq" => WRAP_SEQ,
    "unfold" => UNFOLD,
    "rpt" => REPEAT,
    "rptN" => REPEAT_N,
    "cyc" => CYCLE,

    // expr
    ">Expr" => TO_EXPR,
    "Expr" => WRAP_EXPR,
    "#" => EVAL,
    "tap" => TAP,
    "&#" => AND_EVAL,
    "|#" => OR_EVAL,
    "&|#" => IF_EVAL,
    "->" => BIND_ARGS,

    // vecz
    ":" => GET,
    "has" => HAS,
    ":+" => PUSH_BACK,
    "+:" => PUSH_FRONT,
    "ins" => INSERT,
    "++" => APPEND,
    ":-" => POP_BACK,
    "-:" => POP_FRONT,
    "rmv" => REMOVE_INDEX,
    ":~" => MOVE_BACK,
    "~:" => MOVE_FRONT,
    "mov" => MOVE_INDEX,
    "del" => DELETE,

    "@" => AT,
    "tk" => TAKE,
    "dp" => DROP,
    "chunk" => CHUNK,
    "window" => WINDOW,
    "divvy" => DIVVY,
    "flat" => FLAT,
    "flatR" => FLAT_REC,
    "sort" => SORT,
    "enum" => ENUM,
    "pairs" => PAIRS,
    "zipN" => MULTI_ZIP,
    "cprodN" => MULTI_CPROD,

    "map" => MAP,
    "mapR" => MAP_ATOM,
    "mapF" => FLATMAP,
    "fltr" => FILTER,
    "tk?" => TAKE_WHILE,
    "dp?" => DROP_WHILE,
    "zip" => ZIP,
    "zipR" => ZIP_ATOM,
    "fold" => FOLD,
    "fold1" => FOLD1,
    "scan" => SCAN,
    "sort/" => SORT_BY,
    "sort#" => SORT_BY_KEY,
    "find" => FIND,
    "any" => ANY,
    "all" => ALL,
};
