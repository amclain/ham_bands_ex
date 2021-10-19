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

// #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
// #[derive(Debug, PartialEq, serde::Serialize)]
// #[serde(rename = "Elixir.Bar.Privilege")]
// struct Privilege(ham_bands::Privilege);
// pub struct Privilege {
//     pub mode: &'static str,
//     pub min: u64,
//     pub max: u64,
// }

// impl Deref for Privilege {
//     type Target: ham_bands::Privilege;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

fn list_bands_ex<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let bands = ham_bands::list_bands();
    Ok(bands.encode(env))
}

fn list_license_classes_ex<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let license_classes = ham_bands::list_license_classes();
    Ok(license_classes.encode(env))
}

fn list_frequency_privileges_ex<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let band: &str = args[0].decode()?;
    let license_class: &str = args[1].decode()?;
    let frequency_privileges = ham_bands::list_frequency_privileges(band, license_class);
    // let frequency_privileges: Vec<Privilege> = ham_bands::list_frequency_privileges(band, license_class).into_iter().map(|p| Privilege(p)).collect();

    to_term(env, frequency_privileges).map_err(|err| err.into())
}

// rustler::init!("Elixir.HamBands", [list_bands, list_license_classes]);
