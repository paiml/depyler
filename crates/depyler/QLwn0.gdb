# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpIsaJpM/QLwn0.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
