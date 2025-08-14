# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpXD75pT/uD1_K_V0NCm1.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
