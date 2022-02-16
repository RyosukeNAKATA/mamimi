use crate::python_version::PythonVersion;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Arch {
    X86,
    X64,
    Arm64,
    Armv7l,
    Ppc64le,
    Ppc64,
    s390x,
}

#[cfg(unix)]
/// handle common case: Apple Silicon / Python >= 3.8.9
pub fn get_safe_arch<'a>(arch:&'a Arch,version:&PythonVersion)->&'a Arch{
    use crate::system_info::
}
