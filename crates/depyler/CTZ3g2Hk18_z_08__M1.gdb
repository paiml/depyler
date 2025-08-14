# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpTY8m75/CTZ3g2Hk18_z_08__M1.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
