# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpxWRrBH/Tux684ULY3_7.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
