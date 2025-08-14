# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpjb9dUt/XlC_65_hkmc_V_nW8.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
