# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp9UZGsL/oyUA.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
