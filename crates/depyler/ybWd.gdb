# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpkAo0BF/ybWd.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
