# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpsLflL5/Vnx2yff_I9d4LYc_bF1A.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
