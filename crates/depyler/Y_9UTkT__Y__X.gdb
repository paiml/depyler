# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpgMU3aB/Y_9UTkT__Y__X.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
