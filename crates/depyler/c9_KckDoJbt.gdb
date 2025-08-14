# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpJrO7Gt/c9_KckDoJbt.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
