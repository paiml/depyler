# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpyawY6u/AmJjUS_nt_j.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
