from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="sp_rs",
    version="0.1",
    rust_extensions=[RustExtension("sp.sp_rs", "Cargo.toml")],
    packages=["sp"],
    zip_safe=False,
)