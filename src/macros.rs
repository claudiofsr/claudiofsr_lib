pub mod svec {
    #![macro_use]
    #[macro_export]
    /**
    Create a `Vec<String>` from `Vec<&str>`.

    Example:
    ```
        use claudiofsr_lib::svec;

        let v1: Vec<String> = svec![
            "this",
            "that",
            "the other", // with or without a comma at the end
        ];
        let result1 = vec![
            String::from("this"),
            String::from("that"),
            String::from("the other"),
        ];

        let v2 = svec![ "a", "1", "abc", "", "foobar"];
        let result2 = vec![
            "a",
            "1",
            "abc",
            "",
            "foobar",
        ].iter().map(ToString::to_string).collect::<Vec<String>>();

        assert_eq!(v1, result1);
        assert_eq!(v2, result2);
    ```
    <https://doc.rust-lang.org/book/ch19-06-macros.html>

    <https://doc.rust-lang.org/std/macro.vec.html>
    */
    macro_rules! svec {
        ( $($x:expr_2021),+ $(,)?) => {
            {
                Vec::from([$(String::from($x)),*])
            }
        };
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn macro_svec_works() {
            let v = dbg!(svec!["this", "that", "the other", "123"]);
            let x: Vec<String> = v;
            assert_eq!(
                x,
                [
                    String::from("this"),
                    String::from("that"),
                    String::from("the other"),
                    String::from("123"),
                ]
            );
        }
    }
}

pub mod match_cast {
    #![macro_use]
    #[macro_export]
    /**
    Match through different types.

    Example:
    ```
        use std::any::Any;
        use chrono::NaiveDate;
        use claudiofsr_lib::match_cast;

        let opt_u8: Option<u8> = None;
        let opt_u32: Option<u32> = Some(5);
        let float64: f64 = 32.00870000;
        let string: String = String::from("foo bar baz");
        let opt_naivedate: Option<NaiveDate> = NaiveDate::from_ymd_opt(2015, 3, 14);

        // With std::any::Any it is possible to obtain a vector with different types:
        let values: Vec<&dyn Any> = vec![
            &opt_u8,
            &opt_u32,
            &float64,
            &string,
            &opt_naivedate,
        ];

        // Get lengths of all items in the vector.
        let lengths: Vec<usize> = values
            .into_iter()
            .map(|value| {
                let opt_value_len: Option<usize> = match_cast!( value {
                    val as Option<u8> => {
                        val.as_ref().map(|s| s.to_string().chars().count())
                    },
                    val as Option<u32> => {
                        val.as_ref().map(|s| s.to_string().chars().count())
                    },
                    val as f64 => {
                        Some(val.to_string().chars().count())
                    },
                    val as String => {
                        Some(val.chars().count())
                    },
                    val as Option<NaiveDate> => {
                        // eprintln!("val: {val:?}"); // Some(2015-03-14)
                        val.as_ref().map(|date| date.to_string().chars().count())
                    }
                });

                opt_value_len.unwrap_or(0)
            })
            .collect();

        assert_eq!(
            lengths,
            [0, 1, 7, 11, 10]
        );
    ```

    Font: <https://github.com/therustmonk/match_cast/blob/master/src/lib.rs>
    */
    macro_rules! match_cast {
        ($any:ident { $( $bind:ident as $patt:ty => $body:block $(,)? )+ }) => {{
            let downcast = || {
                $(
                if let Some($bind) = $any.downcast_ref::<$patt>() {
                    return $body;
                }
                )+
                None
            };
            downcast()
        }};
    }

    #[cfg(test)]
    mod tests {
        use chrono::NaiveDate;
        use std::any::Any;

        #[test]
        fn macro_match_cast_works() {
            let opt_u8: Option<u8> = None;
            let opt_u32: Option<u32> = Some(5);
            let opt_i64: Option<i64> = Some(83);
            let opt_f32: Option<f32> = Some(5.78);
            let float64: f64 = 32.00870000;
            let string: String = String::from("foo bar baz");
            let strings: Vec<String> = svec!["a", "ab", "abc"];
            let opt_naivedate: Option<NaiveDate> = NaiveDate::from_ymd_opt(2015, 3, 14);
            let naivedate: NaiveDate = NaiveDate::from_ymd_opt(2015, 3, 14).expect("date");

            // Vector with different types
            let values: Vec<&dyn Any> = vec![
                &opt_u8,
                &opt_u32,
                &opt_i64,
                &opt_f32,
                &float64,
                &string,
                &strings,
                &opt_naivedate,
                &naivedate,
            ];

            // Get lengths of all items in the vector.
            let lengths: Vec<usize> = values
                .into_iter()
                .map(|value| {
                    let opt_value_len: Option<usize> = match_cast!( value {

                        val as Option<u8> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<u16> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<u32> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<u64> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<u128> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<usize> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },

                        val as Option<i8> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<i16> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<i32> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<i64> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<i128> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<isize> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },

                        val as Option<f32> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },
                        val as Option<f64> => {
                            val.as_ref().map(|s| s.to_string().chars().count())
                        },

                        val as f32 => {
                            Some(val.to_string().chars().count())
                        },
                        val as f64 => {
                            Some(val.to_string().chars().count())
                        },

                        val as String => {
                            Some(val.chars().count())
                        },
                        val as Option<String> => {
                            val.as_ref().map(|s| s.chars().count())
                        },
                        val as Vec<String> => {
                            Some(val.iter().map(|s| s.chars().count()).sum())
                        },

                        val as NaiveDate => {
                            Some(val.to_string().chars().count())
                        },
                        val as Option<NaiveDate> => {
                            // eprintln!("val: {val:?}"); // Some(2015-03-14)
                            val.as_ref().map(|date| date.to_string().chars().count())
                        }
                    });

                    opt_value_len.unwrap_or(0)
                })
                .collect();

            assert_eq!(lengths, [0, 1, 2, 4, 7, 11, 6, 10, 10]);
        }
    }
}
