# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpDpVMte/b3_m_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
