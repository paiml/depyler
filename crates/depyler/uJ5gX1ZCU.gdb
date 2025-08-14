# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpbMEk2L/uJ5gX1ZCU.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
