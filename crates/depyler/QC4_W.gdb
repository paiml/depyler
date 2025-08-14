# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpqVfZvf/QC4_W.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
