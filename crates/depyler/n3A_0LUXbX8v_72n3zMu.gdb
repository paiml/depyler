# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpg5sVb0/n3A_0LUXbX8v_72n3zMu.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
