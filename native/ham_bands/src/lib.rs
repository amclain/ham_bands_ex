use rustler::{Encoder, Env, Error, Term};
use serde_rustler::to_term;

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
    }
}

rustler::rustler_export_nifs!(
    "Elixir.HamBands",
    [
        ("list_bands", 0, list_bands_ex),
        ("list_license_classes", 0, list_license_classes_ex),
        ("list_frequency_privileges", 2, list_frequency_privileges_ex),
    ],
    None
);

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "Elixir.HamBands.Privilege")]
pub struct Privilege {
    pub mode: &'static str,
    pub min: u64,
    pub max: u64,
}

// #[rustler::nif]
pub fn list_bands() -> Vec<&'static str> {
    // vec!["2200m", "630m", "160m", "80m", "60m", "40m", "30m", "20m", "17m",
    // "15m", "12m", "10m", "6m", "2m", "1.25m", "70cm", "33cm", "23cm", "13cm",
    // "9cm", "6cm", "3cm", "12mm", "6mm", "4mm", "2.5mm", "2mm", "1mm"]

    vec!["40m", "20m"]
}

fn list_bands_ex<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let bands = list_bands();
    Ok(bands.encode(env))
}

// #[rustler::nif]
pub fn list_license_classes() -> Vec<&'static str> {
    vec!["E", "A", "G", "N", "T"]
}

fn list_license_classes_ex<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let license_classes = list_license_classes();
    Ok(license_classes.encode(env))
}

// #[rustler::nif]
pub fn list_frequency_privileges(band: &str, license_class: &str) -> Vec<Privilege> {
    match band {
        "40m" => list_frequency_privileges_40m(license_class),
        "20m" => list_frequency_privileges_20m(license_class),
        _ => unimplemented!("Band {} not implemented", band),
    }
}

fn list_frequency_privileges_ex<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let band: &str = args[0].decode()?;
    let license_class: &str = args[1].decode()?;
    let frequency_privileges = list_frequency_privileges(band, license_class);

    to_term(env, frequency_privileges).map_err(|err| err.into())
}

fn list_frequency_privileges_40m(license_class: &str) -> Vec<Privilege> {
    match license_class {
        "E" => vec![
            Privilege {
                mode: "data",
                min: 7_000_000,
                max: 7_125_000,
            },
            Privilege {
                mode: "phone",
                min: 7_125_000,
                max: 7_300_000,
            },
        ],
        "A" => vec![
            Privilege {
                mode: "data",
                min: 7_025_000,
                max: 7_125_000,
            },
            Privilege {
                mode: "phone",
                min: 7_125_000,
                max: 7_300_000,
            },
        ],
        "G" => vec![
            Privilege {
                mode: "data",
                min: 7_025_000,
                max: 7_125_000,
            },
            Privilege {
                mode: "phone",
                min: 7_175_000,
                max: 7_300_000,
            },
        ],
        "N" => vec![
            Privilege {
                mode: "cw",
                min: 7_025_000,
                max: 7_075_000,
            },
            Privilege {
                mode: "cw",
                min: 7_100_000,
                max: 7_125_000,
            },
        ],
        "T" => vec![
            Privilege {
                mode: "cw",
                min: 7_025_000,
                max: 7_075_000,
            },
            Privilege {
                mode: "cw",
                min: 7_100_000,
                max: 7_125_000,
            },
        ],
        _ => panic!("Privileges not found"),
    }
}

fn list_frequency_privileges_20m(license_class: &str) -> Vec<Privilege> {
    match license_class {
        "E" => vec![
            Privilege {
                mode: "data",
                min: 14_000_000,
                max: 14_150_000,
            },
            Privilege {
                mode: "phone",
                min: 14_150_000,
                max: 14_350_000,
            },
        ],
        "A" => vec![
            Privilege {
                mode: "data",
                min: 14_025_000,
                max: 14_150_000,
            },
            Privilege {
                mode: "phone",
                min: 14_175_000,
                max: 14_350_000,
            },
        ],
        "G" => vec![
            Privilege {
                mode: "data",
                min: 14_025_000,
                max: 14_150_000,
            },
            Privilege {
                mode: "phone",
                min: 14_225_000,
                max: 14_350_000,
            },
        ],
        _ => panic!("Privileges not found"),
    }
}

// rustler::init!("Elixir.HamBands", [list_bands, list_license_classes]);
