extern crate js_sys;
extern crate wasm_bindgen;
extern crate nom;

mod jess_utils;

use jess_utils::matches::{
    name_braced_list::matches_common_curly_braced_list_comma_sep,
    css_directive::capture_common_css_directive_name
};

use regex::Regex;

use wasm_bindgen::prelude::*;
use nom::{
    IResult,
    bytes::complete::{
        tag,
        take_while_m_n,
    },
    combinator::map_res,
    sequence::tuple
};





#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug,PartialEq)]
struct ImportDirective<'a> {
    path: &'a str,
    parts: Vec<String>,
    default_name: &'a str,
}


// fn parse_common_jess_path_to_file (source: &str) -> String {
//     // path within quotes, single or double
//     // path can omit ext
//     return String::from("");
// }

// fn parse_directive_import_default_name (source: &str) -> IResult<&str, String> {
//     let default_name: String = parse_common_name(source);
//     // i/o source, name if valid or errors
//     return Ok(( source, default_name ));
// }

// fn parse_directive_import_parts (source: &str) -> Vec<String> {
//     let parts: Vec<String> = vec!();
//     // For part of {a, b, c} push
//     parts.push(String::from(""));
//     parts.push(String::from(""));
//     parts.push(String::from(""));
//     return parts;
// }

// fn parse_directive_import_path (source: &str) -> String {
//     return String::from("");
// }

// fn parse_directive_import(source: &str, useDefault: bool) -> IResult<&str, ImportDirective> {

//     // when match @import get righthand side
//     let (source, _) = tag("@import")(source)?;

//     if useDefault {
//         // @import name from '<path>';
//         let (source, (defaultName, path)) = tuple((
//             parse_directive_import_default_name,
//             parse_directive_import_path
//         ))(source)?;

//         // wrapped result
//         return Ok((
//             source,
//             ImportDirective {
//                 //
//                 defaultName: defaultName,
//                 /**
//                  * does this import have parts
//                 */
//                 parts: vec!(),
//                 /**
//                  * The path to import
//                 */
//                 path: path,
//             }
//         ));
//     } else {
//         // @import {} from '<path>';
//         let (source, (parts, path)) = tuple((
//             parse_directive_import_parts,
//             parse_directive_import_path
//         ))(source)?;

//         // wrapped result
//         return Ok((
//             source,
//             ImportDirective {
//                 //
//                 defaultName: "",
//                 /**
//                  * does this import have parts
//                 */
//                 parts: parts,
//                 /**
//                  * The path to import
//                 */
//                 path: path,
//             }
//         ));
//     }
// }

pub fn grammer_import_names (source: &str) {
    
}

pub fn grammer_import_default (source: &str) {

}

pub fn grammer_import_get_error () {
    
}

pub fn grammer_import_type (source: &str) -> String {
    let (_input, directive) = capture_common_css_directive_name(&source);

    return String::from(directive);
}

#[wasm_bindgen]
pub fn compile(source: &str) -> String {
    grammer_import_get_error();
    // let import_uses_default_name = true;
    // let parseImports = parse_directive_import(source, import_uses_default_name);

    return String::from(source);
}


mod tests {
    use super::{
        grammer_import_get_error,
        grammer_import_type,
        grammer_import_default,
        grammer_import_names
    };

    #[test]
    fn test_import_type_requested_import () {

        let import_type = grammer_import_type("@import");
        assert_eq!(import_type, "import");
    }
}