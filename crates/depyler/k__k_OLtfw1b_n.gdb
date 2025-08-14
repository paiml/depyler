# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpokTg3i/k__k_OLtfw1b_n.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
