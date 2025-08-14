# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpVLiCRV/M_4cVA0v83ZE_9.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
