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
    "unrot" => UNROT,
    "roll" => ROLL,
    "unroll" => UNROLL,
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
    "MinF32" => MIN_F32,
    "MaxF32" => MAX_F32,
    "MinF64" => MIN_F64,
    "MaxF64" => MAX_F64,
    "Inf" => INF,
    "Inf32" => INF32,
    "NaN" => NAN,
    "NaN32" => NAN32,

    "i32" => TO_I32,
    "f32" => TO_F32,
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
    "#res" => EVAL_RES,
    "?" => TRY,

    // vec
    ">vec" => TO_VEC,
    "Vec" => WRAP_VEC,
    "*vec" => ALL_VEC,
    "#vec" => EVAL_VEC,
    "," => PAIR,

    // map
    ">map" => TO_MAP,
    "Map" => WRAP_MAP,
    "*map" => ALL_MAP,
    "#map" => EVAL_MAP,

    // set
    ">set" => TO_SET,
    "Set" => WRAP_SET,
    "*set" => ALL_SET,
    "#set" => EVAL_SET,

    // seq
    "SeqN0" => SEQ_N0,

    ">seq" => TO_SEQ,
    "Seq" => WRAP_SEQ,
    "unfold" => UNFOLD,
    "rpt" => REPEAT,
    "rptN" => REPEAT_N,
    "cyc" => CYCLE,

    // expr
    ">expr" => TO_EXPR,
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
