# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp7R60gD/B_O.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
