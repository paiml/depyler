# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpkM9gGy/l1A5K.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
