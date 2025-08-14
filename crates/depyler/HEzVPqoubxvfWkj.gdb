# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpW0qb3J/HEzVPqoubxvfWkj.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
