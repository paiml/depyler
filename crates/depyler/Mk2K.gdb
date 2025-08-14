# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpazZb7k/Mk2K.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
