# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpH9l3ht/j_V4zCc.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
