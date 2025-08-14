# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp8Xicih/m___x_eVW_Y_U.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
