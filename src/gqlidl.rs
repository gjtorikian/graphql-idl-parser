use ast::{Definition, Field, FieldType, GraphQLType};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__schema {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::{Definition, Field, FieldType, GraphQLType};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_21_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22implements_22(&'input str),
        Term_22input_22(&'input str),
        Term_22interface_22(&'input str),
        Term_22scalar_22(&'input str),
        Term_22type_22(&'input str),
        Term_22union_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_23_5c_5cs_2a_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Termr_23_22_5b__A_2dZa_2dz_5d_5b__0_2d9A_2dZa_2dz_5d_2a_22_23(&'input str),
        Nt_22_21_22_3f(::std::option::Option<&'input str>),
        Nt_28_3cField_3e_29(Field),
        Nt_28_3cField_3e_29_2a(::std::vec::Vec<Field>),
        Nt_28_3cField_3e_29_2b(::std::vec::Vec<Field>),
        Nt_28_3cName_3e_20_22_2c_22_29(String),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<String>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<String>),
        NtDefinition(Definition),
        NtDefinition_2a(::std::vec::Vec<Definition>),
        NtDefinition_2b(::std::vec::Vec<Definition>),
        NtDescription(String),
        NtDescription_3f(::std::option::Option<String>),
        NtField(Field),
        NtFieldType(FieldType),
        NtFields(Vec<Field>),
        NtGQLTypeWithFields(GraphQLType),
        NtGQLTypeWithoutFields(GraphQLType),
        NtName(String),
        NtName_3f(::std::option::Option<String>),
        Nt____schema(Vec<Definition>),
        Ntschema(Vec<Definition>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 12, 0, 0, 13, 0,
        // State 1
        0, 0, 0, 0, 0, 0, -37, -37, -37, -37, -37, 0, 0, -37, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 12, 0, 0, 13, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 12, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55,
        // State 12
        0, 0, 0, 0, 0, 0, -39, -39, -39, -39, -39, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, -38, -38, -38, -38, -38, 0, 0, -38, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20,
        // State 16
        0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, -56, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, -34, -34, -34, -34, -34, 0, 0, -34, 0,
        // State 19
        0, 0, 0, 0, 0, 0, -56, -56, -56, -56, -56, 0, 0, -56, 0,
        // State 20
        0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 26, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, -33, -33, -33, -33, -33, 0, 0, -33, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 30,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 37,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 30,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 36, 37,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 30,
        // State 27
        0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 37,
        // State 29
        0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 36, 37,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, -6, -6,
        // State 33
        0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, -14, -14, -14, -14, -14, 0, 0, -14, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39,
        // State 36
        0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 30,
        // State 38
        0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 36, 37,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 36, 37,
        // State 41
        0, 0, 0, 0, 0, 0, -13, -13, -13, -13, -13, 0, 0, -13, 0,
        // State 42
        0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 36, 37,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, -11,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 36, 37,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 36, 37,
        // State 47
        0, 0, 0, 0, 0, 0, -20, -20, -20, -20, -20, 0, 0, -20, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, -7, -7,
        // State 49
        0, 0, 0, 0, 0, 0, -16, -16, -16, -16, -16, 0, 0, -16, 0,
        // State 50
        0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70,
        // State 52
        0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 36, 37,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 36, 37,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 36, 37,
        // State 56
        0, 0, 0, 0, 0, 0, -18, -18, -18, -18, -18, 0, 0, -18, 0,
        // State 57
        0, 0, 0, 0, 0, 0, -15, -15, -15, -15, -15, 0, 0, -15, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, -12,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 36, 37,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 36, 37,
        // State 61
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, 0, 0, -24, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 36, 37,
        // State 63
        0, 0, 0, 0, 0, 0, -19, -19, -19, -19, -19, 0, 0, -19, 0,
        // State 64
        0, 0, 0, 0, 0, 0, -28, -28, -28, -28, -28, 0, 0, -28, 0,
        // State 65
        0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70,
        // State 66
        82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45,
        // State 67
        -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, -46,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84,
        // State 69
        -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, -56, -56,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 86, 36, 37,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 36, 37,
        // State 72
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, 0, 0, -22, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 88, 36, 37,
        // State 74
        0, 0, 0, 0, 0, 0, -17, -17, -17, -17, -17, 0, 0, -17, 0,
        // State 75
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, 0, 0, -26, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 89, 36, 37,
        // State 77
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, 0, 0, -23, 0,
        // State 78
        0, 0, 0, 0, 0, 0, -32, -32, -32, -32, -32, 0, 0, -32, 0,
        // State 79
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, 0, 0, -27, 0,
        // State 80
        90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, -44, -44,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, -43, -43,
        // State 82
        91, 0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        -56, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 36, 37,
        // State 85
        0, 0, 0, 0, 0, 0, -21, -21, -21, -21, -21, 0, 0, -21, 0,
        // State 86
        0, 0, 0, 0, 0, 0, -30, -30, -30, -30, -30, 0, 0, -30, 0,
        // State 87
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, 0, 0, -25, 0,
        // State 88
        0, 0, 0, 0, 0, 0, -31, -31, -31, -31, -31, 0, 0, -31, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, -42, -42,
        // State 90
        0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, -48, -48,
        // State 92
        0, 0, 0, 0, 0, 0, -29, -29, -29, -29, -29, 0, 0, -29, 0,
        // State 93
        -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, -47, -47,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -60,
        -37,
        -61,
        0,
        0,
        0,
        -59,
        0,
        0,
        0,
        0,
        0,
        0,
        -38,
        0,
        0,
        0,
        0,
        -34,
        -56,
        0,
        -33,
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
        -14,
        0,
        0,
        0,
        0,
        0,
        0,
        -13,
        0,
        0,
        0,
        0,
        0,
        -20,
        0,
        -16,
        0,
        0,
        0,
        0,
        0,
        0,
        -18,
        -15,
        0,
        0,
        0,
        -24,
        0,
        -19,
        -28,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -22,
        0,
        -17,
        -26,
        0,
        -23,
        -32,
        -27,
        0,
        0,
        0,
        0,
        0,
        -21,
        -30,
        -25,
        -31,
        0,
        0,
        0,
        -29,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 4, 0, 0, 0, 0, 5, 6, 0, 0, 0, 7,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 4, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 16, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 23
        0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0,
        // State 25
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 68, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 54
        0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 68, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 85, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 49, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###"",""###,
            r###"":""###,
            r###""[""###,
            r###""]""###,
            r###""implements""###,
            r###""input""###,
            r###""interface""###,
            r###""scalar""###,
            r###""type""###,
            r###""union""###,
            r###""{""###,
            r###""}""###,
            r###"r#"#\\s*[^\\n]*"#"###,
            r###"r#"[_A-Za-z][_0-9A-Za-z]*"#"###,
        ];
        __ACTION[(__state * 15)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_schema<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Definition>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (12, _) if true => 10,
                (13, _) if true => 11,
                (14, _) if true => 12,
                (0, _) if true => 13,
                (1, _) if true => 14,
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
                let __action = __ACTION[__state * 15 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_21_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22implements_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22input_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22interface_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22scalar_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22type_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22union_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_7b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_7d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_23_5c_5cs_2a_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b__A_2dZa_2dz_5d_5b__0_2d9A_2dZa_2dz_5d_2a_22_23((__tok0)),
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
    ) -> Option<Result<Vec<Definition>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "!"? = "!" => ActionFn(16);
                let __sym0 = __pop_Term_22_21_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22_21_22_3f(__nt), __end));
                0
            }
            2 => {
                // "!"? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22_21_22_3f(__nt), __end));
                0
            }
            3 => {
                // (<Field>) = Field => ActionFn(25);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_29(__nt), __end));
                1
            }
            4 => {
                // (<Field>)* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_29_2a(__nt), __end));
                2
            }
            5 => {
                // (<Field>)* = (<Field>)+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_29_2a(__nt), __end));
                2
            }
            6 => {
                // (<Field>)+ = Field => ActionFn(40);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_29_2b(__nt), __end));
                3
            }
            7 => {
                // (<Field>)+ = (<Field>)+, Field => ActionFn(41);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_29_2b(__nt), __end));
                3
            }
            8 => {
                // (<Name> ",") = Name, "," => ActionFn(22);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                5
            }
            11 => {
                // (<Name> ",")+ = Name, "," => ActionFn(48);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                6
            }
            12 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(49);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action49::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                6
            }
            13 => {
                // Definition = Description, GQLTypeWithFields, Name, "{", "}" => ActionFn(56);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_Term_22_7b_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            14 => {
                // Definition = GQLTypeWithFields, Name, "{", "}" => ActionFn(57);
                let __sym3 = __pop_Term_22_7d_22(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action57::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            15 => {
                // Definition = Description, GQLTypeWithFields, Name, "{", (<Field>)+, "}" => ActionFn(58);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym3 = __pop_Term_22_7b_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action58::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            16 => {
                // Definition = GQLTypeWithFields, Name, "{", (<Field>)+, "}" => ActionFn(59);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action59::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            17 => {
                // Definition = Description, GQLTypeWithFields, Name, "implements", Name, "{", "}" => ActionFn(74);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_NtName(__symbols);
                let __sym3 = __pop_Term_22implements_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action74::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            18 => {
                // Definition = Description, GQLTypeWithFields, Name, "implements", "{", "}" => ActionFn(75);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Term_22implements_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action75::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            19 => {
                // Definition = GQLTypeWithFields, Name, "implements", Name, "{", "}" => ActionFn(76);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtName(__symbols);
                let __sym2 = __pop_Term_22implements_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action76::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            20 => {
                // Definition = GQLTypeWithFields, Name, "implements", "{", "}" => ActionFn(77);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_Term_22_7b_22(__symbols);
                let __sym2 = __pop_Term_22implements_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action77::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            21 => {
                // Definition = Description, GQLTypeWithFields, Name, "implements", (<Name> ",")+, Name, "{", "}" => ActionFn(78);
                let __sym7 = __pop_Term_22_7d_22(__symbols);
                let __sym6 = __pop_Term_22_7b_22(__symbols);
                let __sym5 = __pop_NtName(__symbols);
                let __sym4 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __sym3 = __pop_Term_22implements_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action78::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            22 => {
                // Definition = Description, GQLTypeWithFields, Name, "implements", (<Name> ",")+, "{", "}" => ActionFn(79);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __sym3 = __pop_Term_22implements_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action79::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            23 => {
                // Definition = GQLTypeWithFields, Name, "implements", (<Name> ",")+, Name, "{", "}" => ActionFn(80);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_NtName(__symbols);
                let __sym3 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __sym2 = __pop_Term_22implements_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action80::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            24 => {
                // Definition = GQLTypeWithFields, Name, "implements", (<Name> ",")+, "{", "}" => ActionFn(81);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __sym2 = __pop_Term_22implements_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action81::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            25 => {
                // Definition = Description, GQLTypeWithFields, Name, "implements", Name, "{", (<Field>)+, "}" => ActionFn(82);
                let __sym7 = __pop_Term_22_7d_22(__symbols);
                let __sym6 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_NtName(__symbols);
                let __sym3 = __pop_Term_22implements_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action82::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            26 => {
                // Definition = Description, GQLTypeWithFields, Name, "implements", "{", (<Field>)+, "}" => ActionFn(83);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Term_22implements_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action83::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            27 => {
                // Definition = GQLTypeWithFields, Name, "implements", Name, "{", (<Field>)+, "}" => ActionFn(84);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtName(__symbols);
                let __sym2 = __pop_Term_22implements_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action84::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            28 => {
                // Definition = GQLTypeWithFields, Name, "implements", "{", (<Field>)+, "}" => ActionFn(85);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym3 = __pop_Term_22_7b_22(__symbols);
                let __sym2 = __pop_Term_22implements_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action85::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            29 => {
                // Definition = Description, GQLTypeWithFields, Name, "implements", (<Name> ",")+, Name, "{", (<Field>)+, "}" => ActionFn(86);
                let __sym8 = __pop_Term_22_7d_22(__symbols);
                let __sym7 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym6 = __pop_Term_22_7b_22(__symbols);
                let __sym5 = __pop_NtName(__symbols);
                let __sym4 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __sym3 = __pop_Term_22implements_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym8.2.clone();
                let __nt = super::__action86::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
                let __states_len = __states.len();
                __states.truncate(__states_len - 9);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            30 => {
                // Definition = Description, GQLTypeWithFields, Name, "implements", (<Name> ",")+, "{", (<Field>)+, "}" => ActionFn(87);
                let __sym7 = __pop_Term_22_7d_22(__symbols);
                let __sym6 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __sym3 = __pop_Term_22implements_22(__symbols);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action87::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            31 => {
                // Definition = GQLTypeWithFields, Name, "implements", (<Name> ",")+, Name, "{", (<Field>)+, "}" => ActionFn(88);
                let __sym7 = __pop_Term_22_7d_22(__symbols);
                let __sym6 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_NtName(__symbols);
                let __sym3 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __sym2 = __pop_Term_22implements_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action88::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            32 => {
                // Definition = GQLTypeWithFields, Name, "implements", (<Name> ",")+, "{", (<Field>)+, "}" => ActionFn(89);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __sym2 = __pop_Term_22implements_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action89::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            33 => {
                // Definition = Description, GQLTypeWithoutFields, Name => ActionFn(68);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLTypeWithoutFields(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            34 => {
                // Definition = GQLTypeWithoutFields, Name => ActionFn(69);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLTypeWithoutFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action69::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                7
            }
            35 => {
                // Definition* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDefinition_2a(__nt), __end));
                8
            }
            36 => {
                // Definition* = Definition+ => ActionFn(29);
                let __sym0 = __pop_NtDefinition_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDefinition_2a(__nt), __end));
                8
            }
            37 => {
                // Definition+ = Definition => ActionFn(30);
                let __sym0 = __pop_NtDefinition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDefinition_2b(__nt), __end));
                9
            }
            38 => {
                // Definition+ = Definition+, Definition => ActionFn(31);
                let __sym1 = __pop_NtDefinition(__symbols);
                let __sym0 = __pop_NtDefinition_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDefinition_2b(__nt), __end));
                9
            }
            39 => {
                // Description = r#"#\\s*[^\\n]*"# => ActionFn(15);
                let __sym0 = __pop_Termr_23_22_23_5c_5cs_2a_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDescription(__nt), __end));
                10
            }
            40 => {
                // Description? = Description => ActionFn(26);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDescription_3f(__nt), __end));
                11
            }
            41 => {
                // Description? =  => ActionFn(27);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action27::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDescription_3f(__nt), __end));
                11
            }
            42 => {
                // Field = Description, Name, ":", FieldType, "!" => ActionFn(70);
                let __sym4 = __pop_Term_22_21_22(__symbols);
                let __sym3 = __pop_NtFieldType(__symbols);
                let __sym2 = __pop_Term_22_3a_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action70::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtField(__nt), __end));
                12
            }
            43 => {
                // Field = Name, ":", FieldType, "!" => ActionFn(71);
                let __sym3 = __pop_Term_22_21_22(__symbols);
                let __sym2 = __pop_NtFieldType(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action71::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtField(__nt), __end));
                12
            }
            44 => {
                // Field = Description, Name, ":", FieldType => ActionFn(72);
                let __sym3 = __pop_NtFieldType(__symbols);
                let __sym2 = __pop_Term_22_3a_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action72::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtField(__nt), __end));
                12
            }
            45 => {
                // Field = Name, ":", FieldType => ActionFn(73);
                let __sym2 = __pop_NtFieldType(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action73::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtField(__nt), __end));
                12
            }
            46 => {
                // FieldType = Name => ActionFn(7);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFieldType(__nt), __end));
                13
            }
            47 => {
                // FieldType = "[", Name, "!", "]" => ActionFn(38);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_Term_22_21_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action38::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtFieldType(__nt), __end));
                13
            }
            48 => {
                // FieldType = "[", Name, "]" => ActionFn(39);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFieldType(__nt), __end));
                13
            }
            49 => {
                // Fields =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtFields(__nt), __end));
                14
            }
            50 => {
                // Fields = (<Field>)+ => ActionFn(47);
                let __sym0 = __pop_Nt_28_3cField_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFields(__nt), __end));
                14
            }
            51 => {
                // GQLTypeWithFields = "type" => ActionFn(9);
                let __sym0 = __pop_Term_22type_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGQLTypeWithFields(__nt), __end));
                15
            }
            52 => {
                // GQLTypeWithFields = "interface" => ActionFn(10);
                let __sym0 = __pop_Term_22interface_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGQLTypeWithFields(__nt), __end));
                15
            }
            53 => {
                // GQLTypeWithFields = "input" => ActionFn(11);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGQLTypeWithFields(__nt), __end));
                15
            }
            54 => {
                // GQLTypeWithoutFields = "scalar" => ActionFn(12);
                let __sym0 = __pop_Term_22scalar_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGQLTypeWithoutFields(__nt), __end));
                16
            }
            55 => {
                // GQLTypeWithoutFields = "union" => ActionFn(13);
                let __sym0 = __pop_Term_22union_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGQLTypeWithoutFields(__nt), __end));
                16
            }
            56 => {
                // Name = r#"[_A-Za-z][_0-9A-Za-z]*"# => ActionFn(14);
                let __sym0 = __pop_Termr_23_22_5b__A_2dZa_2dz_5d_5b__0_2d9A_2dZa_2dz_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                17
            }
            57 => {
                // Name? = Name => ActionFn(18);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName_3f(__nt), __end));
                18
            }
            58 => {
                // Name? =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtName_3f(__nt), __end));
                18
            }
            59 => {
                // __schema = schema => ActionFn(0);
                let __sym0 = __pop_Ntschema(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            60 => {
                // schema =  => ActionFn(54);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action54::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntschema(__nt), __end));
                20
            }
            61 => {
                // schema = Definition+ => ActionFn(55);
                let __sym0 = __pop_NtDefinition_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntschema(__nt), __end));
                20
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 21 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22implements_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22implements_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22interface_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22interface_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22scalar_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22scalar_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22type_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22type_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22union_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22union_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_23_5c_5cs_2a_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_23_5c_5cs_2a_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b__A_2dZa_2dz_5d_5b__0_2d9A_2dZa_2dz_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b__A_2dZa_2dz_5d_5b__0_2d9A_2dZa_2dz_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22_21_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22_21_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDefinition<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Definition, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDefinition(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDefinition_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Definition>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDefinition_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDefinition_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Definition>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDefinition_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDescription<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDescription(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDescription_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDescription_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFieldType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FieldType, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFieldType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtGQLTypeWithFields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GraphQLType, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtGQLTypeWithFields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtGQLTypeWithoutFields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GraphQLType, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtGQLTypeWithoutFields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____schema<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Definition>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____schema(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntschema<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Definition>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntschema(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__schema::parse_schema;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\\#)(?u:[\t-\r - \u{85}-\u{85}\u{a0}-\u{a0}\u{1680}-\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}-\u{202f}\u{205f}-\u{205f}\u{3000}-\u{3000}])*(?u:[\u{0}-\t\u{b}-\u{10ffff}])*",
                "^(?u:[A-Z_-_a-z])(?u:[0-9A-Z_-_a-z])*",
                "^(?u:!)",
                "^(?u:,)",
                "^(?u::)",
                "^(?u:\\[)",
                "^(?u:\\])",
                "^(?u:implements)",
                "^(?u:input)",
                "^(?u:interface)",
                "^(?u:scalar)",
                "^(?u:type)",
                "^(?u:union)",
                "^(?u:\\{)",
                "^(?u:\\})",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\\#)(?u:[\t-\r - \u{85}-\u{85}\u{a0}-\u{a0}\u{1680}-\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}-\u{202f}\u{205f}-\u{205f}\u{3000}-\u{3000}])*(?u:[\u{0}-\t\u{b}-\u{10ffff}])*").unwrap(),
                __regex::Regex::new("^(?u:[A-Z_-_a-z])(?u:[0-9A-Z_-_a-z])*").unwrap(),
                __regex::Regex::new("^(?u:!)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
                __regex::Regex::new("^(?u:\\[)").unwrap(),
                __regex::Regex::new("^(?u:\\])").unwrap(),
                __regex::Regex::new("^(?u:implements)").unwrap(),
                __regex::Regex::new("^(?u:input)").unwrap(),
                __regex::Regex::new("^(?u:interface)").unwrap(),
                __regex::Regex::new("^(?u:scalar)").unwrap(),
                __regex::Regex::new("^(?u:type)").unwrap(),
                __regex::Regex::new("^(?u:union)").unwrap(),
                __regex::Regex::new("^(?u:\\{)").unwrap(),
                __regex::Regex::new("^(?u:\\})").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
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
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 15 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
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
    (_, __0, _): (usize, Vec<Definition>, usize),
) -> Vec<Definition>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Definition>, usize),
) -> Vec<Definition>
{
    __0
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, ::std::option::Option<String>, usize),
    (_, g, _): (usize, GraphQLType, usize),
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, f, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Definition
{
    {
    Definition {
      description: match s {
        None => String::new(),
        Some(s) => {
            s
        }
      },
      typename: g,
      name: n,
      implements: Vec::new(),
      fields: f
    }
  }
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, ::std::option::Option<String>, usize),
    (_, g, _): (usize, GraphQLType, usize),
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, ::std::option::Option<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, f, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Definition
{
    {
    Definition {
      description: match s {
        None => String::new(),
        Some(s) => {
            s
        }
      },
      typename: g,
      name: n,
      implements: match e {
          None => v,
          Some(e) => {
              let mut v = v;
              v.push(e);
              v
          }
      },
      fields: f
    }
  }
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, ::std::option::Option<String>, usize),
    (_, g, _): (usize, GraphQLType, usize),
    (_, n, _): (usize, String, usize),
) -> Definition
{
    {
    Definition {
      description: match s {
        None => String::new(),
        Some(s) => {
            s
        }
      },
      typename: g,
      name: n,
      implements: Vec::new(),
      fields: Vec::new()
    }
  }
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, f, _): (usize, ::std::vec::Vec<Field>, usize),
) -> Vec<Field>
{
    f
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, ::std::option::Option<String>, usize),
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, FieldType, usize),
    (_, r, _): (usize, ::std::option::Option<&'input str>, usize),
) -> Field
{
    {
    Field {
      description: match s {
        None => String::new(),
        Some(s) => {
            s
        }
      },
      name: n,
      fieldtype: t,
      nullable: match r {
        None => true,
        Some(r) => false
      }
    }
  }
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, String, usize),
) -> FieldType
{
    {
    FieldType {
      list: false,
      name: n,
      nullable: true
    }
  }
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, String, usize),
    (_, r, _): (usize, ::std::option::Option<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> FieldType
{
    {
    FieldType {
      name: n,
      list: true,
      nullable: match r {
        None => true,
        Some(r) => false
      }
    }
  }
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> GraphQLType
{
    GraphQLType::ObjectType
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> GraphQLType
{
    GraphQLType::InterfaceType
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> GraphQLType
{
    GraphQLType::InputObjectType
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> GraphQLType
{
    GraphQLType::ScalarType
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> GraphQLType
{
    GraphQLType::UnionType
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_string()
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    s[1..s.len()].trim().to_string()
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<&'input str>
{
    None
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<String>
{
    None
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<String>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
) -> ::std::vec::Vec<String>
{
    v
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Field>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
) -> ::std::vec::Vec<Field>
{
    v
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> Field
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<String>
{
    None
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Definition>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Definition>, usize),
) -> ::std::vec::Vec<Definition>
{
    v
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Definition, usize),
) -> ::std::vec::Vec<Definition>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Definition>, usize),
    (_, e, _): (usize, Definition, usize),
) -> ::std::vec::Vec<Definition>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, e, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, FieldType, usize),
    __4: (usize, &'input str, usize),
) -> Field
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action16(
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, FieldType, usize),
) -> Field
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
) -> FieldType
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action16(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> FieldType
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
>(
    input: &'input str,
    __0: (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action25(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<Field>, usize),
    __5: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action24(
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, ::std::option::Option<String>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __6.2.clone();
    let __end0 = __7.0.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, ::std::option::Option<String>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, ::std::vec::Vec<Field>, usize),
    __8: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action24(
        input,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __8,
    )
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Field>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action22(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action49<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action22(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action50<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::option::Option<String>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, ::std::option::Option<String>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action21(
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::option::Option<String>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, ::std::vec::Vec<Field>, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, ::std::option::Option<String>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, ::std::vec::Vec<Field>, usize),
    __8: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action21(
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
pub fn __action54<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Definition>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action28(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action55<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Definition>, usize),
) -> Vec<Definition>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<Field>, usize),
    __5: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action59<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<Field>, usize),
    __4: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action60<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::option::Option<String>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::option::Option<String>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, ::std::option::Option<String>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action63<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<String>, usize),
    __4: (usize, ::std::option::Option<String>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action64<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::option::Option<String>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, ::std::vec::Vec<Field>, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action65<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::option::Option<String>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, ::std::vec::Vec<Field>, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action66<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, ::std::option::Option<String>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, ::std::vec::Vec<Field>, usize),
    __8: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
pub fn __action67<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<String>, usize),
    __4: (usize, ::std::option::Option<String>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, ::std::vec::Vec<Field>, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action68<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action69<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action70<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, FieldType, usize),
    __4: (usize, &'input str, usize),
) -> Field
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action71<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, FieldType, usize),
    __3: (usize, &'input str, usize),
) -> Field
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action72<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, FieldType, usize),
) -> Field
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action73<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, FieldType, usize),
) -> Field
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action74<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, String, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action18(
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action75<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action76<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, String, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action18(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action77<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action78<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, String, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action18(
        input,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action79<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action80<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<String>, usize),
    __4: (usize, String, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action18(
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action81<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<String>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action82<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, String, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, ::std::vec::Vec<Field>, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action18(
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action83<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, ::std::vec::Vec<Field>, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action84<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, String, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, ::std::vec::Vec<Field>, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action18(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action85<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<Field>, usize),
    __5: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action86<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, String, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, ::std::vec::Vec<Field>, usize),
    __8: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action18(
        input,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
pub fn __action87<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, GraphQLType, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, ::std::vec::Vec<String>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, ::std::vec::Vec<Field>, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action88<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<String>, usize),
    __4: (usize, String, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, ::std::vec::Vec<Field>, usize),
    __7: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action18(
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
pub fn __action89<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<String>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, ::std::vec::Vec<Field>, usize),
    __6: (usize, &'input str, usize),
) -> Definition
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action19(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
        __6,
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
