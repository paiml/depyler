# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpcDFkI9/i0x_GfO_4K3E.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
