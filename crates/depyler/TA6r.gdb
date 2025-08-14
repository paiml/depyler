# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpFjAORQ/TA6r.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
