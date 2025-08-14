# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpiuEpH6/x5p2P_f1_w_K4.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
