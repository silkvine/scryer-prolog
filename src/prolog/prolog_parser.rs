use prolog::ast::*;
use std::cell::Cell;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__TopLevel {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use prolog::ast::*;
    use std::cell::Cell;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2e_22(&'input str),
        Term_22_3a_2d_22(&'input str),
        Term_22_3f_2d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5b_5d_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22___22(&'input str),
        Term_22_7c_22(&'input str),
        Termr_23_22_5bA_2dZ_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(&'input str),
        Termr_23_22_5ba_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt_28_22_2c_22_20_3cTerm_3e_29(Term),
        Nt_28_22_2c_22_20_3cTerm_3e_29_2a(::std::vec::Vec<Term>),
        Nt_28_22_2c_22_20_3cTerm_3e_29_2b(::std::vec::Vec<Term>),
        Nt_28_3cBoxedTerm_3e_20_22_2c_22_29(Box<Term>),
        Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2a(::std::vec::Vec<Box<Term>>),
        Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2b(::std::vec::Vec<Box<Term>>),
        Nt_28_3cPredicateClause_3e_29(PredicateClause),
        Nt_28_3cPredicateClause_3e_29_2b(::std::vec::Vec<PredicateClause>),
        NtAtom(Atom),
        NtBoxedTerm(Box<Term>),
        NtClause(Term),
        NtList(Term),
        NtListInternals(Term),
        NtPredicate(Vec<PredicateClause>),
        NtPredicateClause(PredicateClause),
        NtRule(Rule),
        NtTerm(Term),
        NtTopLevel(TopLevel),
        NtVar(Var),
        Nt____TopLevel(TopLevel),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 0, 16, 17, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 13, 14, 0, 15, 0, 16, 17, 0,
        // State 2
        21, 0, 0, -30, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, -31, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, -12, -12, 0, -12, 0, -12, -12, 0,
        // State 7
        0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 13, 14, 0, 15, 0, 16, 29, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 37, 38, 0, 39, 0, 40, 41, 0,
        // State 13
        0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        -14, 0, 0, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, -13, -13, 0, -13, 0, -13, -13, 0,
        // State 18
        0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 61, 62, 0, 63, 0, 64, 65, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 61, 62, 0, 63, 0, 64, 65, 0,
        // State 23
        0, 0, 0, 0, 0, 0, -24, -24, 0, -24, 0, -24, -24, 0,
        // State 24
        0, 0, 0, 0, 0, 0, -25, -25, 0, -25, 0, -25, -25, 0,
        // State 25
        67, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        -14, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        69, 0, -30, 0, 0, 0, 0, 0, -30, 0, -30, 0, 0, 0,
        // State 30
        0, 0, 70, 0, 0, 0, 0, 0, -20, 0, 71, 0, 0, 0,
        // State 31
        0, 0, -31, 0, 0, 0, 0, 0, -31, 0, -31, 0, 0, 0,
        // State 32
        0, 0, -32, 0, 0, 0, 0, 0, -32, 0, -32, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0,
        // State 34
        0, 0, -15, 0, 0, 0, 0, 0, -15, 0, -15, 0, 0, 0,
        // State 35
        0, 0, -33, 0, 0, 0, 0, 0, -33, 0, -33, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 37, 38, 0, 39, 0, 40, 41, 0,
        // State 37
        0, 0, -18, 0, 0, 0, 0, 0, -18, 0, -18, 0, 0, 0,
        // State 38
        0, 0, -34, 0, 0, 0, 0, 0, -34, 0, -34, 0, 0, 0,
        // State 39
        0, 0, -39, 0, 0, 0, 0, 0, -39, 0, -39, 0, 0, 0,
        // State 40
        -14, 0, -14, 0, 0, 0, 0, 0, -14, 0, -14, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, -24, -24, 0, -24, 0, -24, -24, 0,
        // State 42
        0, 0, 0, 0, 0, 0, -25, -25, 0, -25, 0, -25, -25, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 44
        75, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 76, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 37, 38, 0, 39, 0, 40, 41, 0,
        // State 51
        0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        -14, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        79, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 81, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 37, 38, 0, 39, 0, 40, 41, 0,
        // State 61
        0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        -14, 0, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 81, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 37, 38, 0, 39, 0, 40, 41, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 95, 96, 0, 97, 0, 98, 99, 0,
        // State 71
        0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0,
        // State 73
        0, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 75
        0, 0, 0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, -9, -9, 0, -9, 0, -9, -9, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 105, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 79
        0, 0, 108, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 61, 62, 0, 63, 0, 64, 65, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 108, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 84
        0, 112, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 86
        0, 114, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0,
        // State 88
        115, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0,
        // State 94
        0, 0, 0, 0, 0, 0, 37, 38, 0, 39, 0, 40, 41, 0,
        // State 95
        0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0,
        // State 96
        0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0,
        // State 98
        -14, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0,
        // State 99
        0, 0, -19, 0, 0, 0, 0, 0, -19, 0, -19, 0, 0, 0,
        // State 100
        0, 0, 0, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, -10, -10, 0, -10, 0, -10, -10, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 103
        0, 118, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 106
        0, 120, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 107
        0, 0, 0, 0, 0, 0, 61, 62, 0, 63, 0, 64, 65, 0,
        // State 108
        0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, 122, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, 123, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 113
        0, 0, -16, 0, 0, 0, 0, 0, -16, 0, -16, 0, 0, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 126, 0, 0, 0, 0, 0,
        // State 116
        0, 127, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 128, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 121
        0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        0, 0, -17, 0, 0, 0, 0, 0, -17, 0, -17, 0, 0, 0,
        // State 123
        0, 0, 0, 0, 0, 0, 51, 52, 0, 53, 0, 54, 55, 0,
        // State 124
        0, 130, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 125
        0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0,
        // State 126
        0, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 127
        0, 0, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 128
        0, 131, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0,
        // State 130
        0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        0,
        0,
        0,
        0,
        -36,
        0,
        0,
        0,
        -40,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -23,
        0,
        0,
        0,
        0,
        0,
        -37,
        -38,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -24,
        -25,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -35,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 6, 7, 8, 9, 10, 11, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 5, 0, 0, 18, 19, 20, 0, 11, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 27, 5, 0, 0, 0, 0, 28, 0, 11, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 33, 34, 0, 0, 0, 35, 0, 36, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 44, 0, 0, 45, 46, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 57, 58, 0, 0, 0, 0, 59, 0, 60, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 57, 58, 0, 0, 0, 0, 66, 0, 60, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 33, 73, 0, 0, 0, 35, 0, 36, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 45, 74, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 33, 78, 0, 0, 0, 35, 0, 36, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 33, 82, 0, 0, 0, 35, 0, 36, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 84, 0, 0, 45, 85, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 86, 0, 0, 45, 87, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 33, 88, 0, 0, 0, 35, 0, 36, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 89, 90, 91, 92, 0, 0, 0, 0, 93, 0, 94, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 103, 0, 0, 45, 104, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 106, 0, 0, 45, 107, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 57, 58, 0, 0, 0, 0, 109, 0, 60, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 45, 111, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 45, 113, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 33, 116, 0, 0, 0, 35, 0, 36, 0,
        // State 95
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 96
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 45, 117, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 45, 119, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 57, 58, 0, 0, 0, 0, 121, 0, 60, 0,
        // State 108
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 113
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        0, 0, 0, 0, 0, 124, 0, 0, 45, 125, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 116
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 121
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 123
        0, 0, 0, 0, 0, 0, 0, 0, 45, 129, 47, 48, 0, 0, 0, 0, 49, 0, 50, 0,
        // State 124
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 125
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 126
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 127
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 128
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 130
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###"".""###,
            r###"":-""###,
            r###""?-""###,
            r###""[""###,
            r###""[]""###,
            r###""]""###,
            r###""_""###,
            r###""|""###,
            r###"r#"[A-Z][A-Za-z0-9_]*"#"###,
            r###"r#"[a-z][A-Za-z0-9_]*"#"###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_TopLevel<
        'input,
    >(
        input: &'input str,
    ) -> Result<TopLevel, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2e_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3a_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3f_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5b_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22___22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_7c_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_5bA_2dZ_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<TopLevel,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("," <Term>) = ",", Term => ActionFn(26);
                let __sym1 = __pop_NtTerm(__symbols);
                let __sym0 = __pop_Term_22_2c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2c_22_20_3cTerm_3e_29(__nt), __end));
                0
            }
            2 => {
                // ("," <Term>)* =  => ActionFn(24);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action24::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22_2c_22_20_3cTerm_3e_29_2a(__nt), __end));
                1
            }
            3 => {
                // ("," <Term>)* = ("," <Term>)+ => ActionFn(25);
                let __sym0 = __pop_Nt_28_22_2c_22_20_3cTerm_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22_2c_22_20_3cTerm_3e_29_2a(__nt), __end));
                1
            }
            4 => {
                // ("," <Term>)+ = ",", Term => ActionFn(37);
                let __sym1 = __pop_NtTerm(__symbols);
                let __sym0 = __pop_Term_22_2c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2c_22_20_3cTerm_3e_29_2b(__nt), __end));
                2
            }
            5 => {
                // ("," <Term>)+ = ("," <Term>)+, ",", Term => ActionFn(38);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_Nt_28_22_2c_22_20_3cTerm_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action38::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22_2c_22_20_3cTerm_3e_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<BoxedTerm> ",") = BoxedTerm, "," => ActionFn(32);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtBoxedTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action32::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cBoxedTerm_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<BoxedTerm> ",")* =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<BoxedTerm> ",")* = (<BoxedTerm> ",")+ => ActionFn(31);
                let __sym0 = __pop_Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<BoxedTerm> ",")+ = BoxedTerm, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtBoxedTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<BoxedTerm> ",")+ = (<BoxedTerm> ",")+, BoxedTerm, "," => ActionFn(44);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtBoxedTerm(__symbols);
                let __sym0 = __pop_Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action44::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // (<PredicateClause>) = PredicateClause => ActionFn(29);
                let __sym0 = __pop_NtPredicateClause(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cPredicateClause_3e_29(__nt), __end));
                6
            }
            12 => {
                // (<PredicateClause>)+ = PredicateClause => ActionFn(47);
                let __sym0 = __pop_NtPredicateClause(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cPredicateClause_3e_29_2b(__nt), __end));
                7
            }
            13 => {
                // (<PredicateClause>)+ = (<PredicateClause>)+, PredicateClause => ActionFn(48);
                let __sym1 = __pop_NtPredicateClause(__symbols);
                let __sym0 = __pop_Nt_28_3cPredicateClause_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cPredicateClause_3e_29_2b(__nt), __end));
                7
            }
            14 => {
                // Atom = r#"[a-z][A-Za-z0-9_]*"# => ActionFn(5);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                8
            }
            15 => {
                // BoxedTerm = Term => ActionFn(6);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBoxedTerm(__nt), __end));
                9
            }
            16 => {
                // Clause = Atom, "(", BoxedTerm, ")" => ActionFn(45);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtBoxedTerm(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action45::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtClause(__nt), __end));
                10
            }
            17 => {
                // Clause = Atom, "(", (<BoxedTerm> ",")+, BoxedTerm, ")" => ActionFn(46);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtBoxedTerm(__symbols);
                let __sym2 = __pop_Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2b(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action46::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtClause(__nt), __end));
                10
            }
            18 => {
                // List = "[]" => ActionFn(8);
                let __sym0 = __pop_Term_22_5b_5d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                11
            }
            19 => {
                // List = "[", ListInternals, "]" => ActionFn(9);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtListInternals(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                11
            }
            20 => {
                // ListInternals = BoxedTerm => ActionFn(10);
                let __sym0 = __pop_NtBoxedTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtListInternals(__nt), __end));
                12
            }
            21 => {
                // ListInternals = BoxedTerm, ",", ListInternals => ActionFn(11);
                let __sym2 = __pop_NtListInternals(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtBoxedTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtListInternals(__nt), __end));
                12
            }
            22 => {
                // ListInternals = BoxedTerm, "|", BoxedTerm => ActionFn(12);
                let __sym2 = __pop_NtBoxedTerm(__symbols);
                let __sym1 = __pop_Term_22_7c_22(__symbols);
                let __sym0 = __pop_NtBoxedTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtListInternals(__nt), __end));
                12
            }
            23 => {
                // Predicate = (<PredicateClause>)+, PredicateClause => ActionFn(13);
                let __sym1 = __pop_NtPredicateClause(__symbols);
                let __sym0 = __pop_Nt_28_3cPredicateClause_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPredicate(__nt), __end));
                13
            }
            24 => {
                // PredicateClause = Rule, "." => ActionFn(14);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_NtRule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPredicateClause(__nt), __end));
                14
            }
            25 => {
                // PredicateClause = Term, "." => ActionFn(15);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPredicateClause(__nt), __end));
                14
            }
            26 => {
                // Rule = Clause, ":-", Term => ActionFn(39);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_3a_2d_22(__symbols);
                let __sym0 = __pop_NtClause(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtRule(__nt), __end));
                15
            }
            27 => {
                // Rule = Clause, ":-", Term, ("," <Term>)+ => ActionFn(40);
                let __sym3 = __pop_Nt_28_22_2c_22_20_3cTerm_3e_29_2b(__symbols);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_3a_2d_22(__symbols);
                let __sym0 = __pop_NtClause(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtRule(__nt), __end));
                15
            }
            28 => {
                // Rule = Atom, ":-", Term => ActionFn(41);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_3a_2d_22(__symbols);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action41::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtRule(__nt), __end));
                15
            }
            29 => {
                // Rule = Atom, ":-", Term, ("," <Term>)+ => ActionFn(42);
                let __sym3 = __pop_Nt_28_22_2c_22_20_3cTerm_3e_29_2b(__symbols);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_3a_2d_22(__symbols);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtRule(__nt), __end));
                15
            }
            30 => {
                // Term = Atom => ActionFn(18);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                16
            }
            31 => {
                // Term = Clause => ActionFn(19);
                let __sym0 = __pop_NtClause(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                16
            }
            32 => {
                // Term = List => ActionFn(20);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                16
            }
            33 => {
                // Term = Var => ActionFn(21);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                16
            }
            34 => {
                // Term = "_" => ActionFn(22);
                let __sym0 = __pop_Term_22___22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                16
            }
            35 => {
                // TopLevel = "?-", Term, "." => ActionFn(1);
                let __sym2 = __pop_Term_22_2e_22(__symbols);
                let __sym1 = __pop_NtTerm(__symbols);
                let __sym0 = __pop_Term_22_3f_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTopLevel(__nt), __end));
                17
            }
            36 => {
                // TopLevel = Predicate => ActionFn(2);
                let __sym0 = __pop_NtPredicate(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTopLevel(__nt), __end));
                17
            }
            37 => {
                // TopLevel = Rule, "." => ActionFn(3);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_NtRule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTopLevel(__nt), __end));
                17
            }
            38 => {
                // TopLevel = Term, "." => ActionFn(4);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTopLevel(__nt), __end));
                17
            }
            39 => {
                // Var = r#"[A-Z][A-Za-z0-9_]*"# => ActionFn(23);
                let __sym0 = __pop_Termr_23_22_5bA_2dZ_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                18
            }
            40 => {
                // __TopLevel = TopLevel => ActionFn(0);
                let __sym0 = __pop_NtTopLevel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 20 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3f_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3f_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22___22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22___22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5bA_2dZ_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5bA_2dZ_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dz_5d_5bA_2dZa_2dz0_2d9___5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2c_22_20_3cTerm_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2c_22_20_3cTerm_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2c_22_20_3cTerm_3e_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Term>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2c_22_20_3cTerm_3e_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2c_22_20_3cTerm_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Term>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2c_22_20_3cTerm_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cBoxedTerm_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Term>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cBoxedTerm_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Term>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Term>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cBoxedTerm_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cPredicateClause_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PredicateClause, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cPredicateClause_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cPredicateClause_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<PredicateClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cPredicateClause_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBoxedTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Term>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBoxedTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClause<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClause(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtListInternals<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtListInternals(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPredicate<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<PredicateClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPredicate(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPredicateClause<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PredicateClause, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPredicateClause(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRule<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Rule, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTopLevel<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TopLevel, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTopLevel(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Var, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____TopLevel<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TopLevel, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____TopLevel(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__TopLevel::parse_TopLevel;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 5;
                            continue;
                        }
                        63 => /* '?' */ {
                            __current_state = 6;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        93 => /* ']' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        93 => /* ']' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TopLevel, usize),
) -> TopLevel
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> TopLevel
{
    TopLevel::Query(t)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<PredicateClause>, usize),
) -> TopLevel
{
    TopLevel::Predicate(__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Rule, usize),
    (_, _, _): (usize, &'input str, usize),
) -> TopLevel
{
    TopLevel::Rule(__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> TopLevel
{
    TopLevel::Fact(__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Atom
{
    __0.trim().to_string()
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, Term, usize),
) -> Box<Term>
{
    Box::new(t)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, a, _): (usize, Atom, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ts, _): (usize, ::std::vec::Vec<Box<Term>>, usize),
    (_, t, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Term
{
    {
     	let mut ts = ts;
     	ts.push(t);
	Term::Clause(Cell::default(), a, ts)
    }
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Term
{
    Term::Constant(Cell::default(), Constant::EmptyList)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Term
{
    __0
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, Box<Term>, usize),
) -> Term
{
    Term::Cons(Cell::default(),
                                t,
				Box::new(Term::Constant(Cell::default(),
				                        Constant::EmptyList)))
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, li, _): (usize, Term, usize),
) -> Term
{
    Term::Cons(Cell::default(),
                                                         t,
							 Box::new(li))
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, t1, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t2, _): (usize, Box<Term>, usize),
) -> Term
{
    Term::Cons(Cell::default(), t1, t2)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, pcs, _): (usize, ::std::vec::Vec<PredicateClause>, usize),
    (_, pc, _): (usize, PredicateClause, usize),
) -> Vec<PredicateClause>
{
    {
        let mut pcs = pcs;
	pcs.push(pc);
	pcs
    }
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Rule, usize),
    (_, _, _): (usize, &'input str, usize),
) -> PredicateClause
{
    PredicateClause::Rule(__0)
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> PredicateClause
{
    PredicateClause::Fact(__0)
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, c, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, h, _): (usize, Term, usize),
    (_, cs, _): (usize, ::std::vec::Vec<Term>, usize),
) -> Rule
{
    Rule { head: (c, h), clauses: cs }
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, a, _): (usize, Atom, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, h, _): (usize, Term, usize),
    (_, cs, _): (usize, ::std::vec::Vec<Term>, usize),
) -> Rule
{
    Rule { head: (Term::Constant(Cell::default(), Constant::Atom(a)), h),
               clauses: cs }
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Term
{
    Term::Constant(Cell::default(), Constant::Atom(__0))
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    __0
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    __0
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Var, usize),
) -> Term
{
    Term::Var(Cell::default(), __0)
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Term
{
    Term::AnonVar
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Var
{
    __0.trim().to_string()
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Term>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Term>, usize),
) -> ::std::vec::Vec<Term>
{
    v
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, PredicateClause, usize),
) -> ::std::vec::Vec<PredicateClause>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<PredicateClause>, usize),
    (_, e, _): (usize, PredicateClause, usize),
) -> ::std::vec::Vec<PredicateClause>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, PredicateClause, usize),
) -> PredicateClause
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Term>>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Term>>, usize),
) -> ::std::vec::Vec<Box<Term>>
{
    v
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Term>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Term>, usize),
) -> ::std::vec::Vec<Box<Term>>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Term>>, usize),
    (_, e, _): (usize, Box<Term>, usize),
) -> ::std::vec::Vec<Box<Term>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> ::std::vec::Vec<Term>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Term>, usize),
    (_, e, _): (usize, Term, usize),
) -> ::std::vec::Vec<Term>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Term, usize),
) -> ::std::vec::Vec<Term>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action26(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Term>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Term, usize),
) -> ::std::vec::Vec<Term>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action26(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
>(
    input: &'input str,
    __0: (usize, Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Term, usize),
) -> Rule
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action24(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
>(
    input: &'input str,
    __0: (usize, Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Term, usize),
    __3: (usize, ::std::vec::Vec<Term>, usize),
) -> Rule
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action25(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
>(
    input: &'input str,
    __0: (usize, Atom, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Term, usize),
) -> Rule
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action24(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
>(
    input: &'input str,
    __0: (usize, Atom, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Term, usize),
    __3: (usize, ::std::vec::Vec<Term>, usize),
) -> Rule
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action25(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Term>, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Term>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action32(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Term>>, usize),
    __1: (usize, Box<Term>, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Term>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action32(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
>(
    input: &'input str,
    __0: (usize, Atom, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Box<Term>, usize),
    __3: (usize, &'input str, usize),
) -> Term
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action30(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
>(
    input: &'input str,
    __0: (usize, Atom, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<Box<Term>>, usize),
    __3: (usize, Box<Term>, usize),
    __4: (usize, &'input str, usize),
) -> Term
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action31(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __1,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
>(
    input: &'input str,
    __0: (usize, PredicateClause, usize),
) -> ::std::vec::Vec<PredicateClause>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<PredicateClause>, usize),
    __1: (usize, PredicateClause, usize),
) -> ::std::vec::Vec<PredicateClause>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action29(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
