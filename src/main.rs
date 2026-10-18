pub mod PkgManager;
pub mod PushInterpret;

fn main() {
    let mainfile: String = crate::PkgManager::PkgManager::mainfile();
    crate::PkgManager::PkgManager::check_pkgmanager();
    println!("{}", crate::PushInterpret::PushInterpret::interpret(&mainfile));
}