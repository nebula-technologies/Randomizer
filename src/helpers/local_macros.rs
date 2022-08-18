#[macro_export]
macro_rules! automated_impl {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident);
        )+
    ) => {
        $(
            $(#[$docs])*
            #[doc = " ```\n" ]
            #[doc = " use randomizer::Randomizer;\n"]
            #[doc = " \n" ]
            #[doc = concat!(" let randomized = Randomizer::", stringify!($konst), "(12);\n") ]
            #[doc = " \n" ]
            #[doc = concat!(" assert_eq!(randomized.string().unwrap().chars().count(), 12);\n") ]
            #[doc = " ``` "]
            #[allow(non_snake_case)]
             pub fn $konst(length: usize) -> Self {
                Self::new(length, Some(constants::$konst))
            }
        )+
    }
}

pub(crate) use automated_impl;
