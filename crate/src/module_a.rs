#[cxx_qt::bridge(cxx_file_stem="module_a")]
pub mod qobject {
    unsafe extern "RustQt" {
        #[qobject]
        type ModuleA = super::ModuleARust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn module_a_fn(self: &ModuleA);
    }
}

#[derive(Default)]
pub struct ModuleBRust {}

impl qobject::ModuleB {
    pub fn module_b_fn(self: &qobject::ModuleB) {
        println!("Hello world module A!");
    }
}