# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpdU8oFv/ayB_N_A.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
