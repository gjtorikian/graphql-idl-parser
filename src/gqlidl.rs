use ast::{Definition, GraphQLType};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__schema {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::{Definition, GraphQLType};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22scalar_22(&'input str),
        Term_22type_22(&'input str),
        Termr_23_22_23_5c_5cs_2a_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Termr_23_22_5b__A_2dZa_2dz_5d_5b__0_2d9A_2dZa_2dz_5d_2a_22_23(&'input str),
        NtDefinition(Definition),
        NtDefinition_2a(::std::vec::Vec<Definition>),
        NtDefinition_2b(::std::vec::Vec<Definition>),
        NtDescription(String),
        NtDescription_3f(::std::option::Option<String>),
        NtGQLType(GraphQLType),
        NtName(String),
        Nt____schema(Vec<Definition>),
        Ntschema(Vec<Definition>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        7, 8, 9, 0,
        // State 1
        -5, -5, -5, 0,
        // State 2
        7, 8, 9, 0,
        // State 3
        7, 8, 0, 0,
        // State 4
        0, 0, 0, 13,
        // State 5
        0, 0, 0, 0,
        // State 6
        0, 0, 0, -10,
        // State 7
        0, 0, 0, -11,
        // State 8
        -7, -7, 0, 0,
        // State 9
        -6, -6, -6, 0,
        // State 10
        0, 0, 0, 13,
        // State 11
        -2, -2, -2, 0,
        // State 12
        -12, -12, -12, 0,
        // State 13
        -1, -1, -1, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -14,
        -5,
        -15,
        0,
        0,
        -13,
        0,
        0,
        0,
        -6,
        0,
        -2,
        -12,
        -1,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 0, 3, 4, 0, 5, 0, 0, 6,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        10, 0, 0, 4, 0, 5, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 12, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 14, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""scalar""###,
            r###""type""###,
            r###"r#"#\\s*[^\\n]*"#"###,
            r###"r#"[_A-Za-z][_0-9A-Za-z]*"#"###,
        ];
        __ACTION[(__state * 4)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (0, _) if true => 2,
                (1, _) if true => 3,
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
                let __action = __ACTION[__state * 4 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22scalar_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22type_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_23_5c_5cs_2a_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
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
                // Definition = Description, GQLType, Name => ActionFn(15);
                let __sym2 = __pop_NtName(__symbols);
                let __sym1 = __pop_NtGQLType(__symbols);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                0
            }
            2 => {
                // Definition = GQLType, Name => ActionFn(16);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_NtGQLType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                0
            }
            3 => {
                // Definition* =  => ActionFn(9);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action9::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDefinition_2a(__nt), __end));
                1
            }
            4 => {
                // Definition* = Definition+ => ActionFn(10);
                let __sym0 = __pop_NtDefinition_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDefinition_2a(__nt), __end));
                1
            }
            5 => {
                // Definition+ = Definition => ActionFn(11);
                let __sym0 = __pop_NtDefinition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDefinition_2b(__nt), __end));
                2
            }
            6 => {
                // Definition+ = Definition+, Definition => ActionFn(12);
                let __sym1 = __pop_NtDefinition(__symbols);
                let __sym0 = __pop_NtDefinition_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDefinition_2b(__nt), __end));
                2
            }
            7 => {
                // Description = r#"#\\s*[^\\n]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_23_5c_5cs_2a_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDescription(__nt), __end));
                3
            }
            8 => {
                // Description? = Description => ActionFn(7);
                let __sym0 = __pop_NtDescription(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDescription_3f(__nt), __end));
                4
            }
            9 => {
                // Description? =  => ActionFn(8);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action8::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDescription_3f(__nt), __end));
                4
            }
            10 => {
                // GQLType = "scalar" => ActionFn(3);
                let __sym0 = __pop_Term_22scalar_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGQLType(__nt), __end));
                5
            }
            11 => {
                // GQLType = "type" => ActionFn(4);
                let __sym0 = __pop_Term_22type_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGQLType(__nt), __end));
                5
            }
            12 => {
                // Name = r#"[_A-Za-z][_0-9A-Za-z]*"# => ActionFn(5);
                let __sym0 = __pop_Termr_23_22_5b__A_2dZa_2dz_5d_5b__0_2d9A_2dZa_2dz_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                6
            }
            13 => {
                // __schema = schema => ActionFn(0);
                let __sym0 = __pop_Ntschema(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            14 => {
                // schema =  => ActionFn(13);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action13::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntschema(__nt), __end));
                8
            }
            15 => {
                // schema = Definition+ => ActionFn(14);
                let __sym0 = __pop_NtDefinition_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntschema(__nt), __end));
                8
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 9 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_NtGQLType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GraphQLType, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtGQLType(__v), __r) => (__l, __v, __r),
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
                "^(?u:scalar)",
                "^(?u:type)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\\#)(?u:[\t-\r - \u{85}-\u{85}\u{a0}-\u{a0}\u{1680}-\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}-\u{202f}\u{205f}-\u{205f}\u{3000}-\u{3000}])*(?u:[\u{0}-\t\u{b}-\u{10ffff}])*").unwrap(),
                __regex::Regex::new("^(?u:[A-Z_-_a-z])(?u:[0-9A-Z_-_a-z])*").unwrap(),
                __regex::Regex::new("^(?u:scalar)").unwrap(),
                __regex::Regex::new("^(?u:type)").unwrap(),
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
                    for __i in 0 .. 4 {
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
) -> Definition
{
    {
    Definition {
      gql_type: g,
      name: n,
      description: match s {
        None => String::new(),
        Some(s) => {
            s
        }
      }
    }
  }
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> GraphQLType
{
    GraphQLType::ScalarType
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> GraphQLType
{
    GraphQLType::ObjectType
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_string()
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    s[1..s.len()].trim().to_string()
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action8<
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
pub fn __action9<
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
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Definition>, usize),
) -> ::std::vec::Vec<Definition>
{
    v
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Definition, usize),
) -> ::std::vec::Vec<Definition>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action12<
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
pub fn __action13<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Definition>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action9(
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
pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Definition>, usize),
) -> Vec<Definition>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action10(
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
pub fn __action15<
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
    let __temp0 = __action7(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: (usize, GraphQLType, usize),
    __1: (usize, String, usize),
) -> Definition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action8(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __temp0,
        __0,
        __1,
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
