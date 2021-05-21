extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use std::fs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[proc_macro]
pub fn start_components(_: TokenStream) -> TokenStream {
    let components = get_components();
    let mut res: String = "".to_owned();

    for c in &components {
        res.push_str(&format!(
            "let component_{0} = {0}::{1}{{\n\tframework: &framework,}};\ncomponent_{0}.run();",
            c,
            c.from_case(Case::Kebab).to_case(Case::Pascal)
        ));
    }
    return res.parse().unwrap();
}

fn get_components() -> Vec<String> {
    let paths = fs::read_dir("./src/components").unwrap();
    let mut paths_vect = vec![];
    for path in paths {
        paths_vect.push(
            path.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".rs", ""),
        );
    }
    let index = paths_vect.iter().position(|r| r == "mod").unwrap();

    paths_vect.remove(index);
    return paths_vect;
}
