# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpbfYTxw/Ka_14_75iK.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
