# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpJlvCFN/n541o__rdi_XgiQ.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
