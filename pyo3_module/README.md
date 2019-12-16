Start of an attempt to make a python wrapper for rust_pi3d using pyo3.
You should be able to compile the module either:

    in pyo3_module run ``python3 setup.py bdist_wheel`` then open the
    whl file in ``dist`` and extract ``rpi3d/rpi3d-0.1....so`` to the 
    ``pyo3_module/test`` directory. You should be able to run test1.py

or:

    in pyo3_module run ``cargo build --release`` then copy and rename
    ``librpi3d.so`` to ``pyo3_module/test/rpi3d.so``
