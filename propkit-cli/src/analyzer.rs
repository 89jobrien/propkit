// propkit-cli analyzer — syn-based trait and signature analysis
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::{HashMap, HashSet};
use std::path::Path;
use syn::visit::Visit;
use syn::{ItemFn, ItemImpl, ItemStruct, Visibility};

#[derive(Debug, Clone)]
pub struct Property {
    pub confidence: Confidence,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Confidence::High => write!(f, "high"),
            Confidence::Medium => write!(f, "medium"),
            Confidence::Low => write!(f, "low"),
        }
    }
}

#[derive(Debug)]
pub struct TypeInfo {
    pub name: String,
    pub derives: Vec<String>,
    pub impl_traits: Vec<String>,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub struct FnInfo {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub struct FileAnalysis {
    pub path: String,
    pub types: Vec<TypeInfo>,
    pub functions: Vec<FnInfo>,
}

struct Collector {
    types: HashMap<String, (Vec<String>, Vec<String>)>,
    functions: Vec<FnSig>,
    impl_traits: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct FnSig {
    name: String,
    is_pub: bool,
    param_types: Vec<String>,
    return_type: Option<String>,
    takes_mut_ref: bool,
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let name = node.ident.to_string();
        let derives = extract_derives(&node.attrs);
        self.types.insert(name, (derives, Vec::new()));
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        if let Some((_, trait_path, _)) = &node.trait_
            && let syn::Type::Path(type_path) = &*node.self_ty
        {
            let type_name = type_path
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();
            let trait_name = trait_path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();
            self.impl_traits
                .entry(type_name)
                .or_default()
                .push(trait_name);
        }
        syn::visit::visit_item_impl(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let is_pub = matches!(node.vis, Visibility::Public(_));
        let name = node.sig.ident.to_string();
        let mut param_types = Vec::new();
        let mut takes_mut_ref = false;

        for input in &node.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = input {
                let ty = type_to_string(&pat_type.ty);
                if ty.starts_with("&mut") {
                    takes_mut_ref = true;
                }
                param_types.push(ty);
            }
        }

        let return_type = match &node.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(type_to_string(ty)),
        };

        self.functions.push(FnSig {
            name,
            is_pub,
            param_types,
            return_type,
            takes_mut_ref,
        });
        syn::visit::visit_item_fn(self, node);
    }
}

fn type_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string().replace(' ', "")
}

fn extract_derives(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut derives = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("derive")
            && let Ok(nested) = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            )
        {
            for path in nested {
                if let Some(ident) = path.segments.last() {
                    derives.push(ident.ident.to_string());
                }
            }
        }
    }
    derives
}

