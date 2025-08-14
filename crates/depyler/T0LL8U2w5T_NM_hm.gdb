# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpFKxfIt/T0LL8U2w5T_NM_hm.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
