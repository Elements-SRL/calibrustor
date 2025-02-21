use std::fmt::Debug;

pub trait Uom: Debug + Clone + Copy + Sized {
    fn get_label(&self) -> String;
}

macro_rules! create_uoms {
    [$(($bu:ident, $l: expr)),* $(,)?] => {
        $(
        #[derive(PartialEq, Clone, Copy, Debug, Default, Eq, PartialOrd, Ord, Hash)]
            pub struct $bu;
            impl Uom for $bu {
                fn get_label(&self) -> String {
                    $l.to_string()
                }
            }

        )*
    };
}

create_uoms![(Volt, "V"), (Ampere, "A"), (Hertz, "Hz")];
