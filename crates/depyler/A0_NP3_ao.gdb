# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpPZE5gO/A0_NP3_ao.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
