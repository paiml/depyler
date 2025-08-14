# GDB initialization script for Depyler debugging
# Source: /tmp/.tmptcvEIw/tFfiit_g_Y.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
