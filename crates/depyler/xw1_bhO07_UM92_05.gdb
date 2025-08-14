# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpBsd2lW/xw1_bhO07_UM92_05.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
