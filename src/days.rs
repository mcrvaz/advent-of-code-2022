use paste::paste;
use seq_macro::seq;

macro_rules! mod_day {
    ($number:literal) => {
        paste! { pub mod [<day$number>]; }
    };
}
seq!(N in 1..=14 {
    mod_day!(N);
});