pub fn analyze_file(path: &Path) -> Option<FileAnalysis> {
    let content = std::fs::read_to_string(path).ok()?;
    let syntax: syn::File = syn::parse_file(&content).ok()?;

    let mut collector = Collector {
        types: HashMap::new(),
        functions: Vec::new(),
        impl_traits: HashMap::new(),
    };
    collector.visit_file(&syntax);

    // Merge impl traits into types
    for (type_name, traits) in &collector.impl_traits {
        if let Some((_, impl_traits)) = collector.types.get_mut(type_name) {
            impl_traits.extend(traits.clone());
        }
    }

    let mut types = Vec::new();
    for (name, (derives, impl_traits)) in &collector.types {
        let mut properties = Vec::new();
        let all_traits: HashSet<&str> = derives
            .iter()
            .chain(impl_traits.iter())
            .map(|s| s.as_str())
            .collect();

        // Trait-based analysis
        if all_traits.contains("Eq") && all_traits.contains("Hash") {
            properties.push(Property {
                confidence: Confidence::High,
                description: "hash/eq consistency".into(),
            });
        }
        if all_traits.contains("Ord") {
            properties.push(Property {
                confidence: Confidence::High,
                description: "transitivity, antisymmetry, totality".into(),
            });
        }
        if all_traits.contains("PartialOrd") && all_traits.contains("PartialEq") {
            properties.push(Property {
                confidence: Confidence::High,
                description: "reflexivity, antisymmetry".into(),
            });
        }
        if all_traits.contains("Clone") {
            properties.push(Property {
                confidence: Confidence::High,
                description: "clone equality".into(),
            });
        }
        if all_traits.contains("Serialize") && all_traits.contains("Deserialize") {
            properties.push(Property {
                confidence: Confidence::High,
                description: "serde roundtrip".into(),
            });
        }
        if all_traits.contains("Display") && all_traits.contains("FromStr") {
            properties.push(Property {
                confidence: Confidence::High,
                description: "parse/display roundtrip".into(),
            });
        }
        if all_traits.contains("Default") {
            properties.push(Property {
                confidence: Confidence::High,
                description: "default doesn't panic".into(),
            });
        }

        if !properties.is_empty() {
            let derives_display: Vec<&str> = derives.iter().map(|s| s.as_str()).collect();
            let mut info = TypeInfo {
                name: name.clone(),
                derives: derives_display.into_iter().map(String::from).collect(),
                impl_traits: impl_traits.clone(),
                properties,
            };
            // Sort properties for deterministic output
            info.properties
                .sort_by(|a, b| a.description.cmp(&b.description));
            types.push(info);
        }
    }
    types.sort_by(|a, b| a.name.cmp(&b.name));

    // Signature-based analysis
    let mut functions = Vec::new();
    let fn_names: HashSet<String> = collector.functions.iter().map(|f| f.name.clone()).collect();

    for sig in &collector.functions {
        if !sig.is_pub {
            continue;
        }
        let mut properties = Vec::new();

        // fn foo(T) -> T: idempotence candidate
        if sig.param_types.len() == 1
            && sig.return_type.is_some()
            && sig.return_type.as_ref() == Some(&sig.param_types[0])
        {
            properties.push(Property {
                confidence: Confidence::Low,
                description: "idempotence candidate".into(),
            });
        }

        // fn sort(&mut [T]) / -> Vec<T>: length/element preservation
        if (sig.name.contains("sort") && sig.takes_mut_ref)
            || (sig.name.contains("sort")
                && sig.return_type.as_ref().is_some_and(|r| r.contains("Vec")))
        {
            properties.push(Property {
                confidence: Confidence::Medium,
                description: "length/element preservation".into(),
            });
        }

        // fn encode + fn decode: codec roundtrip
        if sig.name.contains("encode") && fn_names.contains("decode") {
            properties.push(Property {
                confidence: Confidence::Medium,
                description: "codec roundtrip".into(),
            });
        }

        // fn (T, T) -> T: commutativity candidate
        if sig.param_types.len() == 2
            && sig.param_types[0] == sig.param_types[1]
            && sig.return_type.as_ref() == Some(&sig.param_types[0])
        {
            properties.push(Property {
                confidence: Confidence::Low,
                description: "commutativity candidate".into(),
            });
        }

        if !properties.is_empty() {
            functions.push(FnInfo {
                name: sig.name.clone(),
                properties,
            });
        }
    }
    functions.sort_by(|a, b| a.name.cmp(&b.name));

    Some(FileAnalysis {
        path: path.display().to_string(),
        types,
        functions,
    })
}

pub fn format_analysis(analyses: &[FileAnalysis]) -> String {
    let mut out = String::new();
    let mut total_types = 0;
    let mut total_fns = 0;
    let mut total_props = 0;

    for analysis in analyses {
        if analysis.types.is_empty() && analysis.functions.is_empty() {
            continue;
        }
        out.push_str(&analysis.path);
        out.push_str(":\n");

        for ty in &analysis.types {
            total_types += 1;
            let all_traits: Vec<&str> = ty
                .derives
                .iter()
                .chain(ty.impl_traits.iter())
                .map(|s| s.as_str())
                .collect();
            out.push_str(&format!(
                "  {} (derives: {})\n",
                ty.name,
                all_traits.join(", ")
            ));
            for prop in &ty.properties {
                total_props += 1;
                out.push_str(&format!(
                    "    - [{}] {}\n",
                    prop.confidence, prop.description
                ));
            }
            out.push('\n');
        }

        for func in &analysis.functions {
            total_fns += 1;
            out.push_str(&format!("  {}()\n", func.name));
            for prop in &func.properties {
                total_props += 1;
                out.push_str(&format!(
                    "    - [{}] {}\n",
                    prop.confidence, prop.description
                ));
            }
            out.push('\n');
        }
    }

    out.push_str(&format!(
        "  {} types, {} functions, {} suggested properties\n",
        total_types, total_fns, total_props
    ));
    out
}
