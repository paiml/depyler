# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpKc3QVs/O2_1f_4he2lAN3VI_y7.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
