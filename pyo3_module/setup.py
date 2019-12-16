from setuptools import find_packages, setup

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess
    import sys

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust", "--user"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

setup_requires = ['setuptools-rust>=0.10.2']
#install_requires = ['numpy']

setup(
    name='rpi3d',
    version='0.1.0',
    description='Python wrapper on rust_pi3d using pyo3',
    rust_extensions=[RustExtension(
        'rpi3d.rpi3d',
        './Cargo.toml',
    )],
    #install_requires=install_requires,
    setup_requires=setup_requires,
    packages=find_packages(),
    zip_safe=False,
)