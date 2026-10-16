pub mod PkgManager;
pub mod PushInterpret;

fn main() {
    let mainfile = crate::PkgManager::PkgManager::mainfile();
    crate::PkgManager::PkgManager::check_pkgmanager();
    println!("{}", crate::PushInterpret::PushInterpret::interpret(&mainfile));
    println!("{}", crate::PushInterpret::PushInterpret::InterpretBytecode(&crate::PushInterpret::PushInterpret::ConvertToBytecode(&mainfile)));
}