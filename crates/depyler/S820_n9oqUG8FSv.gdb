# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpNdIV8D/S820_n9oqUG8FSv.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
