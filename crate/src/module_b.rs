#[cxx_qt::bridge(cxx_file_stem="module_b")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-gen/module_a.cxxqt.h");
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = "ModuleA"]
        type ModuleB = super::ModuleBRust;
    }
}

#[derive(Default)]
pub struct ModuleBRust {}

impl qobject::ModuleB {}